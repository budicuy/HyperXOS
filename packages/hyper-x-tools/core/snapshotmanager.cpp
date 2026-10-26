#include "snapshotmanager.h"
#include <QFile>
#include <QTextStream>
#include <QDir>
#include <QFileInfo>
#include <QDateTime>

SnapshotManager::SnapshotManager(QObject *parent)
    : QObject(parent), m_process(new QProcess(this))
{
    connect(m_process, &QProcess::readyReadStandardOutput, this, &SnapshotManager::onProcessReadyRead);
    connect(m_process, &QProcess::readyReadStandardError, this, &SnapshotManager::onProcessReadyRead);
    connect(m_process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &SnapshotManager::onProcessFinished);

    detectFilesystem();
    refreshSnapshots();
}

SnapshotManager::~SnapshotManager()
{
    if (m_process->state() != QProcess::NotRunning) {
        m_process->kill();
        m_process->waitForFinished(1000);
    }
}

void SnapshotManager::detectFilesystem()
{
    QFile mounts("/proc/mounts");
    if (mounts.open(QIODevice::ReadOnly | QIODevice::Text)) {
        QTextStream in(&mounts);
        while (!in.atEnd()) {
            QString line = in.readLine();
            QStringList parts = line.split(' ', Qt::SkipEmptyParts);
            if (parts.size() >= 3 && parts[1] == "/") {
                if (parts[2].toLower() == "btrfs") {
                    m_isBtrfs = true;
                }
                break;
            }
        }
        mounts.close();
    }

    m_isTimeshiftInstalled = QFile::exists("/usr/bin/timeshift");
}

bool SnapshotManager::isBtrfs() const
{
    return m_isBtrfs;
}

bool SnapshotManager::isTimeshiftInstalled() const
{
    return m_isTimeshiftInstalled;
}

QList<SnapshotItem> SnapshotManager::getSnapshots() const
{
    return m_snapshots;
}

void SnapshotManager::refreshSnapshots()
{
    m_snapshots.clear();

    if (m_isBtrfs) {
        scanBtrfsSnapshots();
    } else if (m_isTimeshiftInstalled) {
        scanTimeshiftSnapshots();
    }

    emit snapshotsRefreshed(m_snapshots);
}

void SnapshotManager::scanBtrfsSnapshots()
{
    QDir snapDir("/.snapshots");
    if (!snapDir.exists()) {
        return;
    }

    QStringList entries = snapDir.entryList(QDir::Dirs | QDir::NoDotAndDotDot, QDir::Time);
    for (const QString &entry : entries) {
        SnapshotItem item;
        item.id = entry;
        item.name = entry;
        item.path = snapDir.filePath(entry);
        item.isBtrfs = true;

        QFileInfo info(item.path);
        item.createdAt = info.birthTime().isValid() ? info.birthTime() : info.lastModified();

        // Check if there is a desc file or extract from name
        QFile descFile(item.path + "/info.txt");
        if (descFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
            item.description = QTextStream(&descFile).readAll().trimmed();
            descFile.close();
        } else {
            item.description = "Btrfs Root Subvolume Snapshot";
        }
        item.sizeString = "Btrfs CoW (Instant)";

        m_snapshots.append(item);
    }
}

void SnapshotManager::scanTimeshiftSnapshots()
{
    QProcess proc;
    proc.start("timeshift", QStringList() << "--list");
    if (proc.waitForFinished(1000)) {
        QString out = proc.readAllStandardOutput();
        QStringList lines = out.split('\n');
        for (const QString &line : lines) {
            if (line.contains(">") && line.contains("-")) {
                QString clean = line.trimmed();
                QStringList parts = clean.split(' ', Qt::SkipEmptyParts);
                if (parts.size() >= 3) {
                    SnapshotItem item;
                    item.id = parts[1];
                    item.name = parts[1];
                    item.description = "Timeshift System Backup";
                    item.createdAt = QDateTime::fromString(parts[1], "yyyy-MM-dd_hh-mm-ss");
                    item.sizeString = "Incremental";
                    item.isBtrfs = false;
                    m_snapshots.append(item);
                }
            }
        }
    }
}

void SnapshotManager::createSnapshot(const QString &description)
{
    if (m_isBusy) return;

    m_isBusy = true;
    QString timeStr = QDateTime::currentDateTime().toString("yyyy-MM-dd_HH-mm-ss");
    QString desc = description.trimmed().isEmpty() ? "Snapshot Manual HyperXOS" : description.trimmed();

    emit operationStarted(QString("Membuat snapshot baru: %1...").arg(desc));
    emit logOutput(QString("==> Memulai pembuatan snapshot pemulihan sistem...\n"));

    if (m_isBtrfs) {
        QString snapName = QString("snapshot-%1").arg(timeStr);
        QString script = QString("mkdir -p /.snapshots && "
                                 "btrfs subvolume snapshot -r / /.snapshots/%1 && "
                                 "echo '%2' > /.snapshots/%1/info.txt")
                             .arg(snapName).arg(desc);

        emit logOutput(QString("  -> Mode Btrfs: Menyalin subvolume CoW ke /.snapshots/%1\n").arg(snapName));
        m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
    } else if (m_isTimeshiftInstalled) {
        emit logOutput(QString("  -> Mode Timeshift: Menjalankan timeshift --create --comments \"%1\"\n").arg(desc));
        m_process->start("pkexec", QStringList() << "timeshift" << "--create" << "--comments" << desc);
    } else {
        // Fallback rsync incremental backup
        QString snapPath = QString("/var/backups/hyperxos-snapshots/%1").arg(timeStr);
        QString script = QString("mkdir -p '%1' && "
                                 "echo '%2' > '%1/info.txt' && "
                                 "rsync -aAX --delete --exclude={'/proc/*','/sys/*','/dev/*','/run/*','/tmp/*','/var/tmp/*','/mnt/*','/media/*','/lost+found','/var/backups/*'} / '%1/'")
                             .arg(snapPath).arg(desc);

        emit logOutput("  -> Mode Standar: Menggunakan Rsync Incremental System Snapshot...\n");
        m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
    }
}

void SnapshotManager::deleteSnapshot(const QString &snapshotId)
{
    if (m_isBusy) return;

    m_isBusy = true;
    emit operationStarted(QString("Menghapus snapshot %1...").arg(snapshotId));
    emit logOutput(QString("==> Menghapus cadangan snapshot %1...\n").arg(snapshotId));

    if (m_isBtrfs) {
        QString script = QString("btrfs subvolume delete /.snapshots/%1 || rm -rf /.snapshots/%1").arg(snapshotId);
        m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
    } else if (m_isTimeshiftInstalled) {
        m_process->start("pkexec", QStringList() << "timeshift" << "--delete" << "--snapshot" << snapshotId);
    } else {
        QString script = QString("rm -rf '/var/backups/hyperxos-snapshots/%1'").arg(snapshotId);
        m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
    }
}

void SnapshotManager::restoreSnapshot(const QString &snapshotId)
{
    if (m_isBusy) return;

    m_isBusy = true;
    emit operationStarted(QString("Menyiapkan pemulihan ke snapshot %1...").arg(snapshotId));
    emit logOutput(QString("==> Memulihkan konfigurasi sistem dari snapshot %1...\n").arg(snapshotId));

    if (m_isBtrfs) {
        QString script = QString("echo 'Untuk keamanan Btrfs, silakan pilih snapshot %1 dari menu boot Limine/GRUB atau timpa subvolume @ saat mode live USB.' && "
                                 "cp -af /.snapshots/%1/etc /etc.backup-pre-restore 2>/dev/null || true").arg(snapshotId);
        m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
    } else if (m_isTimeshiftInstalled) {
        m_process->start("pkexec", QStringList() << "timeshift" << "--restore" << "--snapshot" << snapshotId);
    } else {
        emit operationFinished(false, "Restore partisi non-Btrfs disarankan melalui Live Media ISO.");
        m_isBusy = false;
    }
}

bool SnapshotManager::isBusy() const
{
    return m_isBusy;
}

void SnapshotManager::onProcessReadyRead()
{
    QByteArray out = m_process->readAllStandardOutput();
    QByteArray err = m_process->readAllStandardError();
    if (!out.isEmpty()) emit logOutput(QString::fromUtf8(out));
    if (!err.isEmpty()) emit logOutput(QString::fromUtf8(err));
}

void SnapshotManager::onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus)
{
    m_isBusy = false;
    refreshSnapshots();

    if (exitStatus == QProcess::NormalExit && exitCode == 0) {
        emit operationFinished(true, "Operasi snapshot cadangan selesai dengan sukses.");
    } else {
        emit operationFinished(false, QString("Operasi snapshot selesai dengan status %1.").arg(exitCode));
    }
}

#include "kernelmanager.h"
#include <QFile>
#include <QTextStream>
#include <QDir>
#include <sys/utsname.h>

KernelManager::KernelManager(QObject *parent)
    : QObject(parent), m_process(new QProcess(this))
{
    connect(m_process, &QProcess::readyReadStandardOutput, this, &KernelManager::onProcessReadyRead);
    connect(m_process, &QProcess::readyReadStandardError, this, &KernelManager::onProcessReadyRead);
    connect(m_process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &KernelManager::onProcessFinished);

    struct utsname uts;
    if (uname(&uts) == 0) {
        m_runningKernel = QString::fromLatin1(uts.release);
    }

    initKernelCatalog();
    refreshKernels();
}

KernelManager::~KernelManager()
{
    if (m_process->state() != QProcess::NotRunning) {
        m_process->kill();
        m_process->waitForFinished(1000);
    }
}

void KernelManager::initKernelCatalog()
{
    m_kernels.clear();

    m_kernels.append({
        "linux-cachyos",
        "Linux CachyOS (BORE)",
        "Kernel performa tinggi resmi CachyOS dengan patch BORE scheduler, LTO, dan auto-FDO. Pilihan terbaik untuk gaming.",
        "linux-cachyos",
        "linux-cachyos-headers",
        "", false, false, "CachyOS Performance"
    });

    m_kernels.append({
        "linux-cachyos-sched-ext",
        "Linux CachyOS Sched-Ext",
        "Dukungan scheduler CPU eBPF dinamis (scx_lavd, scx_rustland) untuk latensi gaming ultra-rendah.",
        "linux-cachyos-sched-ext",
        "linux-cachyos-sched-ext-headers",
        "", false, false, "CachyOS Performance"
    });

    m_kernels.append({
        "linux-cachyos-bore",
        "Linux CachyOS BORE Dedicated",
        "Dikhususkan untuk stabilitas penjadwalan desktop interaktif di bawah beban berat multitasking.",
        "linux-cachyos-bore",
        "linux-cachyos-bore-headers",
        "", false, false, "CachyOS Performance"
    });

    m_kernels.append({
        "linux-cachyos-lts",
        "Linux CachyOS LTS",
        "Long-Term Support kernel untuk kestabilan jangka panjang yang tetap membawa optimasi CachyOS.",
        "linux-cachyos-lts",
        "linux-cachyos-lts-headers",
        "", false, false, "Stabilitas (LTS)"
    });

    m_kernels.append({
        "linux-cachyos-hardened",
        "Linux CachyOS Hardened",
        "Varian keamanan tinggi dengan perlindungan eksploit memori dan mitigasi keamanan ketat.",
        "linux-cachyos-hardened",
        "linux-cachyos-hardened-headers",
        "", false, false, "Keamanan"
    });

    m_kernels.append({
        "linux-zen",
        "Linux Zen (Arch)",
        "Kernel tuning desktop & multimedia dari upstream Arch Linux.",
        "linux-zen",
        "linux-zen-headers",
        "", false, false, "Arch Upstream"
    });

    m_kernels.append({
        "linux",
        "Linux Vanilla (Arch)",
        "Kernel standar resmi Arch Linux.",
        "linux",
        "linux-headers",
        "", false, false, "Arch Upstream"
    });
}

QString KernelManager::getRunningKernel() const
{
    return m_runningKernel;
}

QList<KernelItem> KernelManager::getKernels() const
{
    return m_kernels;
}

bool KernelManager::checkPackageInstalled(const QString &pkgName)
{
    // Check in /var/lib/pacman/local
    QDir localDir("/var/lib/pacman/local");
    QStringList matches = localDir.entryList(QStringList() << QString("%1-[0-9]*").arg(pkgName), QDir::Dirs);
    return !matches.isEmpty();
}

void KernelManager::refreshKernels()
{
    for (int i = 0; i < m_kernels.size(); ++i) {
        bool installed = checkPackageInstalled(m_kernels[i].package);
        m_kernels[i].isInstalled = installed;

        // Check if currently running matches
        bool running = false;
        if (m_kernels[i].package == "linux-cachyos" && m_runningKernel.contains("cachyos") && !m_runningKernel.contains("sched-ext") && !m_runningKernel.contains("bore") && !m_runningKernel.contains("lts") && !m_runningKernel.contains("hardened")) {
            running = true;
        } else if (m_kernels[i].package == "linux-cachyos-sched-ext" && m_runningKernel.contains("sched-ext")) {
            running = true;
        } else if (m_kernels[i].package == "linux-cachyos-bore" && m_runningKernel.contains("bore")) {
            running = true;
        } else if (m_kernels[i].package == "linux-cachyos-lts" && m_runningKernel.contains("cachyos-lts")) {
            running = true;
        } else if (m_kernels[i].package == "linux-cachyos-hardened" && m_runningKernel.contains("hardened")) {
            running = true;
        } else if (m_kernels[i].package == "linux-zen" && m_runningKernel.contains("zen")) {
            running = true;
        } else if (m_kernels[i].package == "linux" && m_runningKernel.contains("arch") && !m_runningKernel.contains("zen") && !m_runningKernel.contains("cachyos")) {
            running = true;
        }
        m_kernels[i].isRunning = running;

        // Find version from pacman local
        if (installed) {
            QDir localDir("/var/lib/pacman/local");
            QStringList matches = localDir.entryList(QStringList() << QString("%1-[0-9]*").arg(m_kernels[i].package), QDir::Dirs);
            if (!matches.isEmpty()) {
                QString dirName = matches.first();
                QString ver = dirName.mid(m_kernels[i].package.length() + 1);
                m_kernels[i].version = ver;
            }
        } else {
            m_kernels[i].version = "";
        }
    }

    emit kernelsRefreshed();
}

void KernelManager::installKernel(const QString &package)
{
    if (m_isBusy) return;

    QString headersPkg = package + "-headers";
    m_isBusy = true;
    emit operationStarted(QString("Memasang kernel %1 & %2...").arg(package).arg(headersPkg));
    emit logOutput(QString("==> Memulai pemasangan kernel %1 beserta headers...\n").arg(package));

    QStringList args;
    args << "pacman" << "-S" << "--noconfirm" << "--needed" << package << headersPkg;

    m_process->start("pkexec", args);
}

void KernelManager::removeKernel(const QString &package)
{
    if (m_isBusy) return;

    // Safety check: do not allow removing currently active kernel
    for (const auto &k : m_kernels) {
        if (k.package == package && k.isRunning) {
            emit operationFinished(false, "Keamanan: Anda tidak dapat mencopot kernel yang sedang aktif berjalan!");
            return;
        }
    }

    QString headersPkg = package + "-headers";
    m_isBusy = true;
    emit operationStarted(QString("Mencopot kernel %1...").arg(package));
    emit logOutput(QString("==> Mencopot kernel %1 dan %2...\n").arg(package).arg(headersPkg));

    QStringList args;
    args << "pacman" << "-R" << "--noconfirm" << package << headersPkg;

    m_process->start("pkexec", args);
}

bool KernelManager::isBusy() const
{
    return m_isBusy;
}

void KernelManager::onProcessReadyRead()
{
    QByteArray out = m_process->readAllStandardOutput();
    QByteArray err = m_process->readAllStandardError();
    if (!out.isEmpty()) emit logOutput(QString::fromUtf8(out));
    if (!err.isEmpty()) emit logOutput(QString::fromUtf8(err));
}

void KernelManager::onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus)
{
    m_isBusy = false;
    refreshKernels();

    if (exitStatus == QProcess::NormalExit && exitCode == 0) {
        emit operationFinished(true, "Operasi kernel berhasil diselesaikan.");
    } else {
        emit operationFinished(false, QString("Operasi kernel gagal dengan kode keluar %1.").arg(exitCode));
    }
}

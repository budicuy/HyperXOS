#include "maintenancepage.h"
#include <QFrame>
#include <QScrollArea>
#include <QMessageBox>

MaintenancePage::MaintenancePage(QWidget *parent)
    : QWidget(parent), m_process(new QProcess(this))
{
    connect(m_process, &QProcess::readyReadStandardOutput, this, &MaintenancePage::onProcessReadyRead);
    connect(m_process, &QProcess::readyReadStandardError, this, &MaintenancePage::onProcessReadyRead);
    connect(m_process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &MaintenancePage::onProcessFinished);

    setupUi();
}

MaintenancePage::~MaintenancePage()
{
    if (m_process->state() != QProcess::NotRunning) {
        m_process->kill();
        m_process->waitForFinished(1000);
    }
}

void MaintenancePage::setupUi()
{
    QVBoxLayout *rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(0, 0, 0, 0);

    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setFrameShape(QFrame::NoFrame);

    QWidget *container = new QWidget(scrollArea);
    QVBoxLayout *mainLayout = new QVBoxLayout(container);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(16);

    // Title & Header
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("🧹 Pemeliharaan & Pembersihan Sistem", container);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel(
        "Bersihkan cache instalasi paket, pangkas log journal yang membengkak, hapus paket yatim, dan perbaiki keyring pacman.", container);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // 1. Pacman Cache Cleaning
    QFrame *cacheCard = new QFrame(container);
    cacheCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *cacheLayout = new QVBoxLayout(cacheCard);
    cacheLayout->setSpacing(10);

    QLabel *cacheTitle = new QLabel("📦 Pembersihan Cache Paket (Pacman)", cacheCard);
    cacheTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #f5a97f;");
    cacheLayout->addWidget(cacheTitle);

    QLabel *cacheDesc = new QLabel(
        "Pacman menyimpan salinan installer semua paket yang pernah diunduh. "
        "Membersihkan cache lama akan mengembalikan ruang disk bergiga-giga tanpa merusak aplikasi.", cacheCard);
    cacheDesc->setStyleSheet("font-size: 11px; color: #a5adcb;");
    cacheDesc->setWordWrap(true);
    cacheLayout->addWidget(cacheDesc);

    QHBoxLayout *cacheBtns = new QHBoxLayout();
    QPushButton *cleanOldBtn = new QPushButton("Hapus Versi Usang (Simpan 2 Versi Terakhir)", cacheCard);
    cleanOldBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(cleanOldBtn, &QPushButton::clicked, this, &MaintenancePage::onCleanOldCacheClicked);

    QPushButton *cleanAllBtn = new QPushButton("Kosongkan Seluruh Cache Tidak Terpasang", cacheCard);
    cleanAllBtn->setStyleSheet("background-color: #ed8796; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(cleanAllBtn, &QPushButton::clicked, this, &MaintenancePage::onCleanAllCacheClicked);

    cacheBtns->addWidget(cleanOldBtn);
    cacheBtns->addWidget(cleanAllBtn);
    cacheBtns->addStretch();
    cacheLayout->addLayout(cacheBtns);
    mainLayout->addWidget(cacheCard);

    // 2. Systemd Journal Vacuum & Orphans
    QFrame *journalCard = new QFrame(container);
    journalCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *journalLayout = new QVBoxLayout(journalCard);
    journalLayout->setSpacing(10);

    QLabel *journalTitle = new QLabel("🗄️ Log Sistem & Paket Yatim (Orphans)", journalCard);
    journalTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #8bd5ca;");
    journalLayout->addWidget(journalTitle);

    QLabel *journalDesc = new QLabel(
        "Pangkas log sistem journald agar tidak memakan ruang penyimpanan, atau bersihkan paket dependensi yang sudah tidak digunakan lagi.", journalCard);
    journalDesc->setStyleSheet("font-size: 11px; color: #a5adcb;");
    journalDesc->setWordWrap(true);
    journalLayout->addWidget(journalDesc);

    QHBoxLayout *journalBtns = new QHBoxLayout();
    QPushButton *vacuumBtn = new QPushButton("Pangkas Log Journal (Batas Maksimal 100MB)", journalCard);
    vacuumBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(vacuumBtn, &QPushButton::clicked, this, &MaintenancePage::onVacuumJournalClicked);

    QPushButton *orphanBtn = new QPushButton("Pindai & Bersihkan Paket Yatim", journalCard);
    orphanBtn->setStyleSheet("background-color: #eed49f; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(orphanBtn, &QPushButton::clicked, this, &MaintenancePage::onRemoveOrphansClicked);

    journalBtns->addWidget(vacuumBtn);
    journalBtns->addWidget(orphanBtn);
    journalBtns->addStretch();
    journalLayout->addLayout(journalBtns);
    mainLayout->addWidget(journalCard);

    // 3. Network & Keyring Repair
    QFrame *netCard = new QFrame(container);
    netCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *netLayout = new QVBoxLayout(netCard);
    netLayout->setSpacing(10);

    QLabel *netTitle = new QLabel("🔑 Perbaikan Pacman Keyring & Ranking Mirror", netCard);
    netTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #c6a0f6;");
    netLayout->addWidget(netTitle);

    QLabel *netDesc = new QLabel(
        "Jika pacman gagal mengunduh paket karena 'unknown trust' atau koneksi lambat, jalankan perbaikan keyring atau urutkan mirror tercepat.", netCard);
    netDesc->setStyleSheet("font-size: 11px; color: #a5adcb;");
    netDesc->setWordWrap(true);
    netLayout->addWidget(netDesc);

    QHBoxLayout *netBtns = new QHBoxLayout();
    QPushButton *fixKeyringBtn = new QPushButton("Perbaiki Keyring Arch & CachyOS", netCard);
    fixKeyringBtn->setStyleSheet("background-color: #8aadf4; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(fixKeyringBtn, &QPushButton::clicked, this, &MaintenancePage::onFixKeyringClicked);

    QPushButton *rankMirrorsBtn = new QPushButton("Peringkatkan Mirror Tercepat (Reflector)", netCard);
    rankMirrorsBtn->setStyleSheet("background-color: #a6da95; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 14px;");
    connect(rankMirrorsBtn, &QPushButton::clicked, this, &MaintenancePage::onRankMirrorsClicked);

    netBtns->addWidget(fixKeyringBtn);
    netBtns->addWidget(rankMirrorsBtn);
    netBtns->addStretch();
    netLayout->addLayout(netBtns);
    mainLayout->addWidget(netCard);

    // Progress Bar
    m_progressBar = new QProgressBar(container);
    m_progressBar->setFixedHeight(4);
    m_progressBar->setTextVisible(false);
    m_progressBar->setRange(0, 0);
    m_progressBar->hide();
    mainLayout->addWidget(m_progressBar);

    // Log Console
    m_logConsole = new QPlainTextEdit(container);
    m_logConsole->setReadOnly(true);
    m_logConsole->setFixedHeight(130);
    m_logConsole->setStyleSheet("background-color: #11121d; color: #cad3f5; font-family: monospace; border: 1px solid #363a4f; border-radius: 6px;");
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);

    mainLayout->addStretch();
    scrollArea->setWidget(container);
    rootLayout->addWidget(scrollArea);
}

void MaintenancePage::runCommand(const QString &title, const QString &cmd, const QStringList &args)
{
    if (m_isBusy) return;

    m_isBusy = true;
    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();

    m_logConsole->appendPlainText(QString("==> %1...\n").arg(title));
    m_process->start(cmd, args);
}

void MaintenancePage::onCleanOldCacheClicked()
{
    runCommand("Membersihkan cache pacman (menyimpan 2 versi terakhir)", "pkexec",
               QStringList() << "paccache" << "-rk2");
}

void MaintenancePage::onCleanAllCacheClicked()
{
    auto reply = QMessageBox::question(
        this, "Konfirmasi Pengosongan Cache",
        "Apakah Anda yakin ingin menghapus seluruh paket cache yang tidak lagi terpasang?",
        QMessageBox::Yes | QMessageBox::No
    );
    if (reply == QMessageBox::Yes) {
        runCommand("Mengosongkan seluruh cache paket uninstalled", "pkexec",
                   QStringList() << "paccache" << "-ruk0");
    }
}

void MaintenancePage::onVacuumJournalClicked()
{
    runCommand("Memangkas log journald ke maksimal 100MB", "pkexec",
               QStringList() << "journalctl" << "--vacuum-size=100M");
}

void MaintenancePage::onRemoveOrphansClicked()
{
    runCommand("Mencari dan menghapus paket yatim (orphans)", "pkexec",
               QStringList() << "bash" << "-c" << "orphans=$(pacman -Qtdq); if [ -n \"$orphans\" ]; then pacman -Rns --noconfirm $orphans; else echo 'Tidak ada paket yatim yang ditemukan. Sistem bersih!'; fi");
}

void MaintenancePage::onFixKeyringClicked()
{
    runCommand("Menginisialisasi dan menyinkronkan ulang keyring", "pkexec",
               QStringList() << "bash" << "-c" << "rm -f /etc/pacman.d/gnupg/S.gpg-agent* 2>/dev/null || true; pacman-key --init && pacman-key --populate archlinux cachyos hyperxos");
}

void MaintenancePage::onRankMirrorsClicked()
{
    runCommand("Menjalankan pemeringkatan mirror tercepat dengan Reflector", "pkexec",
               QStringList() << "reflector" << "--latest" << "10" << "--protocol" << "https" << "--sort" << "rate" << "--download-timeout" << "5" << "--save" << "/etc/pacman.d/mirrorlist");
}

void MaintenancePage::onProcessReadyRead()
{
    QByteArray out = m_process->readAllStandardOutput();
    QByteArray err = m_process->readAllStandardError();
    if (!out.isEmpty()) m_logConsole->appendPlainText(QString::fromUtf8(out));
    if (!err.isEmpty()) m_logConsole->appendPlainText(QString::fromUtf8(err));
}

void MaintenancePage::onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus)
{
    m_isBusy = false;
    m_progressBar->hide();

    if (exitStatus == QProcess::NormalExit && exitCode == 0) {
        m_logConsole->appendPlainText("\n==> [✓] Tugas pemeliharaan selesai dengan sukses.\n");
        QMessageBox::information(this, "Selesai", "Tugas pemeliharaan sistem berhasil diselesaikan.");
    } else {
        m_logConsole->appendPlainText(QString("\n==> [!] Tugas selesai dengan kode keluar %1.\n").arg(exitCode));
    }
}

#include "kernelmanagerpage.h"
#include <QFrame>
#include <QMessageBox>

KernelManagerPage::KernelManagerPage(KernelManager *manager, QWidget *parent)
    : QWidget(parent), m_manager(manager)
{
    setupUi();

    if (m_manager) {
        connect(m_manager, &KernelManager::kernelsRefreshed, this, &KernelManagerPage::onKernelsRefreshed);
        connect(m_manager, &KernelManager::operationStarted, this, &KernelManagerPage::onOperationStarted);
        connect(m_manager, &KernelManager::operationFinished, this, &KernelManagerPage::onOperationFinished);
        connect(m_manager, &KernelManager::logOutput, this, &KernelManagerPage::onLogReceived);
        renderKernelList();
    }
}

KernelManagerPage::~KernelManagerPage()
{
}

void KernelManagerPage::setupUi()
{
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(16);

    // Title & Subtitle
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("🐧 Kernel Manager (CachyOS & Arch)", this);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel(
        "Pasang dan kelola kernel Linux performa tinggi dari repositori CachyOS (BORE, Sched-Ext, LTS) "
        "beserta modul headers & DKMS otomatis.", this);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // Active Kernel Banner
    QFrame *activeBanner = new QFrame(this);
    activeBanner->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 8px; padding: 12px;");
    QHBoxLayout *bannerLayout = new QHBoxLayout(activeBanner);
    bannerLayout->setSpacing(12);

    QLabel *badgeIcon = new QLabel("🚀", activeBanner);
    badgeIcon->setStyleSheet("font-size: 20px;");

    QVBoxLayout *activeTextLayout = new QVBoxLayout();
    QLabel *activeTitle = new QLabel("Kernel yang Sedang Aktif Berjalan (Booted):", activeBanner);
    activeTitle->setStyleSheet("font-size: 11px; color: #a5adcb; font-weight: bold;");

    m_activeKernelLabel = new QLabel(m_manager ? m_manager->getRunningKernel() : "Unknown", activeBanner);
    m_activeKernelLabel->setStyleSheet("font-size: 15px; font-weight: bold; color: #a6da95;");

    activeTextLayout->addWidget(activeTitle);
    activeTextLayout->addWidget(m_activeKernelLabel);

    bannerLayout->addWidget(badgeIcon);
    bannerLayout->addLayout(activeTextLayout);
    bannerLayout->addStretch();

    QPushButton *refreshBtn = new QPushButton("⟳ Segarkan", activeBanner);
    refreshBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; border-radius: 6px; padding: 6px 12px; font-weight: bold;");
    connect(refreshBtn, &QPushButton::clicked, this, [this]() {
        if (m_manager) m_manager->refreshKernels();
    });
    bannerLayout->addWidget(refreshBtn);

    QPushButton *mkinitcpioBtn = new QPushButton("🛠️ Perbarui Bootloader (mkinitcpio)", activeBanner);
    mkinitcpioBtn->setStyleSheet("background-color: #f5a97f; color: #181926; border-radius: 6px; padding: 6px 12px; font-weight: bold;");
    connect(mkinitcpioBtn, &QPushButton::clicked, this, &KernelManagerPage::onRebuildInitramfsClicked);
    bannerLayout->addWidget(mkinitcpioBtn);

    mainLayout->addWidget(activeBanner);

    // Scroll Area for Kernel Cards
    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setFrameShape(QFrame::NoFrame);

    m_kernelCardsContainer = new QWidget(scrollArea);
    m_kernelCardsLayout = new QVBoxLayout(m_kernelCardsContainer);
    m_kernelCardsLayout->setContentsMargins(0, 0, 0, 0);
    m_kernelCardsLayout->setSpacing(10);
    m_kernelCardsLayout->addStretch();

    scrollArea->setWidget(m_kernelCardsContainer);
    mainLayout->addWidget(scrollArea, 1);

    // Progress Bar
    m_progressBar = new QProgressBar(this);
    m_progressBar->setFixedHeight(4);
    m_progressBar->setTextVisible(false);
    m_progressBar->setRange(0, 0);
    m_progressBar->hide();
    mainLayout->addWidget(m_progressBar);

    // Log Console
    m_logConsole = new QPlainTextEdit(this);
    m_logConsole->setReadOnly(true);
    m_logConsole->setFixedHeight(120);
    m_logConsole->setStyleSheet("background-color: #11121d; color: #cad3f5; font-family: monospace; border: 1px solid #363a4f; border-radius: 6px;");
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);
}

void KernelManagerPage::renderKernelList()
{
    if (!m_manager) return;

    // Clear existing cards
    QLayoutItem *child;
    while ((child = m_kernelCardsLayout->takeAt(0)) != nullptr) {
        if (child->widget()) delete child->widget();
        delete child;
    }

    QList<KernelItem> kernels = m_manager->getKernels();
    for (const KernelItem &k : kernels) {
        QFrame *card = new QFrame(m_kernelCardsContainer);
        card->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 8px; padding: 12px;");

        QHBoxLayout *cardLayout = new QHBoxLayout(card);
        cardLayout->setSpacing(14);

        QVBoxLayout *infoLayout = new QVBoxLayout();
        infoLayout->setSpacing(4);

        QHBoxLayout *headerRow = new QHBoxLayout();
        QLabel *nameLabel = new QLabel(k.name, card);
        nameLabel->setStyleSheet("font-size: 15px; font-weight: bold; color: #8aadf4;");
        headerRow->addWidget(nameLabel);

        // Badges
        if (k.isRunning) {
            QLabel *runningBadge = new QLabel("● SEDANG BERJALAN", card);
            runningBadge->setStyleSheet("background-color: #a6da95; color: #181926; font-size: 10px; font-weight: bold; padding: 2px 8px; border-radius: 4px;");
            headerRow->addWidget(runningBadge);
        } else if (k.isInstalled) {
            QLabel *installedBadge = new QLabel("TERPASANG", card);
            installedBadge->setStyleSheet("background-color: #363a4f; color: #8bd5ca; font-size: 10px; font-weight: bold; padding: 2px 8px; border-radius: 4px;");
            headerRow->addWidget(installedBadge);
        }

        QLabel *catBadge = new QLabel(k.category, card);
        catBadge->setStyleSheet("background-color: #1e2030; color: #eed49f; font-size: 10px; padding: 2px 8px; border-radius: 4px; border: 1px solid #363a4f;");
        headerRow->addWidget(catBadge);
        headerRow->addStretch();

        infoLayout->addLayout(headerRow);

        QLabel *descLabel = new QLabel(k.description, card);
        descLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
        descLabel->setWordWrap(true);
        infoLayout->addWidget(descLabel);

        QLabel *pkgLabel = new QLabel(QString("Paket: %1 & %2 %3")
                                      .arg(k.package).arg(k.headersPackage)
                                      .arg(k.version.isEmpty() ? "" : QString("(v%1)").arg(k.version)), card);
        pkgLabel->setStyleSheet("font-size: 10px; color: #6e738d;");
        infoLayout->addWidget(pkgLabel);

        cardLayout->addLayout(infoLayout, 1);

        // Buttons
        QVBoxLayout *btnLayout = new QVBoxLayout();
        btnLayout->setSpacing(6);
        btnLayout->setAlignment(Qt::AlignCenter);

        if (k.isRunning) {
            QPushButton *activeBtn = new QPushButton("Aktif Digunakan", card);
            activeBtn->setEnabled(false);
            activeBtn->setStyleSheet("background-color: #1e2030; color: #a6da95; border: 1px solid #a6da95; border-radius: 6px; padding: 6px 14px; font-weight: bold;");
            btnLayout->addWidget(activeBtn);
        } else if (k.isInstalled) {
            QPushButton *removeBtn = new QPushButton("Copot Kernel", card);
            removeBtn->setStyleSheet("background-color: #ed8796; color: #181926; border-radius: 6px; padding: 6px 14px; font-weight: bold;");
            connect(removeBtn, &QPushButton::clicked, this, [this, k]() {
                onRemoveClicked(k.package);
            });
            btnLayout->addWidget(removeBtn);
        } else {
            QPushButton *installBtn = new QPushButton("Pasang Kernel", card);
            installBtn->setStyleSheet("background-color: #a6da95; color: #181926; border-radius: 6px; padding: 6px 14px; font-weight: bold;");
            connect(installBtn, &QPushButton::clicked, this, [this, k]() {
                onInstallClicked(k.package);
            });
            btnLayout->addWidget(installBtn);
        }

        cardLayout->addLayout(btnLayout);
        m_kernelCardsLayout->addWidget(card);
    }

    m_kernelCardsLayout->addStretch();
}

void KernelManagerPage::onKernelsRefreshed()
{
    if (m_manager) {
        m_activeKernelLabel->setText(m_manager->getRunningKernel());
    }
    renderKernelList();
}

void KernelManagerPage::onInstallClicked(const QString &package)
{
    if (!m_manager || m_manager->isBusy()) return;

    auto reply = QMessageBox::question(
        this, "Konfirmasi Pemasangan Kernel",
        QString("Apakah Anda yakin ingin memasang kernel '%1' beserta paket headers dan modulnya?").arg(package),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_logConsole->clear();
        m_logConsole->show();
        m_progressBar->show();
        m_manager->installKernel(package);
    }
}

void KernelManagerPage::onRemoveClicked(const QString &package)
{
    if (!m_manager || m_manager->isBusy()) return;

    auto reply = QMessageBox::warning(
        this, "Peringatan Pencopotan Kernel",
        QString("Apakah Anda yakin ingin mencopot kernel '%1'? Pastikan masih ada kernel lain yang terpasang untuk boot sistem.").arg(package),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_logConsole->clear();
        m_logConsole->show();
        m_progressBar->show();
        m_manager->removeKernel(package);
    }
}

void KernelManagerPage::onRebuildInitramfsClicked()
{
    if (!m_manager || m_manager->isBusy()) return;

    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();

    onLogReceived("==> Menjalankan regenerasi initramfs untuk semua kernel (mkinitcpio -P)...\n");
    QProcess *proc = new QProcess(this);
    connect(proc, &QProcess::readyReadStandardOutput, this, [this, proc]() {
        onLogReceived(QString::fromUtf8(proc->readAllStandardOutput()));
    });
    connect(proc, &QProcess::readyReadStandardError, this, [this, proc]() {
        onLogReceived(QString::fromUtf8(proc->readAllStandardError()));
    });
    connect(proc, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished), this, [this, proc](int exitCode) {
        m_progressBar->hide();
        if (exitCode == 0) {
            onLogReceived("==> [✓] Pembaruan initramfs berhasil!\n");
            QMessageBox::information(this, "Sukses", "Pembaruan initramfs dan bootloader berhasil.");
        } else {
            onLogReceived(QString("==> [!] Pembaruan gagal dengan kode %1\n").arg(exitCode));
            QMessageBox::critical(this, "Gagal", "Gagal memperbarui initramfs.");
        }
        proc->deleteLater();
    });

    proc->start("pkexec", QStringList() << "mkinitcpio" << "-P");
}

void KernelManagerPage::onOperationStarted(const QString &action)
{
    Q_UNUSED(action);
    m_progressBar->show();
    m_logConsole->show();
}

void KernelManagerPage::onOperationFinished(bool success, const QString &message)
{
    m_progressBar->hide();
    if (success) {
        QMessageBox::information(this, "Operasi Berhasil", message);
    } else {
        QMessageBox::critical(this, "Operasi Gagal", message);
    }
}

void KernelManagerPage::onLogReceived(const QString &text)
{
    m_logConsole->appendPlainText(text);
}

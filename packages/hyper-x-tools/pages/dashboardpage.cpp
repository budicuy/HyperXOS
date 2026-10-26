#include "dashboardpage.h"
#include <QScrollArea>
#include <QProcess>

DashboardPage::DashboardPage(SystemMonitor *monitor, QWidget *parent)
    : QWidget(parent), m_monitor(monitor)
{
    setupUi();

    if (m_monitor) {
        connect(m_monitor, &SystemMonitor::metricsUpdated, this, &DashboardPage::onMetricsUpdated);
        onMetricsUpdated(m_monitor->getLatestMetrics());
    }
}

DashboardPage::~DashboardPage()
{
}

void DashboardPage::setupUi()
{
    QVBoxLayout *rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(0, 0, 0, 0);

    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setFrameShape(QFrame::NoFrame);

    QWidget *container = new QWidget(scrollArea);
    QVBoxLayout *mainLayout = new QVBoxLayout(container);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(20);

    // Page Title & Header
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("📊 Dashboard & Monitoring Sistem", container);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel("Status penggunaan sumber daya komputasi HyperXOS secara langsung (real-time).", container);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // 2x2 Metric Cards Grid
    QGridLayout *cardsGrid = new QGridLayout();
    cardsGrid->setSpacing(16);

    QFrame *cpuCard = createMetricCard("Prosesor (CPU)", "⚡", &m_cpuValueLabel, &m_cpuBar, &m_cpuSubLabel, &m_cpuExtraLabel);
    QFrame *ramCard = createMetricCard("Memori (RAM & Swap)", "🧠", &m_ramValueLabel, &m_ramBar, &m_ramSubLabel, &m_ramExtraLabel);
    QFrame *diskCard = createMetricCard("Penyimpanan Disk (Root /)", "💾", &m_diskValueLabel, &m_diskBar, &m_diskSubLabel, &m_diskExtraLabel);
    QFrame *gpuCard = createMetricCard("Akselerasi Grafis (GPU)", "🎮", &m_gpuValueLabel, &m_gpuBar, &m_gpuSubLabel, &m_gpuExtraLabel);

    cardsGrid->addWidget(cpuCard, 0, 0);
    cardsGrid->addWidget(ramCard, 0, 1);
    cardsGrid->addWidget(diskCard, 1, 0);
    cardsGrid->addWidget(gpuCard, 1, 1);

    mainLayout->addLayout(cardsGrid);

    // System Information Panel
    QFrame *infoFrame = new QFrame(container);
    infoFrame->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *infoLayout = new QVBoxLayout(infoFrame);
    infoLayout->setSpacing(12);

    QLabel *infoTitle = new QLabel("ℹ️ Informasi Sistem Operasi & Sesi", infoFrame);
    infoTitle->setStyleSheet("font-size: 15px; font-weight: bold; color: #f5a97f;");
    infoLayout->addWidget(infoTitle);

    QGridLayout *infoGrid = new QGridLayout();
    infoGrid->setSpacing(10);

    QLabel *osHeader = new QLabel("Distribusi OS:", infoFrame);
    osHeader->setStyleSheet("color: #a5adcb; font-weight: bold;");
    m_osLabel = new QLabel("HyperXOS Linux 2026.10", infoFrame);
    m_osLabel->setStyleSheet("color: #cad3f5;");

    QLabel *kernelHeader = new QLabel("Versi Kernel:", infoFrame);
    kernelHeader->setStyleSheet("color: #a5adcb; font-weight: bold;");
    m_kernelLabel = new QLabel("Memuat...", infoFrame);
    m_kernelLabel->setStyleSheet("color: #a6da95; font-weight: bold;");

    QLabel *uptimeHeader = new QLabel("Waktu Aktif (Uptime):", infoFrame);
    uptimeHeader->setStyleSheet("color: #a5adcb; font-weight: bold;");
    m_uptimeLabel = new QLabel("Memuat...", infoFrame);
    m_uptimeLabel->setStyleSheet("color: #cad3f5;");

    QLabel *sessionHeader = new QLabel("Protokol Sesi Display:", infoFrame);
    sessionHeader->setStyleSheet("color: #a5adcb; font-weight: bold;");
    m_sessionLabel = new QLabel("Wayland (Pure)", infoFrame);
    m_sessionLabel->setStyleSheet("color: #8aadf4; font-weight: bold;");

    infoGrid->addWidget(osHeader, 0, 0);
    infoGrid->addWidget(m_osLabel, 0, 1);
    infoGrid->addWidget(kernelHeader, 0, 2);
    infoGrid->addWidget(m_kernelLabel, 0, 3);
    infoGrid->addWidget(uptimeHeader, 1, 0);
    infoGrid->addWidget(m_uptimeLabel, 1, 1);
    infoGrid->addWidget(sessionHeader, 1, 2);
    infoGrid->addWidget(m_sessionLabel, 1, 3);

    infoLayout->addLayout(infoGrid);
    mainLayout->addWidget(infoFrame);

    // Quick Action Bar
    QHBoxLayout *quickActions = new QHBoxLayout();
    quickActions->setSpacing(12);

    QPushButton *htopBtn = new QPushButton("🖥️ Buka Terminal Monitor (Htop)", container);
    htopBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; border-radius: 6px; padding: 8px 14px; font-weight: bold;");
    connect(htopBtn, &QPushButton::clicked, this, []() {
        QProcess::startDetached("konsole", QStringList() << "-e" << "htop");
    });

    QPushButton *kernelBtn = new QPushButton("🐧 Kelola Kernel CachyOS ➔", container);
    kernelBtn->setStyleSheet("background-color: #f5a97f; color: #181926; border-radius: 6px; padding: 8px 14px; font-weight: bold;");
    connect(kernelBtn, &QPushButton::clicked, this, [this]() {
        emit navigateToPage(2); // Index of Kernel Manager
    });

    QPushButton *gamingBtn = new QPushButton("🎮 Tweak Gaming & Governor ➔", container);
    gamingBtn->setStyleSheet("background-color: #8aadf4; color: #181926; border-radius: 6px; padding: 8px 14px; font-weight: bold;");
    connect(gamingBtn, &QPushButton::clicked, this, [this]() {
        emit navigateToPage(1); // Index of Gaming Tweaks
    });

    quickActions->addWidget(htopBtn);
    quickActions->addStretch();
    quickActions->addWidget(kernelBtn);
    quickActions->addWidget(gamingBtn);

    mainLayout->addLayout(quickActions);
    mainLayout->addStretch();

    scrollArea->setWidget(container);
    rootLayout->addWidget(scrollArea);
}

QFrame* DashboardPage::createMetricCard(const QString &title, const QString &iconText,
                                       QLabel **valueLabel, QProgressBar **bar,
                                       QLabel **subTextLabel, QLabel **extraLabel)
{
    QFrame *card = new QFrame(this);
    card->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");

    QVBoxLayout *layout = new QVBoxLayout(card);
    layout->setSpacing(8);

    QHBoxLayout *header = new QHBoxLayout();
    QLabel *icon = new QLabel(iconText, card);
    icon->setStyleSheet("font-size: 18px;");

    QLabel *cardTitle = new QLabel(title, card);
    cardTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #cad3f5;");

    header->addWidget(icon);
    header->addWidget(cardTitle);
    header->addStretch();

    *valueLabel = new QLabel("0%", card);
    (*valueLabel)->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");
    header->addWidget(*valueLabel);

    layout->addLayout(header);

    *bar = new QProgressBar(card);
    (*bar)->setRange(0, 100);
    (*bar)->setValue(0);
    (*bar)->setTextVisible(false);
    (*bar)->setFixedHeight(8);
    (*bar)->setStyleSheet(
        "QProgressBar { background-color: #1e2030; border-radius: 4px; border: none; }"
        "QProgressBar::chunk { background-color: qlineargradient(x1:0, y1:0, x2:1, y2:0, stop:0 #8aadf4, stop:1 #c6a0f6); border-radius: 4px; }"
    );
    layout->addWidget(*bar);

    QHBoxLayout *footer = new QHBoxLayout();
    *subTextLabel = new QLabel("Memuat...", card);
    (*subTextLabel)->setStyleSheet("font-size: 11px; color: #a5adcb;");

    *extraLabel = new QLabel("", card);
    (*extraLabel)->setStyleSheet("font-size: 11px; color: #8bd5ca; font-weight: bold;");

    footer->addWidget(*subTextLabel);
    footer->addStretch();
    footer->addWidget(*extraLabel);

    layout->addLayout(footer);
    return card;
}

void DashboardPage::onMetricsUpdated(const SystemMetrics &metrics)
{
    // CPU Update
    m_cpuValueLabel->setText(QString("%1%").arg(QString::number(metrics.cpuUsagePercent, 'f', 1)));
    m_cpuBar->setValue(static_cast<int>(metrics.cpuUsagePercent));
    m_cpuSubLabel->setText(QString("%1 Core / Thread (%2)").arg(metrics.cpuCores).arg(metrics.cpuModel.left(30)));
    if (metrics.cpuTempCelsius > 0) {
        m_cpuExtraLabel->setText(QString("Suhu: %1°C").arg(metrics.cpuTempCelsius));
    } else {
        m_cpuExtraLabel->setText("");
    }

    // RAM Update
    double ramUsedGb = (double)metrics.ramUsedBytes / (1024.0 * 1024.0 * 1024.0);
    double ramTotalGb = (double)metrics.ramTotalBytes / (1024.0 * 1024.0 * 1024.0);
    m_ramValueLabel->setText(QString("%1%").arg(QString::number(metrics.ramUsagePercent, 'f', 1)));
    m_ramBar->setValue(static_cast<int>(metrics.ramUsagePercent));
    m_ramSubLabel->setText(QString("Terpakai: %1 GB / %2 GB").arg(QString::number(ramUsedGb, 'f', 1)).arg(QString::number(ramTotalGb, 'f', 1)));
    if (metrics.swapTotalBytes > 0) {
        double swapUsedGb = (double)metrics.swapUsedBytes / (1024.0 * 1024.0 * 1024.0);
        m_ramExtraLabel->setText(QString("Swap: %1 GB").arg(QString::number(swapUsedGb, 'f', 1)));
    } else {
        m_ramExtraLabel->setText("");
    }

    // Disk Update
    double diskUsedGb = (double)metrics.diskUsedBytes / (1024.0 * 1024.0 * 1024.0);
    double diskTotalGb = (double)metrics.diskTotalBytes / (1024.0 * 1024.0 * 1024.0);
    m_diskValueLabel->setText(QString("%1%").arg(QString::number(metrics.diskUsagePercent, 'f', 1)));
    m_diskBar->setValue(static_cast<int>(metrics.diskUsagePercent));
    m_diskSubLabel->setText(QString("Terpakai: %1 GB / %2 GB").arg(QString::number(diskUsedGb, 'f', 1)).arg(QString::number(diskTotalGb, 'f', 1)));
    m_diskExtraLabel->setText(QString("FS: %1").arg(metrics.diskFsType));

    // GPU Update
    if (metrics.gpuUsagePercent >= 0.0) {
        m_gpuValueLabel->setText(QString("%1%").arg(QString::number(metrics.gpuUsagePercent, 'f', 1)));
        m_gpuBar->setValue(static_cast<int>(metrics.gpuUsagePercent));
    } else {
        m_gpuValueLabel->setText("Aktif");
        m_gpuBar->setValue(10);
    }
    m_gpuSubLabel->setText(metrics.gpuModel.left(35));
    if (metrics.vramTotalBytes > 0) {
        double vramUsedMb = (double)metrics.vramUsedBytes / (1024.0 * 1024.0);
        double vramTotalMb = (double)metrics.vramTotalBytes / (1024.0 * 1024.0);
        m_gpuExtraLabel->setText(QString("VRAM: %1 / %2 MB").arg(QString::number(vramUsedMb, 'f', 0)).arg(QString::number(vramTotalMb, 'f', 0)));
    } else {
        m_gpuExtraLabel->setText("");
    }

    // System Info
    m_osLabel->setText(metrics.osName);
    m_kernelLabel->setText(metrics.kernelVersion);
    m_uptimeLabel->setText(metrics.uptimeString);
    m_sessionLabel->setText(metrics.displaySession);
}

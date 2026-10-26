#include "mainwindow.h"
#include <QIcon>
#include <QPixmap>
#include <QSplitter>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent),
      m_monitor(new SystemMonitor(this)),
      m_kernelManager(new KernelManager(this)),
      m_tweaker(new SystemTweaker(this)),
      m_snapshotManager(new SnapshotManager(this)),
      m_packageManager(new PackageManager(this))
{
    setupUi();
    applyStyleSheet();
    setupSidebar();

    connect(m_monitor, &SystemMonitor::metricsUpdated, this, &MainWindow::onMetricsUpdated);
    m_monitor->startMonitoring(1500);

    // Default to Dashboard
    navigateToPage(0);
}

MainWindow::~MainWindow()
{
}

void MainWindow::setupUi()
{
    setWindowTitle("HyperXTools — Pusat Kendali & Dashboard HyperXOS");
    setWindowIcon(QIcon(":/logo.png"));
    resize(1120, 740);
    setMinimumSize(960, 600);

    QWidget *centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);

    QVBoxLayout *rootLayout = new QVBoxLayout(centralWidget);
    rootLayout->setContentsMargins(14, 12, 14, 12);
    rootLayout->setSpacing(10);

    // ==========================================
    // 1. Header Bar
    // ==========================================
    QFrame *headerFrame = new QFrame(this);
    headerFrame->setObjectName("headerFrame");
    QHBoxLayout *headerLayout = new QHBoxLayout(headerFrame);
    headerLayout->setContentsMargins(14, 10, 14, 10);
    headerLayout->setSpacing(14);

    m_logoLabel = new QLabel(this);
    QPixmap logoPix(":/logo.png");
    if (!logoPix.isNull()) {
        m_logoLabel->setPixmap(logoPix.scaled(42, 42, Qt::KeepAspectRatio, Qt::SmoothTransformation));
    }
    headerLayout->addWidget(m_logoLabel);

    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(2);

    m_titleLabel = new QLabel("HyperXTools", this);
    m_titleLabel->setObjectName("headerTitle");

    QLabel *subtitleLabel = new QLabel("Pusat Kendali, Optimasi Performa, Kernel & Snapshot HyperXOS", this);
    subtitleLabel->setObjectName("headerSubtitle");

    titleLayout->addWidget(m_titleLabel);
    titleLayout->addWidget(subtitleLabel);
    headerLayout->addLayout(titleLayout);

    headerLayout->addStretch();

    m_quickStatsLabel = new QLabel("Memuat metrik...", this);
    m_quickStatsLabel->setObjectName("headerStats");
    headerLayout->addWidget(m_quickStatsLabel);

    rootLayout->addWidget(headerFrame);

    // ==========================================
    // 2. Middle Content: Sidebar + Stacked Pages
    // ==========================================
    QHBoxLayout *contentLayout = new QHBoxLayout();
    contentLayout->setSpacing(12);

    // Grouped Sidebar List
    m_sidebar = new QListWidget(this);
    m_sidebar->setObjectName("sidebarNav");
    m_sidebar->setFixedWidth(240);
    connect(m_sidebar, &QListWidget::itemClicked, this, &MainWindow::onSidebarItemClicked);
    contentLayout->addWidget(m_sidebar);

    // QStackedWidget for Multi-Page Management
    m_stackedWidget = new QStackedWidget(this);
    m_stackedWidget->setObjectName("stackedWidget");

    // Instantiating Pages
    m_dashboardPage = new DashboardPage(m_monitor, this);
    connect(m_dashboardPage, &DashboardPage::navigateToPage, this, &MainWindow::navigateToPage);
    m_stackedWidget->addWidget(m_dashboardPage); // Index 0

    m_gamingPage = new GamingTweaksPage(m_tweaker, this);
    m_stackedWidget->addWidget(m_gamingPage);    // Index 1

    m_kernelPage = new KernelManagerPage(m_kernelManager, this);
    m_stackedWidget->addWidget(m_kernelPage);    // Index 2

    m_snapshotPage = new SnapshotPage(m_snapshotManager, this);
    m_stackedWidget->addWidget(m_snapshotPage);  // Index 3

    m_maintenancePage = new MaintenancePage(this);
    m_stackedWidget->addWidget(m_maintenancePage); // Index 4

    m_storePage = new PackageStorePage(m_packageManager, this);
    m_stackedWidget->addWidget(m_storePage);     // Index 5

    contentLayout->addWidget(m_stackedWidget, 1);
    rootLayout->addLayout(contentLayout, 1);
}

void MainWindow::setupSidebar()
{
    m_sidebar->clear();

    auto addSectionHeader = [this](const QString &title) {
        QListWidgetItem *header = new QListWidgetItem(title, m_sidebar);
        header->setFlags(Qt::NoItemFlags);
        header->setData(Qt::UserRole, -1);
    };

    auto addNavItem = [this](const QString &title, int pageIndex) {
        QListWidgetItem *item = new QListWidgetItem(title, m_sidebar);
        item->setData(Qt::UserRole, pageIndex);
    };

    // Group 1: Sistem
    addSectionHeader("── SISTEM ──");
    addNavItem("  📊  Dashboard & Monitor", 0);

    // Group 2: Performa & Kernel
    addSectionHeader("── PERFORMA & KERNEL ──");
    addNavItem("  🎮  Tweak Gaming", 1);
    addNavItem("  🐧  Kernel Manager CachyOS", 2);

    // Group 3: Cadangan & Keamanan
    addSectionHeader("── CADANGAN & PERAWATAN ──");
    addNavItem("  💾  Snapshot & Backup", 3);
    addNavItem("  🧹  Pemeliharaan Sistem", 4);

    // Group 4: Perangkat Lunak
    addSectionHeader("── PERANGKAT LUNAK ──");
    addNavItem("  📦  App Store & Paket", 5);
}

void MainWindow::onSidebarItemClicked(QListWidgetItem *item)
{
    if (!item) return;
    int pageIndex = item->data(Qt::UserRole).toInt();
    if (pageIndex >= 0) {
        m_stackedWidget->setCurrentIndex(pageIndex);
    }
}

void MainWindow::navigateToPage(int index)
{
    if (index >= 0 && index < m_stackedWidget->count()) {
        m_stackedWidget->setCurrentIndex(index);
        for (int i = 0; i < m_sidebar->count(); ++i) {
            QListWidgetItem *it = m_sidebar->item(i);
            if (it->data(Qt::UserRole).toInt() == index) {
                m_sidebar->setCurrentItem(it);
                break;
            }
        }
    }
}

void MainWindow::onMetricsUpdated(const SystemMetrics &metrics)
{
    QString info = QString("Kernel: %1  |  CPU: %2%  |  RAM: %3%")
                       .arg(metrics.kernelVersion)
                       .arg(QString::number(metrics.cpuUsagePercent, 'f', 0))
                       .arg(QString::number(metrics.ramUsagePercent, 'f', 0));
    m_quickStatsLabel->setText(info);
}

void MainWindow::applyStyleSheet()
{
    QString qss = R"(
        QMainWindow {
            background-color: #181926;
            color: #cad3f5;
            font-family: 'Segoe UI', 'Ubuntu', 'Noto Sans', sans-serif;
        }
        #headerFrame {
            background-color: #24273a;
            border: 1px solid #363a4f;
            border-radius: 8px;
        }
        #headerTitle {
            font-size: 18px;
            font-weight: bold;
            color: #8aadf4;
        }
        #headerSubtitle {
            font-size: 11px;
            color: #a5adcb;
        }
        #headerStats {
            font-size: 11px;
            color: #8bd5ca;
            background-color: #1e2030;
            padding: 5px 12px;
            border-radius: 6px;
            border: 1px solid #363a4f;
            font-weight: bold;
        }
        #sidebarNav {
            background-color: #24273a;
            border: 1px solid #363a4f;
            border-radius: 8px;
            padding: 8px;
            outline: none;
        }
        #sidebarNav::item {
            color: #cad3f5;
            padding: 10px 12px;
            border-radius: 6px;
            margin-bottom: 3px;
            font-size: 13px;
        }
        #sidebarNav::item:hover {
            background-color: #363a4f;
            color: #ffffff;
        }
        #sidebarNav::item:selected {
            background-color: #8aadf4;
            color: #181926;
            font-weight: bold;
        }
        #stackedWidget {
            background-color: transparent;
        }
        QScrollBar:vertical {
            background: #181926;
            width: 8px;
            margin: 0;
            border-radius: 4px;
        }
        QScrollBar::handle:vertical {
            background: #363a4f;
            min-height: 20px;
            border-radius: 4px;
        }
        QScrollBar::handle:vertical:hover {
            background: #5b6078;
        }
        QScrollBar::add-line:vertical, QScrollBar::sub-line:vertical {
            height: 0;
        }
    )";
    setStyleSheet(qss);
}

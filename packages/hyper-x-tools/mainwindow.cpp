#include "mainwindow.h"
#include <QApplication>
#include <QFrame>
#include <QSplitter>
#include <QMessageBox>
#include <QIcon>
#include <QPixmap>
#include <QScrollBar>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), m_packageManager(new PackageManager(this))
{
    setupUi();
    applyStyleSheet();

    connect(m_packageManager, &PackageManager::logReceived, this, &MainWindow::onLogReceived);
    connect(m_packageManager, &PackageManager::processStarted, this, &MainWindow::onProcessStarted);
    connect(m_packageManager, &PackageManager::processFinished, this, &MainWindow::onProcessFinished);
    connect(m_packageManager, &PackageManager::cacheRefreshed, this, &MainWindow::onCacheRefreshed);

    populateCategories();
    if (m_categoryList->count() > 0) {
        m_categoryList->setCurrentRow(0);
    }
}

MainWindow::~MainWindow()
{
}

void MainWindow::setupUi()
{
    setWindowTitle("HyperXTools — Package & Application Manager");
    setWindowIcon(QIcon(":/logo.png"));
    resize(980, 680);
    setMinimumSize(850, 550);

    QWidget *centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);

    QVBoxLayout *mainLayout = new QVBoxLayout(centralWidget);
    mainLayout->setContentsMargins(16, 14, 16, 14);
    mainLayout->setSpacing(12);

    // ==========================================
    // 1. Header Bar
    // ==========================================
    QFrame *headerFrame = new QFrame(this);
    headerFrame->setObjectName("headerFrame");
    QHBoxLayout *headerLayout = new QHBoxLayout(headerFrame);
    headerLayout->setContentsMargins(12, 10, 12, 10);
    headerLayout->setSpacing(14);

    m_logoLabel = new QLabel(this);
    QPixmap logoPix(":/logo.png");
    if (!logoPix.isNull()) {
        m_logoLabel->setPixmap(logoPix.scaled(44, 44, Qt::KeepAspectRatio, Qt::SmoothTransformation));
    }
    headerLayout->addWidget(m_logoLabel);

    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(2);

    m_titleLabel = new QLabel("HyperXTools", this);
    m_titleLabel->setObjectName("headerTitle");

    QLabel *subtitleLabel = new QLabel("Pusat Manajemen Aplikasi & Paket Tambahan HyperXOS", this);
    subtitleLabel->setObjectName("headerSubtitle");

    titleLayout->addWidget(m_titleLabel);
    titleLayout->addWidget(subtitleLabel);
    headerLayout->addLayout(titleLayout);

    headerLayout->addStretch();

    m_statsLabel = new QLabel("Memuat paket...", this);
    m_statsLabel->setObjectName("headerStats");
    headerLayout->addWidget(m_statsLabel);

    mainLayout->addWidget(headerFrame);

    // ==========================================
    // 2. Middle Content: Categories + Packages
    // ==========================================
    QHBoxLayout *contentLayout = new QHBoxLayout();
    contentLayout->setSpacing(12);

    // Sidebar Categories
    m_categoryList = new QListWidget(this);
    m_categoryList->setObjectName("categoryList");
    m_categoryList->setFixedWidth(220);
    connect(m_categoryList, &QListWidget::currentRowChanged, this, &MainWindow::onCategorySelected);
    contentLayout->addWidget(m_categoryList);

    // Scroll Area for Packages
    m_scrollArea = new QScrollArea(this);
    m_scrollArea->setObjectName("packageScrollArea");
    m_scrollArea->setWidgetResizable(true);
    m_scrollArea->setFrameShape(QFrame::NoFrame);

    m_packageListContainer = new QWidget(m_scrollArea);
    m_packageListContainer->setObjectName("packageListContainer");
    m_packageListLayout = new QVBoxLayout(m_packageListContainer);
    m_packageListLayout->setContentsMargins(6, 6, 6, 6);
    m_packageListLayout->setSpacing(8);
    m_packageListLayout->addStretch();

    m_scrollArea->setWidget(m_packageListContainer);
    contentLayout->addWidget(m_scrollArea, 1);

    mainLayout->addLayout(contentLayout, 1);

    // ==========================================
    // 3. Progress Bar (hidden by default)
    // ==========================================
    m_progressBar = new QProgressBar(this);
    m_progressBar->setObjectName("progressBar");
    m_progressBar->setTextVisible(false);
    m_progressBar->setRange(0, 0); // Indeterminate
    m_progressBar->setFixedHeight(4);
    m_progressBar->hide();
    mainLayout->addWidget(m_progressBar);

    // ==========================================
    // 4. Action Controls Bar
    // ==========================================
    QFrame *actionFrame = new QFrame(this);
    actionFrame->setObjectName("actionFrame");
    QHBoxLayout *actionLayout = new QHBoxLayout(actionFrame);
    actionLayout->setContentsMargins(10, 8, 10, 8);
    actionLayout->setSpacing(10);

    m_refreshButton = new QPushButton("⟳ Periksa Status", this);
    m_refreshButton->setObjectName("refreshButton");
    connect(m_refreshButton, &QPushButton::clicked, this, &MainWindow::onRefreshClicked);
    actionLayout->addWidget(m_refreshButton);

    m_toggleLogButton = new QPushButton("Terminal Log ▼", this);
    m_toggleLogButton->setObjectName("toggleLogButton");
    connect(m_toggleLogButton, &QPushButton::clicked, this, &MainWindow::onToggleLogClicked);
    actionLayout->addWidget(m_toggleLogButton);

    actionLayout->addStretch();

    m_uninstallButton = new QPushButton("Hapus Terpilih", this);
    m_uninstallButton->setObjectName("uninstallButton");
    m_uninstallButton->setEnabled(false);
    connect(m_uninstallButton, &QPushButton::clicked, this, &MainWindow::onUninstallSelectedClicked);
    actionLayout->addWidget(m_uninstallButton);

    m_installButton = new QPushButton("Pasang Terpilih", this);
    m_installButton->setObjectName("installButton");
    m_installButton->setEnabled(false);
    connect(m_installButton, &QPushButton::clicked, this, &MainWindow::onInstallSelectedClicked);
    actionLayout->addWidget(m_installButton);

    mainLayout->addWidget(actionFrame);

    // ==========================================
    // 5. Embedded Terminal Log Console (collapsible)
    // ==========================================
    m_logConsole = new QPlainTextEdit(this);
    m_logConsole->setObjectName("logConsole");
    m_logConsole->setReadOnly(true);
    m_logConsole->setFixedHeight(160);
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);
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
            padding: 4px 10px;
            border-radius: 6px;
            border: 1px solid #363a4f;
        }
        #categoryList {
            background-color: #24273a;
            border: 1px solid #363a4f;
            border-radius: 8px;
            padding: 6px;
            font-size: 13px;
        }
        #categoryList::item {
            padding: 10px 12px;
            border-radius: 6px;
            color: #cad3f5;
            margin-bottom: 3px;
        }
        #categoryList::item:hover {
            background-color: #363a4f;
        }
        #categoryList::item:selected {
            background-color: #8aadf4;
            color: #181926;
            font-weight: bold;
        }
        #packageListContainer {
            background-color: transparent;
        }
        QFrame[card="true"] {
            background-color: #24273a;
            border: 1px solid #363a4f;
            border-radius: 8px;
            padding: 8px;
        }
        QFrame[card="true"]:hover {
            border-color: #8aadf4;
        }
        QLabel[pkgTitle="true"] {
            font-size: 14px;
            font-weight: bold;
            color: #cad3f5;
        }
        QLabel[pkgDesc="true"] {
            font-size: 11px;
            color: #a5adcb;
        }
        QLabel[badgeInstalled="true"] {
            background-color: rgba(166, 218, 149, 0.15);
            color: #a6da95;
            font-size: 10px;
            font-weight: bold;
            padding: 3px 8px;
            border-radius: 4px;
            border: 1px solid #a6da95;
        }
        QLabel[badgeNotInstalled="true"] {
            background-color: rgba(110, 115, 141, 0.15);
            color: #8087a2;
            font-size: 10px;
            padding: 3px 8px;
            border-radius: 4px;
            border: 1px solid #494d64;
        }
        QPushButton[cardBtn="install"] {
            background-color: #8aadf4;
            color: #181926;
            font-weight: bold;
            border-radius: 6px;
            padding: 6px 14px;
            font-size: 11px;
        }
        QPushButton[cardBtn="install"]:hover {
            background-color: #91d7e3;
        }
        QPushButton[cardBtn="uninstall"] {
            background-color: transparent;
            color: #ed8796;
            border: 1px solid #ed8796;
            font-weight: bold;
            border-radius: 6px;
            padding: 5px 12px;
            font-size: 11px;
        }
        QPushButton[cardBtn="uninstall"]:hover {
            background-color: #ed8796;
            color: #181926;
        }
        #actionFrame {
            background-color: #24273a;
            border: 1px solid #363a4f;
            border-radius: 8px;
        }
        #installButton {
            background-color: #8aadf4;
            color: #181926;
            font-weight: bold;
            font-size: 12px;
            border-radius: 6px;
            padding: 8px 18px;
        }
        #installButton:hover {
            background-color: #91d7e3;
        }
        #installButton:disabled {
            background-color: #363a4f;
            color: #6e738d;
        }
        #uninstallButton {
            background-color: #ed8796;
            color: #181926;
            font-weight: bold;
            font-size: 12px;
            border-radius: 6px;
            padding: 8px 18px;
        }
        #uninstallButton:hover {
            background-color: #ee99a0;
        }
        #uninstallButton:disabled {
            background-color: #363a4f;
            color: #6e738d;
        }
        #refreshButton, #toggleLogButton {
            background-color: #363a4f;
            color: #cad3f5;
            font-size: 12px;
            border-radius: 6px;
            padding: 8px 14px;
            border: 1px solid #494d64;
        }
        #refreshButton:hover, #toggleLogButton:hover {
            background-color: #494d64;
            color: #ffffff;
        }
        #logConsole {
            background-color: #141520;
            color: #a6da95;
            font-family: 'JetBrains Mono', 'Hack', 'Fira Code', 'monospace';
            font-size: 11px;
            border: 1px solid #363a4f;
            border-radius: 6px;
            padding: 6px;
        }
        QCheckBox::indicator {
            width: 18px;
            height: 18px;
            border-radius: 4px;
            border: 1px solid #494d64;
            background-color: #1e2030;
        }
        QCheckBox::indicator:hover {
            border-color: #8aadf4;
        }
        QCheckBox::indicator:checked {
            background-color: #8aadf4;
            border-color: #8aadf4;
            image: url(:/checked.png);
        }
        QScrollBar:vertical {
            border: none;
            background: #181926;
            width: 8px;
            border-radius: 4px;
        }
        QScrollBar::handle:vertical {
            background: #363a4f;
            min-height: 20px;
            border-radius: 4px;
        }
        QScrollBar::handle:vertical:hover {
            background: #8aadf4;
        }
    )";
    setStyleSheet(qss);
}

void MainWindow::populateCategories()
{
    m_categoryList->clear();
    QStringList categories = m_packageManager->getCategories();

    for (const QString &cat : categories) {
        QString displayName = m_packageManager->getCategoryDisplayName(cat);
        QListWidgetItem *item = new QListWidgetItem(displayName, m_categoryList);
        item->setData(Qt::UserRole, cat);
    }
}

void MainWindow::onCategorySelected(int index)
{
    if (index < 0 || index >= m_categoryList->count()) return;
    QListWidgetItem *item = m_categoryList->item(index);
    m_currentCategory = item->data(Qt::UserRole).toString();
    renderPackageList(m_currentCategory);
}

void MainWindow::renderPackageList(const QString &category)
{
    // Clear previous cards
    QLayoutItem *child;
    while ((child = m_packageListLayout->takeAt(0)) != nullptr) {
        if (child->widget()) {
            child->widget()->deleteLater();
        }
        delete child;
    }
    m_checkboxMap.clear();

    QList<PackageItem> packages = m_packageManager->getPackagesByCategory(category);

    for (const PackageItem &pkg : packages) {
        QFrame *card = new QFrame(m_packageListContainer);
        card->setProperty("card", true);
        QHBoxLayout *cardLayout = new QHBoxLayout(card);
        cardLayout->setContentsMargins(12, 10, 12, 10);
        cardLayout->setSpacing(12);

        // Checkbox: automatically checked if installed!
        QCheckBox *cb = new QCheckBox(card);
        cb->setChecked(pkg.isInstalled);
        m_checkboxMap.insert(pkg.id, cb);

        connect(cb, &QCheckBox::toggled, this, [this, pkg](bool checked) {
            onPackageCheckboxChanged(pkg.id, checked);
        });
        cardLayout->addWidget(cb);

        // Name and description
        QVBoxLayout *infoLayout = new QVBoxLayout();
        infoLayout->setSpacing(3);

        QHBoxLayout *titleBadgeLayout = new QHBoxLayout();
        titleBadgeLayout->setSpacing(8);

        QLabel *titleLabel = new QLabel(pkg.name, card);
        titleLabel->setProperty("pkgTitle", true);
        titleBadgeLayout->addWidget(titleLabel);

        QLabel *statusBadge = new QLabel(card);
        if (pkg.isInstalled) {
            statusBadge->setText("✓ Terpasang");
            statusBadge->setProperty("badgeInstalled", true);
        } else {
            statusBadge->setText("Belum Terpasang");
            statusBadge->setProperty("badgeNotInstalled", true);
        }
        titleBadgeLayout->addWidget(statusBadge);
        titleBadgeLayout->addStretch();

        QLabel *descLabel = new QLabel(pkg.description, card);
        descLabel->setProperty("pkgDesc", true);
        descLabel->setWordWrap(true);

        infoLayout->addLayout(titleBadgeLayout);
        infoLayout->addWidget(descLabel);
        cardLayout->addLayout(infoLayout, 1);

        // Action Button: Install or Uninstall
        QPushButton *actionBtn = new QPushButton(card);
        if (pkg.isInstalled) {
            actionBtn->setText("Hapus");
            actionBtn->setProperty("cardBtn", "uninstall");
            connect(actionBtn, &QPushButton::clicked, this, [this, pkg]() {
                onSingleUninstallClicked(pkg.id);
            });
        } else {
            actionBtn->setText("Pasang");
            actionBtn->setProperty("cardBtn", "install");
            connect(actionBtn, &QPushButton::clicked, this, [this, pkg]() {
                onSingleInstallClicked(pkg.id);
            });
        }
        cardLayout->addWidget(actionBtn);

        m_packageListLayout->addWidget(card);
    }

    m_packageListLayout->addStretch();
    updateStatsLabel();
}

void MainWindow::onPackageCheckboxChanged(const QString &packageId, bool checked)
{
    for (const PackageItem &pkg : m_packageManager->getAllPackages()) {
        if (pkg.id == packageId) {
            if (pkg.isInstalled && !checked) {
                // Was installed, now unchecked -> selected for uninstall
                m_selectedToUninstall.insert(packageId);
                m_selectedToInstall.remove(packageId);
            } else if (!pkg.isInstalled && checked) {
                // Was not installed, now checked -> selected for install
                m_selectedToInstall.insert(packageId);
                m_selectedToUninstall.remove(packageId);
            } else {
                m_selectedToInstall.remove(packageId);
                m_selectedToUninstall.remove(packageId);
            }
            break;
        }
    }

    m_installButton->setEnabled(!m_selectedToInstall.isEmpty() && !m_packageManager->isBusy());
    m_installButton->setText(m_selectedToInstall.isEmpty()
                             ? "Pasang Terpilih"
                             : QString("Pasang Terpilih (%1)").arg(m_selectedToInstall.size()));

    m_uninstallButton->setEnabled(!m_selectedToUninstall.isEmpty() && !m_packageManager->isBusy());
    m_uninstallButton->setText(m_selectedToUninstall.isEmpty()
                               ? "Hapus Terpilih"
                               : QString("Hapus Terpilih (%1)").arg(m_selectedToUninstall.size()));

    updateStatsLabel();
}

void MainWindow::onSingleInstallClicked(const QString &packageId)
{
    m_packageManager->executeInstall({packageId});
}

void MainWindow::onSingleUninstallClicked(const QString &packageId)
{
    QMessageBox::StandardButton reply = QMessageBox::question(
        this, "Konfirmasi Penghapusan",
        QString("Apakah Anda yakin ingin menghapus paket '%1' dari sistem?").arg(packageId),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_packageManager->executeUninstall({packageId});
    }
}

void MainWindow::onInstallSelectedClicked()
{
    if (m_selectedToInstall.isEmpty()) return;
    m_packageManager->executeInstall(m_selectedToInstall.values());
    m_selectedToInstall.clear();
    m_installButton->setEnabled(false);
    m_installButton->setText("Pasang Terpilih");
}

void MainWindow::onUninstallSelectedClicked()
{
    if (m_selectedToUninstall.isEmpty()) return;

    QMessageBox::StandardButton reply = QMessageBox::question(
        this, "Konfirmasi Penghapusan",
        QString("Apakah Anda yakin ingin menghapus %1 paket terpilih dari sistem?").arg(m_selectedToUninstall.size()),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_packageManager->executeUninstall(m_selectedToUninstall.values());
        m_selectedToUninstall.clear();
        m_uninstallButton->setEnabled(false);
        m_uninstallButton->setText("Hapus Terpilih");
    }
}

void MainWindow::onRefreshClicked()
{
    m_packageManager->refreshInstalledCache();
}

void MainWindow::onToggleLogClicked()
{
    if (m_logConsole->isVisible()) {
        m_logConsole->hide();
        m_toggleLogButton->setText("Terminal Log ▼");
    } else {
        m_logConsole->show();
        m_toggleLogButton->setText("Terminal Log ▲");
    }
}

void MainWindow::onLogReceived(const QString &text)
{
    m_logConsole->appendPlainText(text);
    m_logConsole->verticalScrollBar()->setValue(m_logConsole->verticalScrollBar()->maximum());
}

void MainWindow::onProcessStarted(const QString &title)
{
    m_progressBar->show();
    m_installButton->setEnabled(false);
    m_uninstallButton->setEnabled(false);
    m_refreshButton->setEnabled(false);
    m_logConsole->show();
    m_toggleLogButton->setText("Terminal Log ▲");
    m_logConsole->appendPlainText(QString("\n--- %1 ---\n").arg(title));
}

void MainWindow::onProcessFinished(bool success, const QString &message)
{
    m_progressBar->hide();
    m_refreshButton->setEnabled(true);
    m_installButton->setEnabled(!m_selectedToInstall.isEmpty());
    m_uninstallButton->setEnabled(!m_selectedToUninstall.isEmpty());

    if (success) {
        QMessageBox::information(this, "Operasi Berhasil", message);
    } else {
        QMessageBox::warning(this, "Operasi Gagal", message);
    }
}

void MainWindow::onCacheRefreshed()
{
    renderPackageList(m_currentCategory);
}

void MainWindow::updateStatsLabel()
{
    int total = 0;
    int installed = 0;

    for (const PackageItem &pkg : m_packageManager->getAllPackages()) {
        total++;
        if (pkg.isInstalled) {
            installed++;
        }
    }

    m_statsLabel->setText(QString("Total: %1 | Terpasang: %2 | Dipilih: +%3 -%4")
                          .arg(total)
                          .arg(installed)
                          .arg(m_selectedToInstall.size())
                          .arg(m_selectedToUninstall.size()));
}

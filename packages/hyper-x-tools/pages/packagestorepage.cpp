#include "packagestorepage.h"
#include <QFrame>
#include <QMessageBox>

PackageStorePage::PackageStorePage(PackageManager *manager, QWidget *parent)
    : QWidget(parent), m_manager(manager), m_currentCategory("all")
{
    setupUi();

    if (m_manager) {
        connect(m_manager, &PackageManager::logReceived, this, &PackageStorePage::onLogReceived);
        connect(m_manager, &PackageManager::processStarted, this, &PackageStorePage::onProcessStarted);
        connect(m_manager, &PackageManager::processFinished, this, &PackageStorePage::onProcessFinished);
        connect(m_manager, &PackageManager::cacheRefreshed, this, &PackageStorePage::onCacheRefreshed);

        renderPackageCards();
    }
}

PackageStorePage::~PackageStorePage()
{
}

void PackageStorePage::setupUi()
{
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(14);

    // Title & Header
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("📦 Katalog Aplikasi & App Store", this);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel(
        "Koleksi aplikasi pilihan HyperXOS: peramban web, suite perkantoran, media player, toolchain developer, dan utilitas.", this);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // Filter & Search Controls Row
    QHBoxLayout *filterRow = new QHBoxLayout();
    filterRow->setSpacing(10);

    m_searchInput = new QLineEdit(this);
    m_searchInput->setPlaceholderText("🔍 Cari nama aplikasi atau kata kunci deskripsi...");
    m_searchInput->setStyleSheet("background-color: #24273a; color: #cad3f5; border: 1px solid #363a4f; border-radius: 6px; padding: 8px 12px;");
    connect(m_searchInput, &QLineEdit::textChanged, this, &PackageStorePage::onSearchTextChanged);
    filterRow->addWidget(m_searchInput, 1);

    m_categoryCombo = new QComboBox(this);
    m_categoryCombo->setStyleSheet("background-color: #24273a; color: #cad3f5; border: 1px solid #363a4f; border-radius: 6px; padding: 6px 12px;");
    m_categoryCombo->addItem("Semua Kategori", "all");

    if (m_manager) {
        QStringList cats = m_manager->getCategories();
        for (const QString &c : cats) {
            m_categoryCombo->addItem(m_manager->getCategoryDisplayName(c), c);
        }
    }
    connect(m_categoryCombo, QOverload<int>::of(&QComboBox::currentIndexChanged), this, &PackageStorePage::onCategoryChanged);
    filterRow->addWidget(m_categoryCombo);

    m_refreshBtn = new QPushButton("⟳ Segarkan", this);
    m_refreshBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; border-radius: 6px; padding: 8px 14px; font-weight: bold;");
    connect(m_refreshBtn, &QPushButton::clicked, this, &PackageStorePage::onRefreshClicked);
    filterRow->addWidget(m_refreshBtn);

    mainLayout->addLayout(filterRow);

    // Status Summary Label
    m_statusSummaryLabel = new QLabel("Memuat katalog...", this);
    m_statusSummaryLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
    mainLayout->addWidget(m_statusSummaryLabel);

    // Scroll Area for Packages
    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setFrameShape(QFrame::NoFrame);

    m_cardsContainer = new QWidget(scrollArea);
    m_cardsLayout = new QVBoxLayout(m_cardsContainer);
    m_cardsLayout->setContentsMargins(0, 0, 0, 0);
    m_cardsLayout->setSpacing(8);
    m_cardsLayout->addStretch();

    scrollArea->setWidget(m_cardsContainer);
    mainLayout->addWidget(scrollArea, 1);

    // Progress Bar
    m_progressBar = new QProgressBar(this);
    m_progressBar->setFixedHeight(4);
    m_progressBar->setTextVisible(false);
    m_progressBar->setRange(0, 0);
    m_progressBar->hide();
    mainLayout->addWidget(m_progressBar);

    // Bottom Action Bar
    QFrame *bottomFrame = new QFrame(this);
    bottomFrame->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 8px; padding: 10px;");
    QHBoxLayout *bottomLayout = new QHBoxLayout(bottomFrame);
    bottomLayout->setSpacing(10);

    QPushButton *toggleLogBtn = new QPushButton("Terminal Log ▼", bottomFrame);
    toggleLogBtn->setStyleSheet("background-color: #1e2030; color: #cad3f5; border: 1px solid #363a4f; border-radius: 6px; padding: 6px 12px;");
    connect(toggleLogBtn, &QPushButton::clicked, this, [this]() {
        m_logConsole->setVisible(!m_logConsole->isVisible());
    });
    bottomLayout->addWidget(toggleLogBtn);

    bottomLayout->addStretch();

    m_uninstallSelectedBtn = new QPushButton("Hapus Terpilih", bottomFrame);
    m_uninstallSelectedBtn->setStyleSheet("background-color: #ed8796; color: #181926; font-weight: bold; border-radius: 6px; padding: 7px 16px;");
    m_uninstallSelectedBtn->setEnabled(false);
    connect(m_uninstallSelectedBtn, &QPushButton::clicked, this, &PackageStorePage::onUninstallSelectedClicked);
    bottomLayout->addWidget(m_uninstallSelectedBtn);

    m_installSelectedBtn = new QPushButton("Pasang Terpilih", bottomFrame);
    m_installSelectedBtn->setStyleSheet("background-color: #a6da95; color: #181926; font-weight: bold; border-radius: 6px; padding: 7px 16px;");
    m_installSelectedBtn->setEnabled(false);
    connect(m_installSelectedBtn, &QPushButton::clicked, this, &PackageStorePage::onInstallSelectedClicked);
    bottomLayout->addWidget(m_installSelectedBtn);

    mainLayout->addWidget(bottomFrame);

    // Log Console
    m_logConsole = new QPlainTextEdit(this);
    m_logConsole->setReadOnly(true);
    m_logConsole->setFixedHeight(120);
    m_logConsole->setStyleSheet("background-color: #11121d; color: #cad3f5; font-family: monospace; border: 1px solid #363a4f; border-radius: 6px;");
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);
}

void PackageStorePage::renderPackageCards()
{
    if (!m_manager) return;

    // Clear previous widgets
    QLayoutItem *child;
    while ((child = m_cardsLayout->takeAt(0)) != nullptr) {
        if (child->widget()) delete child->widget();
        delete child;
    }
    m_checkboxMap.clear();

    QString cat = (m_currentCategory == "all") ? QString() : m_currentCategory;
    QList<PackageItem> items = m_manager->searchPackages(m_searchQuery, cat);

    int installedCount = 0;
    for (const PackageItem &p : items) {
        if (p.isInstalled) installedCount++;

        QFrame *card = new QFrame(m_cardsContainer);
        card->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 8px; padding: 10px;");

        QHBoxLayout *cardLayout = new QHBoxLayout(card);
        cardLayout->setSpacing(12);

        QCheckBox *cb = new QCheckBox(card);
        cb->setStyleSheet("QCheckBox::indicator { width: 18px; height: 18px; }");
        m_checkboxMap[p.id] = cb;
        connect(cb, &QCheckBox::toggled, this, [this, p](bool checked) {
            onPackageCheckboxChanged(p.id, checked);
        });
        cardLayout->addWidget(cb);

        QVBoxLayout *textLayout = new QVBoxLayout();
        textLayout->setSpacing(2);

        QHBoxLayout *nameRow = new QHBoxLayout();
        QLabel *nameLabel = new QLabel(p.name, card);
        nameLabel->setStyleSheet("font-size: 14px; font-weight: bold; color: #cad3f5;");
        nameRow->addWidget(nameLabel);

        if (p.isInstalled) {
            QLabel *badge = new QLabel("TERPASANG", card);
            badge->setStyleSheet("background-color: #363a4f; color: #a6da95; font-size: 10px; font-weight: bold; padding: 2px 6px; border-radius: 4px;");
            nameRow->addWidget(badge);
        }
        nameRow->addStretch();
        textLayout->addLayout(nameRow);

        QLabel *descLabel = new QLabel(p.description, card);
        descLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
        descLabel->setWordWrap(true);
        textLayout->addWidget(descLabel);

        QLabel *pkgListLabel = new QLabel(QString("Paket: %1").arg(p.packages.join(", ")), card);
        pkgListLabel->setStyleSheet("font-size: 10px; color: #6e738d;");
        textLayout->addWidget(pkgListLabel);

        cardLayout->addLayout(textLayout, 1);

        // Action button per card
        if (p.isInstalled) {
            QPushButton *btn = new QPushButton("Hapus", card);
            btn->setStyleSheet("background-color: #363a4f; color: #ed8796; border-radius: 6px; padding: 6px 14px; font-weight: bold;");
            connect(btn, &QPushButton::clicked, this, [this, p]() {
                onSingleUninstallClicked(p.id);
            });
            cardLayout->addWidget(btn);
        } else {
            QPushButton *btn = new QPushButton("Pasang", card);
            btn->setStyleSheet("background-color: #8aadf4; color: #181926; border-radius: 6px; padding: 6px 14px; font-weight: bold;");
            connect(btn, &QPushButton::clicked, this, [this, p]() {
                onSingleInstallClicked(p.id);
            });
            cardLayout->addWidget(btn);
        }

        m_cardsLayout->addWidget(card);
    }

    m_cardsLayout->addStretch();
    m_statusSummaryLabel->setText(QString("Menampilkan %1 aplikasi (%2 terpasang pada sistem).")
                                      .arg(items.size()).arg(installedCount));
    updateActionButtons();
}

void PackageStorePage::onSearchTextChanged(const QString &text)
{
    m_searchQuery = text;
    renderPackageCards();
}

void PackageStorePage::onCategoryChanged(int index)
{
    m_currentCategory = m_categoryCombo->itemData(index).toString();
    renderPackageCards();
}

void PackageStorePage::onRefreshClicked()
{
    if (m_manager) {
        m_manager->refreshInstalledCache();
    }
}

void PackageStorePage::onPackageCheckboxChanged(const QString &packageId, bool checked)
{
    for (const auto &item : m_manager->getAllPackages()) {
        if (item.id == packageId) {
            if (item.isInstalled) {
                if (checked) m_selectedToUninstall.insert(packageId);
                else m_selectedToUninstall.remove(packageId);
            } else {
                if (checked) m_selectedToInstall.insert(packageId);
                else m_selectedToInstall.remove(packageId);
            }
            break;
        }
    }
    updateActionButtons();
}

void PackageStorePage::updateActionButtons()
{
    m_installSelectedBtn->setEnabled(!m_selectedToInstall.isEmpty() && (!m_manager || !m_manager->isBusy()));
    m_installSelectedBtn->setText(QString("Pasang Terpilih (%1)").arg(m_selectedToInstall.size()));

    m_uninstallSelectedBtn->setEnabled(!m_selectedToUninstall.isEmpty() && (!m_manager || !m_manager->isBusy()));
    m_uninstallSelectedBtn->setText(QString("Hapus Terpilih (%1)").arg(m_selectedToUninstall.size()));
}

void PackageStorePage::onSingleInstallClicked(const QString &packageId)
{
    if (!m_manager || m_manager->isBusy()) return;
    for (const auto &item : m_manager->getAllPackages()) {
        if (item.id == packageId) {
            m_manager->executeInstall(item.packages);
            break;
        }
    }
}

void PackageStorePage::onSingleUninstallClicked(const QString &packageId)
{
    if (!m_manager || m_manager->isBusy()) return;
    for (const auto &item : m_manager->getAllPackages()) {
        if (item.id == packageId) {
            m_manager->executeUninstall(item.packages);
            break;
        }
    }
}

void PackageStorePage::onInstallSelectedClicked()
{
    if (!m_manager || m_manager->isBusy() || m_selectedToInstall.isEmpty()) return;

    QStringList pkgs;
    for (const QString &id : m_selectedToInstall) {
        for (const auto &item : m_manager->getAllPackages()) {
            if (item.id == id) {
                pkgs.append(item.packages);
            }
        }
    }
    m_selectedToInstall.clear();
    m_manager->executeInstall(pkgs);
}

void PackageStorePage::onUninstallSelectedClicked()
{
    if (!m_manager || m_manager->isBusy() || m_selectedToUninstall.isEmpty()) return;

    QStringList pkgs;
    for (const QString &id : m_selectedToUninstall) {
        for (const auto &item : m_manager->getAllPackages()) {
            if (item.id == id) {
                pkgs.append(item.packages);
            }
        }
    }
    m_selectedToUninstall.clear();
    m_manager->executeUninstall(pkgs);
}

void PackageStorePage::onLogReceived(const QString &text)
{
    m_logConsole->appendPlainText(text);
}

void PackageStorePage::onProcessStarted(const QString &title)
{
    Q_UNUSED(title);
    m_progressBar->show();
    m_logConsole->show();
    updateActionButtons();
}

void PackageStorePage::onProcessFinished(bool success, const QString &message)
{
    m_progressBar->hide();
    updateActionButtons();
    if (success) {
        QMessageBox::information(this, "Selesai", message);
    } else {
        QMessageBox::critical(this, "Galat", message);
    }
}

void PackageStorePage::onCacheRefreshed()
{
    renderPackageCards();
}

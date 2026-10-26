#include "gametweakspage.h"
#include <QFrame>
#include <QScrollArea>
#include <QMessageBox>

GamingTweaksPage::GamingTweaksPage(SystemTweaker *tweaker, QWidget *parent)
    : QWidget(parent), m_tweaker(tweaker)
{
    setupUi();

    if (m_tweaker) {
        connect(m_tweaker, &SystemTweaker::statusRefreshed, this, &GamingTweaksPage::onStatusRefreshed);
        connect(m_tweaker, &SystemTweaker::operationStarted, this, &GamingTweaksPage::onOperationStarted);
        connect(m_tweaker, &SystemTweaker::operationFinished, this, &GamingTweaksPage::onOperationFinished);
        connect(m_tweaker, &SystemTweaker::logOutput, this, &GamingTweaksPage::onLogReceived);
        onStatusRefreshed(m_tweaker->getStatus());
    }
}

GamingTweaksPage::~GamingTweaksPage()
{
}

void GamingTweaksPage::setupUi()
{
    QVBoxLayout *rootLayout = new QVBoxLayout(this);
    rootLayout->setContentsMargins(0, 0, 0, 0);

    QScrollArea *scrollArea = new QScrollArea(this);
    scrollArea->setWidgetResizable(true);
    scrollArea->setFrameShape(QFrame::NoFrame);

    QWidget *container = new QWidget(scrollArea);
    QVBoxLayout *mainLayout = new QVBoxLayout(container);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(18);

    // Title & Header
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("🎮 Tweak Performa & Gaming HyperXOS", container);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel(
        "Optimalisasi penjadwal CPU, memori virtual kernel, daemon gamemode, dan toolchain game sekali klik.", container);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // 1. CPU Governor Control Card
    QFrame *govCard = new QFrame(container);
    govCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *govLayout = new QVBoxLayout(govCard);
    govLayout->setSpacing(10);

    QHBoxLayout *govHeader = new QHBoxLayout();
    QLabel *govTitle = new QLabel("⚡ CPU Scaling Governor (Manajemen Frekuensi CPU)", govCard);
    govTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #f5a97f;");
    govHeader->addWidget(govTitle);
    govHeader->addStretch();

    m_currentGovLabel = new QLabel("Governor Saat Ini: ...", govCard);
    m_currentGovLabel->setStyleSheet("font-size: 11px; background-color: #1e2030; color: #a6da95; padding: 3px 8px; border-radius: 4px; font-weight: bold;");
    govHeader->addWidget(m_currentGovLabel);
    govLayout->addLayout(govHeader);

    QLabel *govDesc = new QLabel(
        "Mode Performance memacu CPU ke frekuensi maksimal tanpa throttling agresif. "
        "Mode Schedutil menyeimbangkan performa dengan konsumsi daya sesuai beban kerja.", govCard);
    govDesc->setStyleSheet("font-size: 11px; color: #a5adcb;");
    govDesc->setWordWrap(true);
    govLayout->addWidget(govDesc);

    QHBoxLayout *govActionLayout = new QHBoxLayout();
    m_govComboBox = new QComboBox(govCard);
    m_govComboBox->setStyleSheet("background-color: #1e2030; color: #cad3f5; border: 1px solid #363a4f; border-radius: 6px; padding: 6px 12px;");
    m_govComboBox->setMinimumWidth(200);

    m_applyGovBtn = new QPushButton("Terapkan Governor", govCard);
    m_applyGovBtn->setStyleSheet("background-color: #8aadf4; color: #181926; font-weight: bold; border-radius: 6px; padding: 7px 16px;");
    connect(m_applyGovBtn, &QPushButton::clicked, this, &GamingTweaksPage::onApplyGovernorClicked);

    govActionLayout->addWidget(m_govComboBox);
    govActionLayout->addWidget(m_applyGovBtn);
    govActionLayout->addStretch();
    govLayout->addLayout(govActionLayout);

    mainLayout->addWidget(govCard);

    // 2. Kernel Memory & Anti-Stuttering Card
    QFrame *memCard = new QFrame(container);
    memCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *memLayout = new QVBoxLayout(memCard);
    memLayout->setSpacing(14);

    QLabel *memTitle = new QLabel("🧠 Tweak Memori Kernel & Stabilitas Game", memCard);
    memTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #8bd5ca;");
    memLayout->addWidget(memTitle);

    // Row A: vm.max_map_count
    QHBoxLayout *mapRow = new QHBoxLayout();
    QVBoxLayout *mapText = new QVBoxLayout();
    QLabel *mapTitle = new QLabel("Virtual Memory Map Count (`vm.max_map_count`)", memCard);
    mapTitle->setStyleSheet("font-weight: bold; color: #cad3f5;");
    m_mapCountStatusLabel = new QLabel("Nilai saat ini: Memuat...", memCard);
    m_mapCountStatusLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
    mapText->addWidget(mapTitle);
    mapText->addWidget(m_mapCountStatusLabel);

    m_applyMapCountBtn = new QPushButton("Aktifkan Tweak Gaming (2147483642)", memCard);
    m_applyMapCountBtn->setStyleSheet("background-color: #a6da95; color: #181926; font-weight: bold; border-radius: 6px; padding: 6px 14px;");
    connect(m_applyMapCountBtn, &QPushButton::clicked, this, &GamingTweaksPage::onApplyMapCountClicked);

    mapRow->addLayout(mapText);
    mapRow->addStretch();
    mapRow->addWidget(m_applyMapCountBtn);
    memLayout->addLayout(mapRow);

    // Divider
    QFrame *line1 = new QFrame(memCard);
    line1->setFrameShape(QFrame::HLine);
    line1->setStyleSheet("color: #363a4f;");
    memLayout->addWidget(line1);

    // Row B: Split lock mitigate
    QHBoxLayout *splitRow = new QHBoxLayout();
    QVBoxLayout *splitText = new QVBoxLayout();
    QLabel *splitTitle = new QLabel("Split Lock Mitigation (Anti-Stuttering FPS)", memCard);
    splitTitle->setStyleSheet("font-weight: bold; color: #cad3f5;");
    m_splitLockStatusLabel = new QLabel("Status saat ini: Memuat...", memCard);
    m_splitLockStatusLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
    splitText->addWidget(splitTitle);
    splitText->addWidget(m_splitLockStatusLabel);

    m_toggleSplitLockBtn = new QPushButton("Nonaktifkan Penalti (Anti-Stuttering)", memCard);
    m_toggleSplitLockBtn->setStyleSheet("background-color: #f5a97f; color: #181926; font-weight: bold; border-radius: 6px; padding: 6px 14px;");
    connect(m_toggleSplitLockBtn, &QPushButton::clicked, this, &GamingTweaksPage::onToggleSplitLockClicked);

    splitRow->addLayout(splitText);
    splitRow->addStretch();
    splitRow->addWidget(m_toggleSplitLockBtn);
    memLayout->addLayout(splitRow);

    // Divider
    QFrame *line2 = new QFrame(memCard);
    line2->setFrameShape(QFrame::HLine);
    line2->setStyleSheet("color: #363a4f;");
    memLayout->addWidget(line2);

    // Row C: GameMode Daemon
    QHBoxLayout *gmRow = new QHBoxLayout();
    QVBoxLayout *gmText = new QVBoxLayout();
    QLabel *gmTitle = new QLabel("Feral GameMode Daemon (`gamemoded`)", memCard);
    gmTitle->setStyleSheet("font-weight: bold; color: #cad3f5;");
    m_gameModeStatusLabel = new QLabel("Status daemon: Memuat...", memCard);
    m_gameModeStatusLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
    gmText->addWidget(gmTitle);
    gmText->addWidget(m_gameModeStatusLabel);

    m_toggleGameModeBtn = new QPushButton("Alihkan GameMode", memCard);
    m_toggleGameModeBtn->setStyleSheet("background-color: #c6a0f6; color: #181926; font-weight: bold; border-radius: 6px; padding: 6px 14px;");
    connect(m_toggleGameModeBtn, &QPushButton::clicked, this, &GamingTweaksPage::onToggleGameModeClicked);

    gmRow->addLayout(gmText);
    gmRow->addStretch();
    gmRow->addWidget(m_toggleGameModeBtn);
    memLayout->addLayout(gmRow);

    mainLayout->addWidget(memCard);

    // 3. Gaming Toolchain Quick Install Card
    QFrame *runtimesCard = new QFrame(container);
    runtimesCard->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QHBoxLayout *rtLayout = new QHBoxLayout(runtimesCard);
    rtLayout->setSpacing(14);

    QVBoxLayout *rtText = new QVBoxLayout();
    QLabel *rtTitle = new QLabel("🕹️ Toolchain & Runtime Gaming Esensial", runtimesCard);
    rtTitle->setStyleSheet("font-size: 14px; font-weight: bold; color: #c6a0f6;");
    m_runtimesStatusLabel = new QLabel("Memeriksa status paket gaming (Steam, Lutris, MangoHud, VKD3D)...", runtimesCard);
    m_runtimesStatusLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
    rtText->addWidget(rtTitle);
    rtText->addWidget(m_runtimesStatusLabel);

    m_installRuntimesBtn = new QPushButton("Pasang Toolchain Gaming", runtimesCard);
    m_installRuntimesBtn->setStyleSheet("background-color: #eed49f; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 16px;");
    connect(m_installRuntimesBtn, &QPushButton::clicked, this, &GamingTweaksPage::onInstallGamingRuntimesClicked);

    rtLayout->addLayout(rtText);
    rtLayout->addStretch();
    rtLayout->addWidget(m_installRuntimesBtn);

    mainLayout->addWidget(runtimesCard);

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
    m_logConsole->setFixedHeight(120);
    m_logConsole->setStyleSheet("background-color: #11121d; color: #cad3f5; font-family: monospace; border: 1px solid #363a4f; border-radius: 6px;");
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);

    mainLayout->addStretch();
    scrollArea->setWidget(container);
    rootLayout->addWidget(scrollArea);
}

void GamingTweaksPage::onStatusRefreshed(const TweakStatus &status)
{
    // Update Governor
    m_currentGovLabel->setText(QString("Governor Saat Ini: %1").arg(status.currentGovernor));
    m_govComboBox->clear();
    for (const QString &gov : status.availableGovernors) {
        m_govComboBox->addItem(gov);
    }
    int idx = m_govComboBox->findText(status.currentGovernor);
    if (idx >= 0) m_govComboBox->setCurrentIndex(idx);

    // Update vm.max_map_count
    if (status.isGamingMapCount) {
        m_mapCountStatusLabel->setText(QString("Nilai saat ini: %1 (Teroptimasi untuk Gaming ✓)").arg(status.maxMapCount));
        m_mapCountStatusLabel->setStyleSheet("font-size: 11px; color: #a6da95; font-weight: bold;");
        m_applyMapCountBtn->setEnabled(false);
        m_applyMapCountBtn->setText("Sudah Teroptimasi");
    } else {
        m_mapCountStatusLabel->setText(QString("Nilai saat ini: %1 (Nilai standar, disarankan ditingkatkan)").arg(status.maxMapCount));
        m_mapCountStatusLabel->setStyleSheet("font-size: 11px; color: #eed49f;");
        m_applyMapCountBtn->setEnabled(true);
        m_applyMapCountBtn->setText("Aktifkan Tweak Gaming");
    }

    // Update Split Lock
    if (status.splitLockMitigate == 0) {
        m_splitLockStatusLabel->setText("Status: 0 (Penalti nonaktif, performa FPS stabil ✓)");
        m_splitLockStatusLabel->setStyleSheet("font-size: 11px; color: #a6da95; font-weight: bold;");
        m_toggleSplitLockBtn->setText("Aktifkan Kembali Standar");
    } else if (status.splitLockMitigate == 1) {
        m_splitLockStatusLabel->setText("Status: 1 (Mitigasi aktif, berpotensi stuttering pada game)");
        m_splitLockStatusLabel->setStyleSheet("font-size: 11px; color: #eed49f;");
        m_toggleSplitLockBtn->setText("Nonaktifkan Penalti (Rekomendasi)");
    } else {
        m_splitLockStatusLabel->setText("Fitur split lock mitigate tidak didukung arsitektur CPU ini.");
        m_toggleSplitLockBtn->setEnabled(false);
    }

    // Update GameMode
    if (status.isGameModeActive) {
        m_gameModeStatusLabel->setText("Daemon Gamemode: AKTIF (Running)");
        m_gameModeStatusLabel->setStyleSheet("font-size: 11px; color: #a6da95; font-weight: bold;");
        m_toggleGameModeBtn->setText("Nonaktifkan Daemon");
    } else {
        m_gameModeStatusLabel->setText("Daemon Gamemode: NONAKTIF");
        m_gameModeStatusLabel->setStyleSheet("font-size: 11px; color: #a5adcb;");
        m_toggleGameModeBtn->setText("Aktifkan Daemon");
    }

    // Runtimes status
    QStringList installedList;
    if (status.isSteamInstalled) installedList << "Steam";
    if (status.isLutrisInstalled) installedList << "Lutris";
    if (status.isHeroicInstalled) installedList << "Heroic";
    if (status.isMangoHudInstalled) installedList << "MangoHud";

    if (!installedList.isEmpty()) {
        m_runtimesStatusLabel->setText(QString("Terpasang: %1").arg(installedList.join(", ")));
        m_runtimesStatusLabel->setStyleSheet("font-size: 11px; color: #8bd5ca; font-weight: bold;");
    } else {
        m_runtimesStatusLabel->setText("Belum ada toolchain game esensial yang terpasang.");
        m_runtimesStatusLabel->setStyleSheet("font-size: 11px; color: #eed49f;");
    }
}

void GamingTweaksPage::onApplyGovernorClicked()
{
    if (!m_tweaker || m_tweaker->isBusy()) return;
    QString gov = m_govComboBox->currentText();
    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();
    m_tweaker->setCpuGovernor(gov);
}

void GamingTweaksPage::onApplyMapCountClicked()
{
    if (!m_tweaker || m_tweaker->isBusy()) return;
    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();
    m_tweaker->applyGamingMemoryTweak();
}

void GamingTweaksPage::onToggleSplitLockClicked()
{
    if (!m_tweaker || m_tweaker->isBusy()) return;
    TweakStatus s = m_tweaker->getStatus();
    bool enable = (s.splitLockMitigate == 0); // Toggle to 1 if currently 0
    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();
    m_tweaker->setSplitLockMitigate(enable);
}

void GamingTweaksPage::onToggleGameModeClicked()
{
    if (!m_tweaker) return;
    TweakStatus s = m_tweaker->getStatus();
    m_tweaker->toggleGameModeService(!s.isGameModeActive);
}

void GamingTweaksPage::onInstallGamingRuntimesClicked()
{
    if (!m_tweaker || m_tweaker->isBusy()) return;
    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();
    m_tweaker->installGamingRuntimes();
}

void GamingTweaksPage::onOperationStarted(const QString &title)
{
    Q_UNUSED(title);
    m_progressBar->show();
    m_logConsole->show();
}

void GamingTweaksPage::onOperationFinished(bool success, const QString &message)
{
    m_progressBar->hide();
    if (success) {
        QMessageBox::information(this, "Sukses", message);
    } else {
        QMessageBox::critical(this, "Galat", message);
    }
}

void GamingTweaksPage::onLogReceived(const QString &text)
{
    m_logConsole->appendPlainText(text);
}

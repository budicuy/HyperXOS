#include "snapshotpage.h"
#include <QFrame>
#include <QHeaderView>
#include <QMessageBox>

SnapshotPage::SnapshotPage(SnapshotManager *manager, QWidget *parent)
    : QWidget(parent), m_manager(manager)
{
    setupUi();

    if (m_manager) {
        connect(m_manager, &SnapshotManager::snapshotsRefreshed, this, &SnapshotPage::onSnapshotsRefreshed);
        connect(m_manager, &SnapshotManager::operationStarted, this, &SnapshotPage::onOperationStarted);
        connect(m_manager, &SnapshotManager::operationFinished, this, &SnapshotPage::onOperationFinished);
        connect(m_manager, &SnapshotManager::logOutput, this, &SnapshotPage::onLogReceived);
        onSnapshotsRefreshed(m_manager->getSnapshots());
    }
}

SnapshotPage::~SnapshotPage()
{
}

void SnapshotPage::setupUi()
{
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->setContentsMargins(20, 20, 20, 20);
    mainLayout->setSpacing(16);

    // Title & Header
    QVBoxLayout *titleLayout = new QVBoxLayout();
    titleLayout->setSpacing(4);

    QLabel *pageTitle = new QLabel("💾 Snapshot & Pemulihan Sistem", this);
    pageTitle->setStyleSheet("font-size: 20px; font-weight: bold; color: #8aadf4;");

    QLabel *pageSubtitle = new QLabel(
        "Ambil snapshot instan sistem untuk membuat titik pemulihan aman dan lakukan rollback bersih kapan pun.", this);
    pageSubtitle->setStyleSheet("font-size: 12px; color: #a5adcb;");

    titleLayout->addWidget(pageTitle);
    titleLayout->addWidget(pageSubtitle);
    mainLayout->addLayout(titleLayout);

    // Engine Info & Creation Frame
    QFrame *topFrame = new QFrame(this);
    topFrame->setStyleSheet("background-color: #24273a; border: 1px solid #363a4f; border-radius: 10px; padding: 14px;");
    QVBoxLayout *topLayout = new QVBoxLayout(topFrame);
    topLayout->setSpacing(12);

    QHBoxLayout *engineRow = new QHBoxLayout();
    QLabel *engineTitle = new QLabel("Mesin Snapshot Aktif:", topFrame);
    engineTitle->setStyleSheet("font-weight: bold; color: #cad3f5;");

    m_fsEngineLabel = new QLabel("Mendeteksi...", topFrame);
    m_fsEngineLabel->setStyleSheet("background-color: #1e2030; color: #a6da95; font-weight: bold; padding: 3px 8px; border-radius: 4px;");

    if (m_manager) {
        if (m_manager->isBtrfs()) {
            m_fsEngineLabel->setText("Btrfs CoW Subvolume Snapshot (Instan & Efisien ✓)");
        } else if (m_manager->isTimeshiftInstalled()) {
            m_fsEngineLabel->setText("Timeshift System Restore Engine ✓");
        } else {
            m_fsEngineLabel->setText("Rsync Incremental Snapshot System");
        }
    }

    engineRow->addWidget(engineTitle);
    engineRow->addWidget(m_fsEngineLabel);
    engineRow->addStretch();

    m_refreshBtn = new QPushButton("⟳ Segarkan", topFrame);
    m_refreshBtn->setStyleSheet("background-color: #363a4f; color: #cad3f5; border-radius: 6px; padding: 6px 12px; font-weight: bold;");
    connect(m_refreshBtn, &QPushButton::clicked, this, [this]() {
        if (m_manager) m_manager->refreshSnapshots();
    });
    engineRow->addWidget(m_refreshBtn);
    topLayout->addLayout(engineRow);

    // Create Input Row
    QHBoxLayout *inputRow = new QHBoxLayout();
    m_descInput = new QLineEdit(topFrame);
    m_descInput->setPlaceholderText("Ketik deskripsi snapshot (misal: Sebelum instalasi driver GPU baru)...");
    m_descInput->setStyleSheet("background-color: #1e2030; color: #cad3f5; border: 1px solid #363a4f; border-radius: 6px; padding: 8px 12px;");

    m_createBtn = new QPushButton("📸 Buat Snapshot Sekarang", topFrame);
    m_createBtn->setStyleSheet("background-color: #a6da95; color: #181926; font-weight: bold; border-radius: 6px; padding: 8px 16px;");
    connect(m_createBtn, &QPushButton::clicked, this, &SnapshotPage::onCreateSnapshotClicked);

    inputRow->addWidget(m_descInput, 1);
    inputRow->addWidget(m_createBtn);
    topLayout->addLayout(inputRow);

    mainLayout->addWidget(topFrame);

    // Snapshot History Table
    m_tableWidget = new QTableWidget(this);
    m_tableWidget->setColumnCount(5);
    m_tableWidget->setHorizontalHeaderLabels(QStringList() << "ID / Nama" << "Tanggal Pembuatan" << "Keterangan" << "Tipe / Ukuran" << "Aksi Pemulihan");
    m_tableWidget->horizontalHeader()->setSectionResizeMode(0, QHeaderView::ResizeToContents);
    m_tableWidget->horizontalHeader()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    m_tableWidget->horizontalHeader()->setSectionResizeMode(2, QHeaderView::Stretch);
    m_tableWidget->horizontalHeader()->setSectionResizeMode(3, QHeaderView::ResizeToContents);
    m_tableWidget->horizontalHeader()->setSectionResizeMode(4, QHeaderView::ResizeToContents);
    m_tableWidget->setStyleSheet(
        "QTableWidget { background-color: #1e2030; color: #cad3f5; gridline-color: #363a4f; border: 1px solid #363a4f; border-radius: 8px; }"
        "QHeaderView::section { background-color: #24273a; color: #8aadf4; font-weight: bold; border: 1px solid #363a4f; padding: 6px; }"
    );
    mainLayout->addWidget(m_tableWidget, 1);

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
    m_logConsole->setFixedHeight(110);
    m_logConsole->setStyleSheet("background-color: #11121d; color: #cad3f5; font-family: monospace; border: 1px solid #363a4f; border-radius: 6px;");
    m_logConsole->hide();
    mainLayout->addWidget(m_logConsole);
}

void SnapshotPage::onSnapshotsRefreshed(const QList<SnapshotItem> &snapshots)
{
    updateTable(snapshots);
}

void SnapshotPage::updateTable(const QList<SnapshotItem> &snapshots)
{
    m_tableWidget->setRowCount(0);

    for (int row = 0; row < snapshots.size(); ++row) {
        const SnapshotItem &s = snapshots[row];
        m_tableWidget->insertRow(row);

        QTableWidgetItem *idItem = new QTableWidgetItem(s.name);
        idItem->setFlags(idItem->flags() ^ Qt::ItemIsEditable);
        m_tableWidget->setItem(row, 0, idItem);

        QString dateStr = s.createdAt.isValid() ? s.createdAt.toString("dd MMM yyyy, HH:mm") : "-";
        QTableWidgetItem *dateItem = new QTableWidgetItem(dateStr);
        dateItem->setFlags(dateItem->flags() ^ Qt::ItemIsEditable);
        m_tableWidget->setItem(row, 1, dateItem);

        QTableWidgetItem *descItem = new QTableWidgetItem(s.description);
        descItem->setFlags(descItem->flags() ^ Qt::ItemIsEditable);
        m_tableWidget->setItem(row, 2, descItem);

        QTableWidgetItem *sizeItem = new QTableWidgetItem(s.sizeString);
        sizeItem->setFlags(sizeItem->flags() ^ Qt::ItemIsEditable);
        m_tableWidget->setItem(row, 3, sizeItem);

        // Action Buttons Widget
        QWidget *actionWidget = new QWidget(this);
        QHBoxLayout *actionLayout = new QHBoxLayout(actionWidget);
        actionLayout->setContentsMargins(4, 2, 4, 2);
        actionLayout->setSpacing(6);

        QPushButton *restoreBtn = new QPushButton("⟲ Pulihkan", actionWidget);
        restoreBtn->setStyleSheet("background-color: #8aadf4; color: #181926; font-weight: bold; border-radius: 4px; padding: 4px 10px;");
        connect(restoreBtn, &QPushButton::clicked, this, [this, s]() {
            onRestoreSnapshotClicked(s.id);
        });

        QPushButton *deleteBtn = new QPushButton("🗑️ Hapus", actionWidget);
        deleteBtn->setStyleSheet("background-color: #ed8796; color: #181926; font-weight: bold; border-radius: 4px; padding: 4px 10px;");
        connect(deleteBtn, &QPushButton::clicked, this, [this, s]() {
            onDeleteSnapshotClicked(s.id);
        });

        actionLayout->addWidget(restoreBtn);
        actionLayout->addWidget(deleteBtn);
        m_tableWidget->setCellWidget(row, 4, actionWidget);
    }
}

void SnapshotPage::onCreateSnapshotClicked()
{
    if (!m_manager || m_manager->isBusy()) return;
    QString desc = m_descInput->text().trimmed();

    m_logConsole->clear();
    m_logConsole->show();
    m_progressBar->show();
    m_manager->createSnapshot(desc);
    m_descInput->clear();
}

void SnapshotPage::onDeleteSnapshotClicked(const QString &snapshotId)
{
    if (!m_manager || m_manager->isBusy()) return;

    auto reply = QMessageBox::warning(
        this, "Konfirmasi Penghapusan",
        QString("Apakah Anda yakin ingin menghapus permanen snapshot '%1'?").arg(snapshotId),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_logConsole->clear();
        m_logConsole->show();
        m_progressBar->show();
        m_manager->deleteSnapshot(snapshotId);
    }
}

void SnapshotPage::onRestoreSnapshotClicked(const QString &snapshotId)
{
    if (!m_manager || m_manager->isBusy()) return;

    auto reply = QMessageBox::question(
        this, "Konfirmasi Pemulihan Sistem",
        QString("Apakah Anda yakin ingin memulihkan kondisi sistem ke snapshot '%1'?").arg(snapshotId),
        QMessageBox::Yes | QMessageBox::No
    );

    if (reply == QMessageBox::Yes) {
        m_logConsole->clear();
        m_logConsole->show();
        m_progressBar->show();
        m_manager->restoreSnapshot(snapshotId);
    }
}

void SnapshotPage::onOperationStarted(const QString &title)
{
    Q_UNUSED(title);
    m_progressBar->show();
    m_logConsole->show();
}

void SnapshotPage::onOperationFinished(bool success, const QString &message)
{
    m_progressBar->hide();
    if (success) {
        QMessageBox::information(this, "Operasi Berhasil", message);
    } else {
        QMessageBox::critical(this, "Operasi Gagal", message);
    }
}

void SnapshotPage::onLogReceived(const QString &text)
{
    m_logConsole->appendPlainText(text);
}

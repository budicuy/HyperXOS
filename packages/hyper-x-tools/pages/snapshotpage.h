#ifndef SNAPSHOTPAGE_H
#define SNAPSHOTPAGE_H

#include <QWidget>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QTableWidget>
#include <QProgressBar>
#include <QPlainTextEdit>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include "../core/snapshotmanager.h"

class SnapshotPage : public QWidget {
    Q_OBJECT

public:
    explicit SnapshotPage(SnapshotManager *manager, QWidget *parent = nullptr);
    ~SnapshotPage();

private slots:
    void onSnapshotsRefreshed(const QList<SnapshotItem> &snapshots);
    void onCreateSnapshotClicked();
    void onDeleteSnapshotClicked(const QString &snapshotId);
    void onRestoreSnapshotClicked(const QString &snapshotId);

    void onOperationStarted(const QString &title);
    void onOperationFinished(bool success, const QString &message);
    void onLogReceived(const QString &text);

private:
    void setupUi();
    void updateTable(const QList<SnapshotItem> &snapshots);

    SnapshotManager *m_manager;

    QLabel *m_fsEngineLabel;
    QLineEdit *m_descInput;
    QPushButton *m_createBtn;
    QPushButton *m_refreshBtn;
    QTableWidget *m_tableWidget;

    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;
};

#endif // SNAPSHOTPAGE_H

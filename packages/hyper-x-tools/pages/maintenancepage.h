#ifndef MAINTENANCEPAGE_H
#define MAINTENANCEPAGE_H

#include <QWidget>
#include <QLabel>
#include <QPushButton>
#include <QProgressBar>
#include <QPlainTextEdit>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QProcess>

class MaintenancePage : public QWidget {
    Q_OBJECT

public:
    explicit MaintenancePage(QWidget *parent = nullptr);
    ~MaintenancePage();

private slots:
    void onCleanOldCacheClicked();
    void onCleanAllCacheClicked();
    void onVacuumJournalClicked();
    void onRemoveOrphansClicked();
    void onFixKeyringClicked();
    void onRankMirrorsClicked();

    void onProcessReadyRead();
    void onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus);

private:
    void setupUi();
    void runCommand(const QString &title, const QString &cmd, const QStringList &args);

    QProcess *m_process;
    bool m_isBusy = false;

    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;
};

#endif // MAINTENANCEPAGE_H

#ifndef GAMETWEAKSPAGE_H
#define GAMETWEAKSPAGE_H

#include <QWidget>
#include <QLabel>
#include <QPushButton>
#include <QComboBox>
#include <QProgressBar>
#include <QPlainTextEdit>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include "../core/systemtweaker.h"

class GamingTweaksPage : public QWidget {
    Q_OBJECT

public:
    explicit GamingTweaksPage(SystemTweaker *tweaker, QWidget *parent = nullptr);
    ~GamingTweaksPage();

private slots:
    void onStatusRefreshed(const TweakStatus &status);
    void onApplyGovernorClicked();
    void onApplyMapCountClicked();
    void onToggleSplitLockClicked();
    void onToggleGameModeClicked();
    void onInstallGamingRuntimesClicked();

    void onOperationStarted(const QString &title);
    void onOperationFinished(bool success, const QString &message);
    void onLogReceived(const QString &text);

private:
    void setupUi();

    SystemTweaker *m_tweaker;

    // CPU Governor Widgets
    QLabel *m_currentGovLabel;
    QComboBox *m_govComboBox;
    QPushButton *m_applyGovBtn;

    // Memory & Kernel Widgets
    QLabel *m_mapCountStatusLabel;
    QPushButton *m_applyMapCountBtn;
    QLabel *m_splitLockStatusLabel;
    QPushButton *m_toggleSplitLockBtn;
    QLabel *m_gameModeStatusLabel;
    QPushButton *m_toggleGameModeBtn;

    // Runtimes
    QLabel *m_runtimesStatusLabel;
    QPushButton *m_installRuntimesBtn;

    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;
};

#endif // GAMETWEAKSPAGE_H

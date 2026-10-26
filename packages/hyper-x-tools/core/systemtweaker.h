#ifndef SYSTEMTWEAKER_H
#define SYSTEMTWEAKER_H

#include <QObject>
#include <QString>
#include <QStringList>
#include <QProcess>

struct TweakStatus {
    QString currentGovernor;
    QStringList availableGovernors;
    quint64 maxMapCount = 0;
    bool isGamingMapCount = false;
    int splitLockMitigate = -1; // -1 if not supported
    bool isGameModeActive = false;
    bool isGameModeInstalled = false;
    bool isMangoHudInstalled = false;
    bool isSteamInstalled = false;
    bool isLutrisInstalled = false;
    bool isHeroicInstalled = false;
};

class SystemTweaker : public QObject {
    Q_OBJECT

public:
    explicit SystemTweaker(QObject *parent = nullptr);
    ~SystemTweaker();

    TweakStatus getStatus();
    void refreshStatus();

    void setCpuGovernor(const QString &governor);
    void applyGamingMemoryTweak();
    void setSplitLockMitigate(bool enable);
    void toggleGameModeService(bool enable);
    void installGamingRuntimes();
    bool isBusy() const;

signals:
    void statusRefreshed(const TweakStatus &status);
    void operationStarted(const QString &title);
    void operationFinished(bool success, const QString &message);
    void logOutput(const QString &text);

private slots:
    void onProcessReadyRead();
    void onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus);

private:
    void readStatus();

    TweakStatus m_status;
    QProcess *m_process;
    bool m_isBusy = false;
};

#endif // SYSTEMTWEAKER_H

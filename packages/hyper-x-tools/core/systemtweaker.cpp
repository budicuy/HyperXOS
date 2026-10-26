#include "systemtweaker.h"
#include <QFile>
#include <QTextStream>
#include <QDir>

SystemTweaker::SystemTweaker(QObject *parent)
    : QObject(parent), m_process(new QProcess(this))
{
    connect(m_process, &QProcess::readyReadStandardOutput, this, &SystemTweaker::onProcessReadyRead);
    connect(m_process, &QProcess::readyReadStandardError, this, &SystemTweaker::onProcessReadyRead);
    connect(m_process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &SystemTweaker::onProcessFinished);

    readStatus();
}

SystemTweaker::~SystemTweaker()
{
    if (m_process->state() != QProcess::NotRunning) {
        m_process->kill();
        m_process->waitForFinished(1000);
    }
}

TweakStatus SystemTweaker::getStatus()
{
    readStatus();
    return m_status;
}

void SystemTweaker::refreshStatus()
{
    readStatus();
    emit statusRefreshed(m_status);
}

void SystemTweaker::readStatus()
{
    // 1. Current Governor
    QFile govFile("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor");
    if (govFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        m_status.currentGovernor = QTextStream(&govFile).readLine().trimmed();
        govFile.close();
    } else {
        m_status.currentGovernor = "Unknown";
    }

    // 2. Available Governors
    QFile availGovFile("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors");
    if (availGovFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        QString line = QTextStream(&availGovFile).readLine().trimmed();
        m_status.availableGovernors = line.split(' ', Qt::SkipEmptyParts);
        availGovFile.close();
    }
    if (m_status.availableGovernors.isEmpty()) {
        m_status.availableGovernors << "performance" << "schedutil" << "powersave";
    }

    // 3. vm.max_map_count
    QFile mapFile("/proc/sys/vm/max_map_count");
    if (mapFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        m_status.maxMapCount = QTextStream(&mapFile).readLine().trimmed().toULongLong();
        m_status.isGamingMapCount = (m_status.maxMapCount >= 2147483642ULL);
        mapFile.close();
    }

    // 4. Split lock mitigate
    QFile splitLockFile("/proc/sys/kernel/split_lock_mitigate");
    if (splitLockFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        m_status.splitLockMitigate = QTextStream(&splitLockFile).readLine().trimmed().toInt();
        splitLockFile.close();
    } else {
        m_status.splitLockMitigate = -1;
    }

    // 5. Packages check
    QDir localDir("/var/lib/pacman/local");
    m_status.isGameModeInstalled = !localDir.entryList(QStringList() << "gamemode-[0-9]*", QDir::Dirs).isEmpty();
    m_status.isMangoHudInstalled = !localDir.entryList(QStringList() << "mangohud-[0-9]*", QDir::Dirs).isEmpty();
    m_status.isSteamInstalled = !localDir.entryList(QStringList() << "steam-[0-9]*", QDir::Dirs).isEmpty();
    m_status.isLutrisInstalled = !localDir.entryList(QStringList() << "lutris-[0-9]*", QDir::Dirs).isEmpty();
    m_status.isHeroicInstalled = !localDir.entryList(QStringList() << "heroic-games-launcher-bin-[0-9]*" << "heroic-[0-9]*", QDir::Dirs).isEmpty();

    // 6. GameMode active status
    if (m_status.isGameModeInstalled) {
        QProcess checkProc;
        checkProc.start("systemctl", QStringList() << "--user" << "is-active" << "gamemoded");
        if (checkProc.waitForFinished(300)) {
            QString out = checkProc.readAllStandardOutput().trimmed();
            m_status.isGameModeActive = (out == "active");
        }
    }
}

void SystemTweaker::setCpuGovernor(const QString &governor)
{
    if (m_isBusy) return;

    m_isBusy = true;
    emit operationStarted(QString("Mengubah CPU Governor ke %1...").arg(governor));
    emit logOutput(QString("==> Menerapkan CPU Governor: %1\n").arg(governor));

    QString script = QString("for g in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do echo '%1' > \"$g\"; done").arg(governor);
    m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
}

void SystemTweaker::applyGamingMemoryTweak()
{
    if (m_isBusy) return;

    m_isBusy = true;
    emit operationStarted("Menerapkan konfigurasi kernel gaming (vm.max_map_count)...");
    emit logOutput("==> Mengatur vm.max_map_count = 2147483642 untuk stabilitas game modern...\n");

    QString script = "sysctl -w vm.max_map_count=2147483642 && echo 'vm.max_map_count=2147483642' > /etc/sysctl.d/99-gaming-memory.conf";
    m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
}

void SystemTweaker::setSplitLockMitigate(bool enable)
{
    if (m_isBusy) return;

    m_isBusy = true;
    int val = enable ? 1 : 0;
    emit operationStarted(enable ? "Mengaktifkan Split Lock Mitigate..." : "Menonaktifkan Split Lock Mitigate (Anti-stuttering FPS)...");
    emit logOutput(QString("==> Mengatur kernel.split_lock_mitigate = %1\n").arg(val));

    QString script = QString("sysctl -w kernel.split_lock_mitigate=%1 && echo 'kernel.split_lock_mitigate=%1' > /etc/sysctl.d/99-split-lock.conf").arg(val);
    m_process->start("pkexec", QStringList() << "bash" << "-c" << script);
}

void SystemTweaker::toggleGameModeService(bool enable)
{
    if (enable) {
        QProcess::startDetached("systemctl", QStringList() << "--user" << "enable" << "--now" << "gamemoded");
        emit logOutput("==> GameMode daemon diaktifkan (systemctl --user enable --now gamemoded).\n");
    } else {
        QProcess::startDetached("systemctl", QStringList() << "--user" << "disable" << "--now" << "gamemoded");
        emit logOutput("==> GameMode daemon dinonaktifkan.\n");
    }
    refreshStatus();
}

void SystemTweaker::installGamingRuntimes()
{
    if (m_isBusy) return;

    m_isBusy = true;
    emit operationStarted("Memasang gaming runtimes & toolchain esensial...");
    emit logOutput("==> Memasang Steam, Lutris, GameMode, MangoHud, dan VKD3D...\n");

    QStringList packages;
    packages << "steam" << "lutris" << "gamemode" << "mangohud" << "lib32-mangohud" << "vkd3d" << "lib32-vkd3d";

    QStringList args;
    args << "pacman" << "-S" << "--noconfirm" << "--needed";
    args.append(packages);

    m_process->start("pkexec", args);
}

bool SystemTweaker::isBusy() const
{
    return m_isBusy;
}

void SystemTweaker::onProcessReadyRead()
{
    QByteArray out = m_process->readAllStandardOutput();
    QByteArray err = m_process->readAllStandardError();
    if (!out.isEmpty()) emit logOutput(QString::fromUtf8(out));
    if (!err.isEmpty()) emit logOutput(QString::fromUtf8(err));
}

void SystemTweaker::onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus)
{
    m_isBusy = false;
    refreshStatus();

    if (exitStatus == QProcess::NormalExit && exitCode == 0) {
        emit operationFinished(true, "Konfigurasi performa berhasil diterapkan.");
    } else {
        emit operationFinished(false, QString("Konfigurasi gagal diterapkan (kode keluar %1).").arg(exitCode));
    }
}

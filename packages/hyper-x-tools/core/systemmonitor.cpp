#include "systemmonitor.h"
#include <QFile>
#include <QTextStream>
#include <QDir>
#include <QProcess>
#include <sys/statvfs.h>
#include <sys/utsname.h>
#include <unistd.h>

SystemMonitor::SystemMonitor(QObject *parent)
    : QObject(parent), m_timer(new QTimer(this))
{
    connect(m_timer, &QTimer::timeout, this, &SystemMonitor::onTimerTick);
    readSystemInfo();
    onTimerTick(); // Initial read
}

SystemMonitor::~SystemMonitor()
{
    stopMonitoring();
}

void SystemMonitor::startMonitoring(int intervalMs)
{
    if (!m_timer->isActive()) {
        m_timer->start(intervalMs);
    }
}

void SystemMonitor::stopMonitoring()
{
    if (m_timer->isActive()) {
        m_timer->stop();
    }
}

SystemMetrics SystemMonitor::getLatestMetrics() const
{
    return m_metrics;
}

void SystemMonitor::onTimerTick()
{
    readCpuStats();
    readMemInfo();
    readDiskStats();
    readGpuStats();
    readSystemInfo(); // updates uptime

    emit metricsUpdated(m_metrics);
}

void SystemMonitor::readCpuStats()
{
    QFile file("/proc/stat");
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return;
    }

    QTextStream in(&file);
    QString line = in.readLine();
    file.close();

    if (!line.startsWith("cpu ")) {
        return;
    }

    QStringList parts = line.split(' ', Qt::SkipEmptyParts);
    if (parts.size() < 8) {
        return;
    }

    unsigned long long user = parts[1].toULongLong();
    unsigned long long nice = parts[2].toULongLong();
    unsigned long long system = parts[3].toULongLong();
    unsigned long long idle = parts[4].toULongLong();
    unsigned long long iowait = parts[5].toULongLong();
    unsigned long long irq = parts[6].toULongLong();
    unsigned long long softirq = parts[7].toULongLong();
    unsigned long long steal = (parts.size() > 8) ? parts[8].toULongLong() : 0;

    if (!m_firstCpuRead) {
        unsigned long long prevIdleTime = m_prevIdle + m_prevIowait;
        unsigned long long idleTime = idle + iowait;

        unsigned long long prevNonIdle = m_prevUser + m_prevNice + m_prevSystem + m_prevIrq + m_prevSoftirq + m_prevSteal;
        unsigned long long nonIdle = user + nice + system + irq + softirq + steal;

        unsigned long long prevTotal = prevIdleTime + prevNonIdle;
        unsigned long long total = idleTime + nonIdle;

        unsigned long long totald = total - prevTotal;
        unsigned long long idled = idleTime - prevIdleTime;

        if (totald > 0) {
            double usage = (double)(totald - idled) / (double)totald * 100.0;
            if (usage < 0.0) usage = 0.0;
            if (usage > 100.0) usage = 100.0;
            m_metrics.cpuUsagePercent = usage;
        }
    } else {
        m_firstCpuRead = false;
    }

    m_prevUser = user;
    m_prevNice = nice;
    m_prevSystem = system;
    m_prevIdle = idle;
    m_prevIowait = iowait;
    m_prevIrq = irq;
    m_prevSoftirq = softirq;
    m_prevSteal = steal;

    // Read CPU Temperature
    int temp = 0;
    // Check thermal zone
    for (int i = 0; i < 5; ++i) {
        QFile tfile(QString("/sys/class/thermal/thermal_zone%1/temp").arg(i));
        if (tfile.open(QIODevice::ReadOnly | QIODevice::Text)) {
            QString val = QTextStream(&tfile).readLine().trimmed();
            int t = val.toInt();
            if (t > 1000) t /= 1000;
            if (t > temp) temp = t;
            tfile.close();
        }
    }
    // Check hwmon
    if (temp == 0) {
        QDir hwDir("/sys/class/hwmon");
        QStringList entries = hwDir.entryList(QDir::Dirs | QDir::NoDotAndDotDot);
        for (const QString &entry : entries) {
            QFile hfile(QString("/sys/class/hwmon/%1/temp1_input").arg(entry));
            if (hfile.open(QIODevice::ReadOnly | QIODevice::Text)) {
                QString val = QTextStream(&hfile).readLine().trimmed();
                int t = val.toInt();
                if (t > 1000) t /= 1000;
                if (t > temp) temp = t;
                hfile.close();
            }
        }
    }
    m_metrics.cpuTempCelsius = temp;

    // Read CPU Model & Cores & Frequency if not set
    if (m_metrics.cpuModel.isEmpty()) {
        QFile cinfo("/proc/cpuinfo");
        if (cinfo.open(QIODevice::ReadOnly | QIODevice::Text)) {
            QTextStream cin(&cinfo);
            int cores = 0;
            while (!cin.atEnd()) {
                QString cline = cin.readLine();
                if (cline.startsWith("model name") && m_metrics.cpuModel.isEmpty()) {
                    QStringList split = cline.split(':');
                    if (split.size() > 1) {
                        m_metrics.cpuModel = split[1].trimmed();
                    }
                }
                if (cline.startsWith("processor")) {
                    cores++;
                }
                if (cline.startsWith("cpu MHz") && m_metrics.cpuFreqMhz == 0) {
                    QStringList split = cline.split(':');
                    if (split.size() > 1) {
                        m_metrics.cpuFreqMhz = static_cast<int>(split[1].trimmed().toDouble());
                    }
                }
            }
            m_metrics.cpuCores = cores;
            cinfo.close();
        }
    }
}

void SystemMonitor::readMemInfo()
{
    QFile file("/proc/meminfo");
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return;
    }

    QTextStream in(&file);
    quint64 totalKb = 0;
    quint64 availKb = 0;
    quint64 swapTotalKb = 0;
    quint64 swapFreeKb = 0;

    while (!in.atEnd()) {
        QString line = in.readLine();
        if (line.startsWith("MemTotal:")) {
            totalKb = line.split(' ', Qt::SkipEmptyParts).value(1).toULongLong();
        } else if (line.startsWith("MemAvailable:")) {
            availKb = line.split(' ', Qt::SkipEmptyParts).value(1).toULongLong();
        } else if (line.startsWith("SwapTotal:")) {
            swapTotalKb = line.split(' ', Qt::SkipEmptyParts).value(1).toULongLong();
        } else if (line.startsWith("SwapFree:")) {
            swapFreeKb = line.split(' ', Qt::SkipEmptyParts).value(1).toULongLong();
        }
    }
    file.close();

    m_metrics.ramTotalBytes = totalKb * 1024;
    quint64 usedKb = (totalKb > availKb) ? (totalKb - availKb) : 0;
    m_metrics.ramUsedBytes = usedKb * 1024;
    m_metrics.ramFreeBytes = availKb * 1024;

    if (totalKb > 0) {
        m_metrics.ramUsagePercent = (double)usedKb / (double)totalKb * 100.0;
    }

    m_metrics.swapTotalBytes = swapTotalKb * 1024;
    quint64 swapUsedKb = (swapTotalKb > swapFreeKb) ? (swapTotalKb - swapFreeKb) : 0;
    m_metrics.swapUsedBytes = swapUsedKb * 1024;

    if (swapTotalKb > 0) {
        m_metrics.swapUsagePercent = (double)swapUsedKb / (double)swapTotalKb * 100.0;
    } else {
        m_metrics.swapUsagePercent = 0.0;
    }
}

void SystemMonitor::readDiskStats()
{
    struct statvfs stat;
    if (statvfs("/", &stat) == 0) {
        quint64 total = (quint64)stat.f_blocks * stat.f_frsize;
        quint64 free = (quint64)stat.f_bavail * stat.f_frsize;
        quint64 used = (total > free) ? (total - free) : 0;

        m_metrics.diskTotalBytes = total;
        m_metrics.diskUsedBytes = used;
        m_metrics.diskFreeBytes = free;

        if (total > 0) {
            m_metrics.diskUsagePercent = (double)used / (double)total * 100.0;
        }
    }

    if (m_metrics.diskFsType.isEmpty()) {
        QFile mounts("/proc/mounts");
        if (mounts.open(QIODevice::ReadOnly | QIODevice::Text)) {
            QTextStream min(&mounts);
            while (!min.atEnd()) {
                QString mline = min.readLine();
                QStringList mparts = mline.split(' ', Qt::SkipEmptyParts);
                if (mparts.size() >= 3 && mparts[1] == "/") {
                    m_metrics.diskFsType = mparts[2].toUpper();
                    break;
                }
            }
            mounts.close();
        }
    }
}

void SystemMonitor::readGpuStats()
{
    // 1. AMD GPU check
    QFile amdGpu("/sys/class/drm/card0/device/gpu_busy_percent");
    if (amdGpu.open(QIODevice::ReadOnly | QIODevice::Text)) {
        QString val = QTextStream(&amdGpu).readLine().trimmed();
        m_metrics.gpuUsagePercent = val.toDouble();
        amdGpu.close();

        // Read VRAM
        QFile vramUsed("/sys/class/drm/card0/device/mem_info_vram_used");
        if (vramUsed.open(QIODevice::ReadOnly | QIODevice::Text)) {
            m_metrics.vramUsedBytes = QTextStream(&vramUsed).readLine().trimmed().toULongLong();
            vramUsed.close();
        }
        QFile vramTotal("/sys/class/drm/card0/device/mem_info_vram_total");
        if (vramTotal.open(QIODevice::ReadOnly | QIODevice::Text)) {
            m_metrics.vramTotalBytes = QTextStream(&vramTotal).readLine().trimmed().toULongLong();
            vramTotal.close();
        }
        if (m_metrics.gpuModel.isEmpty()) {
            m_metrics.gpuModel = "AMD Radeon Graphics";
        }
        return;
    }

    // 2. NVIDIA GPU check
    if (QFile::exists("/usr/bin/nvidia-smi")) {
        QProcess proc;
        proc.start("nvidia-smi", QStringList() << "--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu"
                                               << "--format=csv,noheader,nounits");
        if (proc.waitForFinished(500)) {
            QString out = proc.readAllStandardOutput().trimmed();
            QStringList parts = out.split(',');
            if (parts.size() >= 5) {
                m_metrics.gpuModel = parts[0].trimmed();
                m_metrics.gpuUsagePercent = parts[1].trimmed().toDouble();
                m_metrics.vramUsedBytes = parts[2].trimmed().toULongLong() * 1024 * 1024;
                m_metrics.vramTotalBytes = parts[3].trimmed().toULongLong() * 1024 * 1024;
                m_metrics.gpuTempCelsius = parts[4].trimmed().toInt();
                return;
            }
        }
    }

    // 3. Intel GPU check
    QFile intelGpu("/sys/class/drm/card0/gt_act_freq_mhz");
    if (intelGpu.open(QIODevice::ReadOnly | QIODevice::Text)) {
        intelGpu.close();
        if (m_metrics.gpuModel.isEmpty()) {
            m_metrics.gpuModel = "Intel Graphics (Integrated)";
        }
        m_metrics.gpuUsagePercent = -1.0; // Intel sysfs doesn't expose standard busy % easily
        return;
    }

    if (m_metrics.gpuModel.isEmpty()) {
        m_metrics.gpuModel = "Universal Mesa / DRM Driver";
    }
}

void SystemMonitor::readSystemInfo()
{
    // Kernel version
    if (m_metrics.kernelVersion.isEmpty()) {
        struct utsname uts;
        if (uname(&uts) == 0) {
            m_metrics.kernelVersion = QString::fromLatin1(uts.release);
        }
    }

    // OS Name
    if (m_metrics.osName.isEmpty()) {
        m_metrics.osName = "HyperXOS Linux";
    }

    // Display session
    if (m_metrics.displaySession.isEmpty()) {
        QString wayland = qgetenv("WAYLAND_DISPLAY");
        if (!wayland.isEmpty()) {
            m_metrics.displaySession = "Wayland";
        } else {
            m_metrics.displaySession = "X11";
        }
    }

    // Uptime
    QFile uptimeFile("/proc/uptime");
    if (uptimeFile.open(QIODevice::ReadOnly | QIODevice::Text)) {
        QString line = QTextStream(&uptimeFile).readLine();
        double uptimeSec = line.split(' ').value(0).toDouble();
        int hours = static_cast<int>(uptimeSec / 3600);
        int minutes = static_cast<int>((uptimeSec - (hours * 3600)) / 60);

        if (hours > 0) {
            m_metrics.uptimeString = QString("%1 jam %2 menit").arg(hours).arg(minutes);
        } else {
            m_metrics.uptimeString = QString("%1 menit").arg(minutes);
        }
        uptimeFile.close();
    }
}

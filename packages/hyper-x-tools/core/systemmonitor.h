#ifndef SYSTEMMONITOR_H
#define SYSTEMMONITOR_H

#include <QObject>
#include <QString>
#include <QTimer>

struct SystemMetrics {
    // CPU
    double cpuUsagePercent = 0.0;
    int cpuTempCelsius = 0;
    QString cpuModel;
    int cpuCores = 0;
    int cpuFreqMhz = 0;

    // RAM
    quint64 ramTotalBytes = 0;
    quint64 ramUsedBytes = 0;
    quint64 ramFreeBytes = 0;
    double ramUsagePercent = 0.0;

    // Swap
    quint64 swapTotalBytes = 0;
    quint64 swapUsedBytes = 0;
    double swapUsagePercent = 0.0;

    // Disk (Root /)
    quint64 diskTotalBytes = 0;
    quint64 diskUsedBytes = 0;
    quint64 diskFreeBytes = 0;
    double diskUsagePercent = 0.0;
    QString diskFsType;

    // GPU
    QString gpuModel;
    double gpuUsagePercent = -1.0;
    int gpuTempCelsius = 0;
    quint64 vramTotalBytes = 0;
    quint64 vramUsedBytes = 0;

    // System info
    QString kernelVersion;
    QString osName;
    QString uptimeString;
    QString displaySession;
};

class SystemMonitor : public QObject {
    Q_OBJECT

public:
    explicit SystemMonitor(QObject *parent = nullptr);
    ~SystemMonitor();

    void startMonitoring(int intervalMs = 1500);
    void stopMonitoring();
    SystemMetrics getLatestMetrics() const;

signals:
    void metricsUpdated(const SystemMetrics &metrics);

private slots:
    void onTimerTick();

private:
    void readCpuStats();
    void readMemInfo();
    void readDiskStats();
    void readGpuStats();
    void readSystemInfo();

    QTimer *m_timer;
    SystemMetrics m_metrics;

    // CPU calculation state
    unsigned long long m_prevUser = 0;
    unsigned long long m_prevNice = 0;
    unsigned long long m_prevSystem = 0;
    unsigned long long m_prevIdle = 0;
    unsigned long long m_prevIowait = 0;
    unsigned long long m_prevIrq = 0;
    unsigned long long m_prevSoftirq = 0;
    unsigned long long m_prevSteal = 0;
    bool m_firstCpuRead = true;
};

#endif // SYSTEMMONITOR_H

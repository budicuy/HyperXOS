#ifndef DASHBOARDPAGE_H
#define DASHBOARDPAGE_H

#include <QWidget>
#include <QLabel>
#include <QProgressBar>
#include <QGridLayout>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFrame>
#include <QPushButton>
#include "../core/systemmonitor.h"

class DashboardPage : public QWidget {
    Q_OBJECT

public:
    explicit DashboardPage(SystemMonitor *monitor, QWidget *parent = nullptr);
    ~DashboardPage();

public slots:
    void onMetricsUpdated(const SystemMetrics &metrics);

signals:
    void navigateToPage(int pageIndex);

private:
    void setupUi();
    QFrame* createMetricCard(const QString &title, const QString &iconText,
                             QLabel **valueLabel, QProgressBar **bar,
                             QLabel **subTextLabel, QLabel **extraLabel);

    SystemMonitor *m_monitor;

    // CPU Card
    QLabel *m_cpuValueLabel;
    QProgressBar *m_cpuBar;
    QLabel *m_cpuSubLabel;
    QLabel *m_cpuExtraLabel;

    // RAM Card
    QLabel *m_ramValueLabel;
    QProgressBar *m_ramBar;
    QLabel *m_ramSubLabel;
    QLabel *m_ramExtraLabel;

    // Disk Card
    QLabel *m_diskValueLabel;
    QProgressBar *m_diskBar;
    QLabel *m_diskSubLabel;
    QLabel *m_diskExtraLabel;

    // GPU Card
    QLabel *m_gpuValueLabel;
    QProgressBar *m_gpuBar;
    QLabel *m_gpuSubLabel;
    QLabel *m_gpuExtraLabel;

    // System Info Card
    QLabel *m_kernelLabel;
    QLabel *m_osLabel;
    QLabel *m_uptimeLabel;
    QLabel *m_sessionLabel;
};

#endif // DASHBOARDPAGE_H

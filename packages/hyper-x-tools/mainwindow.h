#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QListWidget>
#include <QStackedWidget>
#include <QLabel>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFrame>

// Core Engines
#include "core/systemmonitor.h"
#include "core/kernelmanager.h"
#include "core/systemtweaker.h"
#include "core/snapshotmanager.h"
#include "core/packagemanager.h"

// Pages
#include "pages/dashboardpage.h"
#include "pages/gametweakspage.h"
#include "pages/kernelmanagerpage.h"
#include "pages/snapshotpage.h"
#include "pages/maintenancepage.h"
#include "pages/packagestorepage.h"

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

public slots:
    void navigateToPage(int index);

private slots:
    void onSidebarItemClicked(QListWidgetItem *item);
    void onMetricsUpdated(const SystemMetrics &metrics);

private:
    void setupUi();
    void applyStyleSheet();
    void setupSidebar();

    // Core
    SystemMonitor *m_monitor;
    KernelManager *m_kernelManager;
    SystemTweaker *m_tweaker;
    SnapshotManager *m_snapshotManager;
    PackageManager *m_packageManager;

    // UI Structure
    QListWidget *m_sidebar;
    QStackedWidget *m_stackedWidget;

    // Header Widgets
    QLabel *m_logoLabel;
    QLabel *m_titleLabel;
    QLabel *m_quickStatsLabel;

    // Pages
    DashboardPage *m_dashboardPage;
    GamingTweaksPage *m_gamingPage;
    KernelManagerPage *m_kernelPage;
    SnapshotPage *m_snapshotPage;
    MaintenancePage *m_maintenancePage;
    PackageStorePage *m_storePage;
};

#endif // MAINWINDOW_H

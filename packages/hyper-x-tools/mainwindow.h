#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QListWidget>
#include <QScrollArea>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QCheckBox>
#include <QPlainTextEdit>
#include <QProgressBar>
#include <QMap>
#include "packagemanager.h"

class MainWindow : public QMainWindow {
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

private slots:
    void onCategorySelected(int index);
    void onRefreshClicked();
    void onInstallSelectedClicked();
    void onUninstallSelectedClicked();
    void onToggleLogClicked();
    void onPackageCheckboxChanged(const QString &packageId, bool checked);
    void onSingleInstallClicked(const QString &packageId);
    void onSingleUninstallClicked(const QString &packageId);

    void onLogReceived(const QString &text);
    void onProcessStarted(const QString &title);
    void onProcessFinished(bool success, const QString &message);
    void onCacheRefreshed();

private:
    void setupUi();
    void applyStyleSheet();
    void populateCategories();
    void renderPackageList(const QString &category);
    void updateStatsLabel();

    PackageManager *m_packageManager;
    QString m_currentCategory;

    // UI Widgets
    QLabel *m_logoLabel;
    QLabel *m_titleLabel;
    QLabel *m_statsLabel;
    QListWidget *m_categoryList;
    QWidget *m_packageListContainer;
    QVBoxLayout *m_packageListLayout;
    QScrollArea *m_scrollArea;

    QPushButton *m_installButton;
    QPushButton *m_uninstallButton;
    QPushButton *m_refreshButton;
    QPushButton *m_toggleLogButton;
    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;

    // State tracking
    QMap<QString, QCheckBox*> m_checkboxMap;
    QSet<QString> m_selectedToInstall;
    QSet<QString> m_selectedToUninstall;
};

#endif // MAINWINDOW_H

#ifndef PACKAGESTOREPAGE_H
#define PACKAGESTOREPAGE_H

#include <QWidget>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QComboBox>
#include <QScrollArea>
#include <QProgressBar>
#include <QPlainTextEdit>
#include <QCheckBox>
#include <QMap>
#include <QSet>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include "../core/packagemanager.h"

class PackageStorePage : public QWidget {
    Q_OBJECT

public:
    explicit PackageStorePage(PackageManager *manager, QWidget *parent = nullptr);
    ~PackageStorePage();

private slots:
    void onSearchTextChanged(const QString &text);
    void onCategoryChanged(int index);
    void onRefreshClicked();
    void onInstallSelectedClicked();
    void onUninstallSelectedClicked();
    void onPackageCheckboxChanged(const QString &packageId, bool checked);
    void onSingleInstallClicked(const QString &packageId);
    void onSingleUninstallClicked(const QString &packageId);

    void onLogReceived(const QString &text);
    void onProcessStarted(const QString &title);
    void onProcessFinished(bool success, const QString &message);
    void onCacheRefreshed();

private:
    void setupUi();
    void renderPackageCards();
    void updateActionButtons();

    PackageManager *m_manager;
    QString m_currentCategory;
    QString m_searchQuery;

    QLineEdit *m_searchInput;
    QComboBox *m_categoryCombo;
    QLabel *m_statusSummaryLabel;

    QWidget *m_cardsContainer;
    QVBoxLayout *m_cardsLayout;

    QPushButton *m_installSelectedBtn;
    QPushButton *m_uninstallSelectedBtn;
    QPushButton *m_refreshBtn;
    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;

    QMap<QString, QCheckBox*> m_checkboxMap;
    QSet<QString> m_selectedToInstall;
    QSet<QString> m_selectedToUninstall;
};

#endif // PACKAGESTOREPAGE_H

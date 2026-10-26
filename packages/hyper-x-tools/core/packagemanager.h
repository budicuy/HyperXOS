#ifndef PACKAGEMANAGER_H
#define PACKAGEMANAGER_H

#include <QObject>
#include <QString>
#include <QStringList>
#include <QList>
#include <QSet>
#include <QProcess>

struct PackageItem {
    QString id;
    QString name;
    QString description;
    QString category;
    QStringList packages;
    bool isInstalled;
};

class PackageManager : public QObject {
    Q_OBJECT

public:
    explicit PackageManager(QObject *parent = nullptr);
    ~PackageManager();

    void refreshInstalledCache();
    bool isPackageInstalled(const QStringList &packages) const;

    QList<PackageItem> getAllPackages() const;
    QList<PackageItem> getPackagesByCategory(const QString &category) const;
    QList<PackageItem> searchPackages(const QString &query, const QString &category = QString()) const;
    QStringList getCategories() const;
    QString getCategoryDisplayName(const QString &category) const;

    void executeInstall(const QStringList &packagesToInstall);
    void executeUninstall(const QStringList &packagesToUninstall);
    bool isBusy() const;

signals:
    void logReceived(const QString &text);
    void processStarted(const QString &title);
    void processFinished(bool success, const QString &message);
    void cacheRefreshed();

private slots:
    void onReadyReadStandardOutput();
    void onReadyReadStandardError();
    void onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus);

private:
    void initPackageDefinitions();

    QList<PackageItem> m_packageDefinitions;
    QSet<QString> m_installedPackagesCache;
    QProcess *m_process;
    bool m_isBusy;
    QString m_currentOperation;
};

#endif // PACKAGEMANAGER_H

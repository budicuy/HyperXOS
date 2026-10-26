#ifndef KERNELMANAGER_H
#define KERNELMANAGER_H

#include <QObject>
#include <QString>
#include <QStringList>
#include <QList>
#include <QProcess>

struct KernelItem {
    QString id;
    QString name;
    QString description;
    QString package;
    QString headersPackage;
    QString version;
    bool isInstalled = false;
    bool isRunning = false;
    QString category;
};

class KernelManager : public QObject {
    Q_OBJECT

public:
    explicit KernelManager(QObject *parent = nullptr);
    ~KernelManager();

    QString getRunningKernel() const;
    QList<KernelItem> getKernels() const;
    void refreshKernels();

    void installKernel(const QString &package);
    void removeKernel(const QString &package);
    bool isBusy() const;

signals:
    void kernelsRefreshed();
    void operationStarted(const QString &action);
    void operationFinished(bool success, const QString &message);
    void logOutput(const QString &text);

private slots:
    void onProcessReadyRead();
    void onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus);

private:
    void initKernelCatalog();
    bool checkPackageInstalled(const QString &pkgName);

    QList<KernelItem> m_kernels;
    QString m_runningKernel;
    QProcess *m_process;
    bool m_isBusy = false;
};

#endif // KERNELMANAGER_H

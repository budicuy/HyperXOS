#ifndef SNAPSHOTMANAGER_H
#define SNAPSHOTMANAGER_H

#include <QObject>
#include <QString>
#include <QList>
#include <QDateTime>
#include <QProcess>

struct SnapshotItem {
    QString id;
    QString name;
    QString description;
    QDateTime createdAt;
    QString path;
    QString sizeString;
    bool isBtrfs = false;
};

class SnapshotManager : public QObject {
    Q_OBJECT

public:
    explicit SnapshotManager(QObject *parent = nullptr);
    ~SnapshotManager();

    bool isBtrfs() const;
    bool isTimeshiftInstalled() const;
    QList<SnapshotItem> getSnapshots() const;
    void refreshSnapshots();

    void createSnapshot(const QString &description);
    void deleteSnapshot(const QString &snapshotId);
    void restoreSnapshot(const QString &snapshotId);
    bool isBusy() const;

signals:
    void snapshotsRefreshed(const QList<SnapshotItem> &snapshots);
    void operationStarted(const QString &title);
    void operationFinished(bool success, const QString &message);
    void logOutput(const QString &text);

private slots:
    void onProcessReadyRead();
    void onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus);

private:
    void detectFilesystem();
    void scanBtrfsSnapshots();
    void scanTimeshiftSnapshots();

    bool m_isBtrfs = false;
    bool m_isTimeshiftInstalled = false;
    QList<SnapshotItem> m_snapshots;
    QProcess *m_process;
    bool m_isBusy = false;
};

#endif // SNAPSHOTMANAGER_H

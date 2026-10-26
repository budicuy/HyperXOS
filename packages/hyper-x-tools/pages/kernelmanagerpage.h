#ifndef KERNELMANAGERPAGE_H
#define KERNELMANAGERPAGE_H

#include <QWidget>
#include <QLabel>
#include <QPushButton>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QScrollArea>
#include <QPlainTextEdit>
#include <QProgressBar>
#include "../core/kernelmanager.h"

class KernelManagerPage : public QWidget {
    Q_OBJECT

public:
    explicit KernelManagerPage(KernelManager *manager, QWidget *parent = nullptr);
    ~KernelManagerPage();

private slots:
    void onKernelsRefreshed();
    void onInstallClicked(const QString &package);
    void onRemoveClicked(const QString &package);
    void onRebuildInitramfsClicked();

    void onOperationStarted(const QString &action);
    void onOperationFinished(bool success, const QString &message);
    void onLogReceived(const QString &text);

private:
    void setupUi();
    void renderKernelList();

    KernelManager *m_manager;
    QLabel *m_activeKernelLabel;
    QWidget *m_kernelCardsContainer;
    QVBoxLayout *m_kernelCardsLayout;
    QProgressBar *m_progressBar;
    QPlainTextEdit *m_logConsole;
};

#endif // KERNELMANAGERPAGE_H

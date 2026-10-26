#include <QApplication>
#include "mainwindow.h"

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    app.setApplicationName("HyperXTools");
    app.setApplicationDisplayName("HyperXTools");
    app.setOrganizationName("HyperXOS");
    app.setOrganizationDomain("hyperxos.org");
    app.setApplicationVersion("2.0.0");

    MainWindow window;
    window.show();

    return app.exec();
}

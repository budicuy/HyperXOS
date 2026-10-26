QT += core gui widgets
CONFIG += c++17 release
TARGET = hyper-x-tools
TEMPLATE = app

SOURCES += main.cpp \
           mainwindow.cpp \
           core/systemmonitor.cpp \
           core/kernelmanager.cpp \
           core/systemtweaker.cpp \
           core/snapshotmanager.cpp \
           core/packagemanager.cpp \
           pages/dashboardpage.cpp \
           pages/gametweakspage.cpp \
           pages/kernelmanagerpage.cpp \
           pages/snapshotpage.cpp \
           pages/maintenancepage.cpp \
           pages/packagestorepage.cpp

HEADERS += mainwindow.h \
           core/systemmonitor.h \
           core/kernelmanager.h \
           core/systemtweaker.h \
           core/snapshotmanager.h \
           core/packagemanager.h \
           pages/dashboardpage.h \
           pages/gametweakspage.h \
           pages/kernelmanagerpage.h \
           pages/snapshotpage.h \
           pages/maintenancepage.h \
           pages/packagestorepage.h

RESOURCES += resources.qrc

target.path = /usr/bin
INSTALLS += target

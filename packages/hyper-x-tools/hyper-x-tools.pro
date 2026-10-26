QT += core gui widgets
CONFIG += c++17 release
TARGET = hyper-x-tools
TEMPLATE = app

SOURCES += main.cpp \
           mainwindow.cpp \
           packagemanager.cpp

HEADERS += mainwindow.h \
           packagemanager.h

RESOURCES += resources.qrc

target.path = /usr/bin
INSTALLS += target

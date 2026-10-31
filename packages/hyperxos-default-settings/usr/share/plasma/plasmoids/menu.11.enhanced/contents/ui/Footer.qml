import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtCore

import org.kde.plasma.plasmoid
import org.kde.plasma.core as PlasmaCore
import org.kde.plasma.extras as PlasmaExtras

import org.kde.plasma.private.kicker 0.1 as Kicker
import org.kde.coreaddons 1.0 as KCoreAddons
import org.kde.plasma.private.shell

import org.kde.kwindowsystem
import org.kde.kquickcontrolsaddons

import org.kde.plasma.components as PlasmaComponents3

import org.kde.kirigami as Kirigami
import org.kde.plasma.private.sessions

RowLayout {
    spacing: Kirigami.Units.largeSpacing

    KCoreAddons.KUser { id: kuser }
    Logic { id: logic }

    SessionManagement {
        id: sessionManager
    }

    RowLayout {
        Image {
            id: iconUser
            source: {
                var faceUrl = kuser.faceIconUrl.toString();
                if (faceUrl !== "") {
                    return faceUrl;
                }
                return "file:///usr/share/icons/breeze/apps/48/kuser.svg";
            }
            cache: false
            visible: source !== ""
            sourceSize.height: 24
            sourceSize.width: 24
            fillMode: Image.PreserveAspectFit
            Layout.alignment: Qt.AlignVCenter
            layer.enabled: true

            MouseArea {
                cursorShape: Qt.PointingHandCursor
                anchors.fill: parent
                onClicked: logic.runCommand("systemsettings kcm_users")
            }
        }

        PlasmaExtras.Heading {
            wrapMode: Text.NoWrap
            color: Kirigami.Theme.textColor
            level: 3
            font.bold: true
            text: qsTr(kuser.fullName)

            MouseArea {
                cursorShape: Qt.PointingHandCursor
                anchors.fill: parent
                onClicked: logic.runCommand("systemsettings kcm_users")
            }
        }
    }

    Item {
        Layout.fillWidth: true
    }

    PlasmaComponents3.ToolButton {
        id: configureButton
        icon.width: 24
        icon.height: 24
        icon.name: "configure"
        PlasmaComponents3.ToolTip.delay: 1000
        PlasmaComponents3.ToolTip.timeout: 1000
        PlasmaComponents3.ToolTip.visible: hovered
        PlasmaComponents3.ToolTip.text: i18n("System Preferences")

        onClicked: logic.runCommand("systemsettings")
    }

    PlasmaComponents3.ToolButton {
        id: powerButton
        icon.width: 24
        icon.height: 24
        icon.name: "system-shutdown"
        Layout.rightMargin: 10
        PlasmaComponents3.ToolTip.delay: 1000
        PlasmaComponents3.ToolTip.timeout: 1000
        PlasmaComponents3.ToolTip.visible: hovered
        PlasmaComponents3.ToolTip.text: i18n("Power")

        onClicked: {
            powerMenu.openRelative();
        }

        PlasmaExtras.Menu {
            id: powerMenu
            visualParent: powerButton
            placement: PlasmaExtras.Menu.TopPosedRightAlignedPopup

            PlasmaExtras.MenuItem {
                icon: "system-suspend"
                text: i18n("Sleep")
                onClicked: {
                    sessionManager.suspend();
                    if (typeof root !== "undefined" && root.toggle) root.toggle();
                }
            }

            PlasmaExtras.MenuItem {
                icon: "system-reboot"
                text: i18n("Restart")
                onClicked: {
                    sessionManager.requestReboot();
                    if (typeof root !== "undefined" && root.toggle) root.toggle();
                }
            }

            PlasmaExtras.MenuItem {
                icon: "system-shutdown"
                text: i18n("Shut down")
                onClicked: {
                    sessionManager.requestShutdown();
                    if (typeof root !== "undefined" && root.toggle) root.toggle();
                }
            }

            PlasmaExtras.MenuItem {
                icon: "system-lock-screen"
                text: i18n("Lock")
                visible: sessionManager.canLock
                onClicked: {
                    sessionManager.requestLogout();
                    if (typeof root !== "undefined" && root.toggle) root.toggle();
                }
            }
        }
    }
}

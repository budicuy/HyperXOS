import QtQuick
import org.kde.plasma.components as PlasmaComponents3
import org.kde.plasma.networkmanagement as PlasmaNM
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    PlasmaNM.Handler {
        id: handler
    }
    PlasmaNM.EnabledConnections {
        id: enabledConnections
    }
    PlasmaNM.AvailableDevices {
        id: availableDevices
    }
    PlasmaNM.ConnectionIcon {
        id: activeConnectionIcon
    }
    PlasmaNM.NetworkStatus {
        id: netStatus
    }

    readonly property bool wifiAvailable: availableDevices.wirelessDeviceAvailable
    readonly property bool wifiOn: wifiAvailable && enabledConnections.wirelessEnabled
    readonly property bool wiredAvailable: availableDevices.wiredDeviceAvailable
    readonly property bool isWiredActive: activeConnectionIcon.connectionIcon.indexOf("wired") !== -1

    label: {
        if (isWiredActive)
            return qsTr("Ethernet");
        if (wifiAvailable)
            return qsTr("Wi-Fi");
        if (wiredAvailable)
            return qsTr("Ethernet");
        return qsTr("Jaringan");
    }
    iconSource: {
        var base = activeConnectionIcon.connectionIcon;
        if (isWiredActive) {
            return base.length > 0 ? base : "network-wired";
        }
        if (!wifiAvailable) {
            return wiredAvailable ? "network-wired-disconnected" : "network-offline";
        }
        if (!wifiOn)
            return "network-wireless-offline";
        if (base.indexOf("network-wireless") === 0)
            return "network-wireless-100";
        if (base.indexOf("network-mobile") === 0)
            return "network-mobile-100";
        return base.length > 0 ? base : "network-wireless-100";
    }
    active: isWiredActive || wifiOn

    onClicked: {
        if (wifiAvailable) {
            handler.enableWireless(!wifiOn);
        } else {
            actionPanel.requestPage("network");
        }
    }

    tooltipText: {
        if (isWiredActive)
            return qsTr("Ethernet — Terhubung");
        if (!wifiAvailable)
            return qsTr("Modul Wi-Fi tidak terdeteksi");
        if (wifiOn)
            return qsTr("Wi-Fi — Aktif");
        return qsTr("Wi-Fi — Nonaktif");
    }

    onRightClicked: networkMenu.popup()

    onMiddleClicked: Qt.openUrlExternally("systemsettings://kcm_networkmanagement")

    PlasmaComponents3.Menu {
        id: networkMenu

        PlasmaComponents3.MenuItem {
            text: qsTr("Open Wi-Fi settings")
            icon.name: "configure"
            onClicked: Qt.openUrlExternally("systemsettings://kcm_networkmanagement")
        }
        PlasmaComponents3.MenuItem {
            text: qsTr("Create hotspot…")
            icon.name: "network-wireless-hotspot"
            onClicked: handler.createHotspot()
        }
        PlasmaComponents3.MenuItem {
            text: qsTr("Connect to hidden network…")
            icon.name: "network-wireless-secure"
            onClicked: handler.addAndActivateConnection("")
        }
        PlasmaComponents3.MenuItem {
            text: qsTr("Known networks…")
            icon.name: "preferences-system-network"
            onClicked: Qt.openUrlExternally("systemsettings://kcm_networkmanagement")
        }
    }
}

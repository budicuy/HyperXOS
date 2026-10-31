import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.plasma.components as PlasmaComponents3
import org.kde.bluezqt as BluezQt
import "../lib" as Lib
import "../js/funcs.js" as Funcs

Lib.SplitTile {
    id: tile

    property QtObject btManager: BluezQt.Manager
    readonly property bool hasAdapter: btManager.adapters.length > 0

    label: qsTr("Bluetooth")
    iconSource: hasAdapter ? Funcs.btStatus(btManager).icon : "network-bluetooth-inactive-symbolic"
    active: hasAdapter && Funcs.btStatus(btManager).active

    onClicked: {
        if (hasAdapter) {
            Funcs.toggleBluetooth(btManager);
        } else {
            actionPanel.requestPage("bluetooth");
        }
    }

    readonly property int connectedCount: {
        if (!hasAdapter) return 0;
        var c = 0;
        for (var i = 0; i < btManager.devices.length; i++) {
            if (btManager.devices[i].connected)
                c++;
        }
        return c;
    }

    tooltipText: {
        if (!hasAdapter)
            return qsTr("Bluetooth — Perangkat tidak terdeteksi");
        if (!Funcs.btStatus(btManager).active)
            return qsTr("Bluetooth — Off");
        if (connectedCount > 0)
            return qsTr("Bluetooth — %1 connected").arg(connectedCount);
        return qsTr("Bluetooth — On, not connected");
    }

    onRightClicked: btMenu.popup()

    onMiddleClicked: Qt.openUrlExternally("systemsettings://kcm_bluetooth")

    PlasmaComponents3.Menu {
        id: btMenu

        PlasmaComponents3.MenuItem {
            text: qsTr("Add new device…")
            icon.name: "list-add"
            onClicked: btExec.exec("bluedevil-wizard")
        }
        PlasmaComponents3.MenuItem {
            text: qsTr("Send file…")
            icon.name: "document-send"
            onClicked: btExec.exec("bluedevil-sendfile")
        }
        PlasmaComponents3.MenuItem {
            text: qsTr("Bluetooth settings")
            icon.name: "configure"
            onClicked: Qt.openUrlExternally("systemsettings://kcm_bluetooth")
        }
    }

    Plasma5Support.DataSource {
        id: btExec
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
        }
        function exec(cmd) {
            connectSource(cmd);
        }
    }
}

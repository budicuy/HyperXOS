import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    property bool touchpadEnabled: true
    property bool hasTouchpad: false

    visible: hasTouchpad

    label: touchpadEnabled ? qsTr("Touchpad On") : qsTr("Touchpad Off")
    iconSource: "input-touchpad-symbolic"
    active: !touchpadEnabled

    Plasma5Support.DataSource {
        id: detector
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
            if (data && data["stdout"]) {
                tile.hasTouchpad = (data["stdout"].trim() === "1");
            }
        }
        Component.onCompleted: {
            connectSource("grep -iqE 'touchpad|trackpad' /proc/bus/input/devices && echo 1 || echo 0");
        }
    }

    onClicked: {
        var willEnable = !touchpadEnabled;
        detector.connectSource("kwriteconfig6 --file kcminputrc --group Touchpad --key TouchpadEnabled " + (willEnable ? "true" : "false") + " && qdbus6 org.kde.KWin /KWin reconfigure");
        touchpadEnabled = willEnable;
    }

    tooltipText: touchpadEnabled ? qsTr("Touchpad — Enabled") : qsTr("Touchpad — Disabled")
}

import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("Lock Screen")
    iconSource: "system-lock-screen-symbolic"
    active: false

    Plasma5Support.DataSource {
        id: execEngine
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
        }
        function run(cmd) {
            connectSource(cmd);
        }
    }

    onClicked: {
        execEngine.run("qdbus6 org.freedesktop.ScreenSaver /ScreenSaver Lock || loginctl lock-session &");
    }

    tooltipText: qsTr("Lock Screen Immediately")
}

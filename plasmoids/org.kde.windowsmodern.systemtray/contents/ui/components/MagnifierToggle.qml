import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    property bool zoomed: false

    label: zoomed ? qsTr("Zoomed In") : qsTr("Magnifier")
    iconSource: "zoom-in-symbolic"
    active: zoomed

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
        zoomed = !zoomed;
        if (zoomed) {
            execEngine.run("kwriteconfig6 --file kwinrc --group Plugins --key zoomEnabled true && qdbus6 org.kde.KWin /KWin reconfigure && qdbus6 org.kde.kglobalaccel /component/kwin invokeShortcut 'view_zoom_in' 2>/dev/null || true");
        } else {
            execEngine.run("qdbus6 org.kde.kglobalaccel /component/kwin invokeShortcut 'view_actual_size' 2>/dev/null || true");
        }
    }

    tooltipText: zoomed ? qsTr("Magnifier — Active (Click to reset zoom)") : qsTr("Magnifier — Zoom in screen")
}

import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Project")
    iconSource: "video-display-symbolic"
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
        execEngine.run("kcmshell6 kcm_kscreen || systemsettings kcm_kscreen &");
    }

    tooltipText: qsTr("Display Configuration & Project Screen")
}

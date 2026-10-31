import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Vaults")
    iconSource: "folder-locked-symbolic"
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
        execEngine.run("kcmshell6 kcm_plasmavault || systemsettings &");
    }

    tooltipText: qsTr("Plasma Vaults — Encrypted Storage")
}

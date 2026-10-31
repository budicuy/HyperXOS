import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("HyperX Hub")
    iconSource: "preferences-system-symbolic"
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
        execEngine.run("hyper-x-tools &");
    }

    tooltipText: qsTr("HyperX Control Center — System Optimization & Hardware")
}

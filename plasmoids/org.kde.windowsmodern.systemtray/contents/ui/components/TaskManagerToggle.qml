import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("Task Manager")
    iconSource: "speedometer"
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
        execEngine.run("plasma-systemmonitor || ksysguard || konsole -e htop &");
    }

    tooltipText: qsTr("Task Manager — System Performance & Processes")
}

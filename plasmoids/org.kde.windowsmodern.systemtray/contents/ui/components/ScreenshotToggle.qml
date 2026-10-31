import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("Snipping Tool")
    iconSource: "edit-cut-symbolic"
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
        execEngine.run("spectacle -r || spectacle &");
    }

    onRightClicked: {
        execEngine.run("spectacle &");
    }

    tooltipText: qsTr("Snipping Tool — Capture Screen Region")
}

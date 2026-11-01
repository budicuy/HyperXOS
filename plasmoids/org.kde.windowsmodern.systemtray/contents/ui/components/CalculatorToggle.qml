import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("Calculator")
    iconSource: "accessories-calculator-symbolic"
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
        execEngine.run("(command -v kcalc >/dev/null 2>&1 && kcalc || plasmawindowed org.kde.plasma.calculator) >/dev/null 2>&1 &");
    }

    tooltipText: qsTr("Open Calculator")
}

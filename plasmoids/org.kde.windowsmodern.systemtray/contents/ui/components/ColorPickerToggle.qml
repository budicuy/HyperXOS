import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    label: qsTr("Color Picker")
    iconSource: "color-picker-symbolic"
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
        execEngine.run("(COLOR=$(qdbus6 org.kde.KWin /ColorPicker org.kde.kwin.ColorPicker.pick 2>/dev/null) && [ -n \"$COLOR\" ] && printf '%s' \"$COLOR\" | wl-copy 2>/dev/null) || kcolorchooser &");
    }

    tooltipText: qsTr("Color Picker — Pick any pixel color on screen")
}

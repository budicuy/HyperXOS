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
        execEngine.run(
            "(sleep 0.15; " +
            "RAW=$(qdbus6 org.kde.KWin /ColorPicker org.kde.kwin.ColorPicker.pick 2>/dev/null | tr -d '[:space:]'); " +
            "if [ -n \"$RAW\" ]; then " +
            "  HEX=$(printf '#%06X' $((RAW & 0xFFFFFF))); " +
            "  qdbus6 org.kde.klipper /klipper org.kde.klipper.klipper.setClipboardContents \"$HEX\" >/dev/null 2>&1; " +
            "  gdbus call --session --dest org.freedesktop.Notifications --object-path /org/freedesktop/Notifications " +
            "    --method org.freedesktop.Notifications.Notify \"Color Picker\" 0 \"color-picker\" " +
            "    \"Warna Disalin: $HEX\" \"Kode warna $HEX telah disalin ke Clipboard\" \"[]\" \"{}\" 3500 >/dev/null 2>&1; " +
            "fi) &"
        );
    }

    tooltipText: qsTr("Color Picker — Pilih warna piksel di layar & salin otomatis ke Clipboard")
}

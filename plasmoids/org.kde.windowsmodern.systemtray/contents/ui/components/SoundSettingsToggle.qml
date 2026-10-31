import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Sound Mixer")
    iconSource: "audio-volume-high-symbolic"
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
        execEngine.run("kcmshell6 kcm_pulseaudio || systemsettings kcm_pulseaudio &");
    }

    tooltipText: qsTr("Sound Mixer & Audio Devices")
}

import QtQuick
import org.kde.plasma.private.volume as Vol
import "../lib" as Lib

Lib.Tile {
    id: tile

    readonly property var sourceModel: Vol.SourceModel {}
    readonly property var sourceFilterModel: Vol.PulseObjectFilterModel {
        sourceModel: tile.sourceModel
        filterOutInactiveDevices: true
    }

    readonly property var preferredSource: Vol.PreferredDevice.source

    readonly property bool muted: preferredSource ? preferredSource.muted : true
    readonly property bool available: Boolean(preferredSource) && sourceFilterModel.count > 0

    label: muted ? qsTr("Mic Muted") : qsTr("Mic On")
    iconSource: muted ? "audio-input-microphone-muted" : "microphone-sensitivity-high"
    active: available && !muted
    visible: true

    onClicked: {
        if (preferredSource) {
            preferredSource.muted = !preferredSource.muted;
        } else {
            Qt.openUrlExternally("systemsettings://kcm_pulseaudio");
        }
    }

    tooltipText: !available
        ? qsTr("Microphone — No input device detected")
        : (muted ? qsTr("Microphone — Muted") : qsTr("Microphone — Active"))
}

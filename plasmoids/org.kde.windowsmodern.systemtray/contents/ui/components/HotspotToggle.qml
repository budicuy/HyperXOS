import QtQuick
import org.kde.plasma.networkmanagement as PlasmaNM
import "../lib" as Lib

Lib.Tile {
    id: tile

    PlasmaNM.Handler {
        id: handler
    }

    readonly property bool hotspotSupported: handler.hotspotSupported
    property bool hotspotActive: false

    label: qsTr("Hotspot")
    iconSource: "network-wireless-hotspot"
    active: hotspotActive
    visible: true

    onClicked: {
        if (!hotspotSupported) {
            Qt.openUrlExternally("systemsettings://kcm_networkmanagement");
            return;
        }
        if (hotspotActive) {
            handler.stopHotspot();
            hotspotActive = false;
        } else {
            handler.createHotspot();
            hotspotActive = true;
        }
    }

    tooltipText: !hotspotSupported
        ? qsTr("Mobile Hotspot — Wi-Fi adapter required")
        : (hotspotActive ? qsTr("Mobile Hotspot — On") : qsTr("Mobile Hotspot — Off"))
}

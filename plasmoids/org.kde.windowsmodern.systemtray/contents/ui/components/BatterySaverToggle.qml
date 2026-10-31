import QtQuick
import org.kde.plasma.private.batterymonitor
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    PowerProfilesControl {
        id: powerProfiles
    }

    readonly property bool saverOn: powerProfiles.activeProfile === "power-saver"
    readonly property bool perfOn: powerProfiles.activeProfile === "performance"

    visible: true
    label: saverOn ? qsTr("Power Saver") : (perfOn ? qsTr("Performance") : qsTr("Power Mode"))
    iconSource: {
        switch (powerProfiles.activeProfile) {
        case "performance":
            return "speedometer";
        case "power-saver":
            return "battery-low-symbolic";
        default:
            return "battery-profile-balanced-symbolic";
        }
    }
    active: saverOn || perfOn

    onClicked: {
        if (saverOn) {
            powerProfiles.setProfile(powerProfiles.configuredProfile || "balanced");
        } else {
            powerProfiles.setProfile("power-saver");
        }
    }

    tooltipText: {
        switch (powerProfiles.activeProfile) {
        case "performance":
            return qsTr("Power Mode — Performance");
        case "power-saver":
            return qsTr("Power Mode — Power Saver");
        default:
            return qsTr("Power Mode — Balanced");
        }
    }
}

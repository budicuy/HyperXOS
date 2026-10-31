import QtQuick
import org.kde.plasma.private.brightnesscontrolplugin
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.plasma.workspace.dbus as DBus
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    property int overrideState: -1 // -1 = use config/dbus, 0 = off, 1 = on
    property bool configActive: false
    readonly property bool dbusActive: nightLight.enabled && !nightLight.inhibited
    readonly property bool isNightLightOn: overrideState !== -1 ? (overrideState === 1) : (dbusActive || configActive)

    label: isNightLightOn ? qsTr("Night Light On") : qsTr("Night Light")
    iconSource: isNightLightOn ? "redshift-status-on-symbolic" : "redshift-status-off-symbolic"
    active: isNightLightOn

    Plasma5Support.DataSource {
        id: stateReader
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
            if (data && data["stdout"] !== undefined) {
                var val = data["stdout"].trim().toLowerCase();
                tile.configActive = (val === "true" || val === "1");
                tile.overrideState = -1;
            }
        }
        function refresh() {
            connectSource("kreadconfig6 --file kwinrc --group NightColor --key Active --default false");
        }
        Component.onCompleted: refresh()
    }

    Plasma5Support.DataSource {
        id: execEngine
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
            nightLight.updateAll();
            stateReader.refresh();
        }
        function run(cmd) {
            connectSource(cmd);
        }
    }

    onClicked: {
        var willEnable = !isNightLightOn;
        overrideState = willEnable ? 1 : 0;
        configActive = willEnable;

        if (willEnable) {
            var cmdOn = "TEMP=$(kreadconfig6 --file kwinrc --group NightColor --key NightTemperature --default 3800); "
                      + "kwriteconfig6 --file kwinrc --group NightColor --key Mode Constant; "
                      + "kwriteconfig6 --file kwinrc --group NightColor --key NightTemperature \"$TEMP\"; "
                      + "kwriteconfig6 --file kwinrc --group NightColor --key Active true; "
                      + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true; "
                      + "qdbus6 org.kde.KWin.NightLight /org/kde/KWin/NightLight org.kde.KWin.NightLight.preview \"$TEMP\" 2>/dev/null || true";
            execEngine.run(cmdOn);
            if (nightLight.inhibited) {
                NightLightInhibitor.toggleInhibition();
            }
        } else {
            var cmdOff = "kwriteconfig6 --file kwinrc --group NightColor --key Active false; "
                       + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true; "
                       + "qdbus6 org.kde.KWin.NightLight /org/kde/KWin/NightLight org.kde.KWin.NightLight.stopPreview 2>/dev/null || true";
            execEngine.run(cmdOff);
        }
    }

    tooltipText: isNightLightOn ? qsTr("Night Light — On (Warm Screen Filter)") : qsTr("Night Light — Off")

    DBus.Properties {
        id: nightLight
        busType: DBus.BusType.Session
        service: "org.kde.KWin.NightLight"
        path: "/org/kde/KWin/NightLight"
        iface: "org.kde.KWin.NightLight"

        readonly property bool available: Boolean(properties.available)
        readonly property bool running: Boolean(properties.running)
        readonly property bool enabled: Boolean(properties.enabled)
        readonly property bool inhibited: NightLightInhibitor.inhibited
    }
}

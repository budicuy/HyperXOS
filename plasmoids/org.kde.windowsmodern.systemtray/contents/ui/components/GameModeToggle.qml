import QtQuick
import org.kde.plasma.private.batterymonitor
import org.kde.plasma.plasma5support as Plasma5Support
import "../lib" as Lib

Lib.Tile {
    id: tile

    PowerProfilesControl {
        id: powerProfiles
    }

    property bool manualGameMode: false
    readonly property bool isPerformance: powerProfiles.activeProfile === "performance" || manualGameMode

    label: isPerformance ? qsTr("Game Mode On") : qsTr("Game Mode")
    iconSource: "input-gaming-symbolic"
    active: isPerformance

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
        var nextState = !isPerformance;
        manualGameMode = nextState;
        if (nextState) {
            powerProfiles.setProfile("performance");
            execEngine.run("kwriteconfig6 --file kwinrc --group Compositing --key LatencyPolicy ExtremelyLow && qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true");
        } else {
            powerProfiles.setProfile("balanced");
            execEngine.run("kwriteconfig6 --file kwinrc --group Compositing --key LatencyPolicy Medium && qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true");
        }
    }

    tooltipText: isPerformance
        ? qsTr("Game Mode — Active (High Performance & Low Latency)")
        : qsTr("Game Mode — Off (Click to boost gaming performance)")
}

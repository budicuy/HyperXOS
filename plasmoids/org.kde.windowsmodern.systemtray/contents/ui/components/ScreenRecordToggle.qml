import QtQuick
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.taskmanager as TaskManager
import org.kde.pipewire.record as PipeWireRecord
import "../lib" as Lib

Lib.Tile {
    id: tile

    property bool recordingActive: false
    property int elapsedSeconds: 0
    property string savedFilePath: ""

    function pad2(n) {
        return n < 10 ? "0" + n : "" + n;
    }

    function formatTime(sec) {
        var m = Math.floor(sec / 60);
        var s = sec % 60;
        return pad2(m) + ":" + pad2(s);
    }

    label: recordingActive
        ? qsTr("Stop (%1)").arg(formatTime(elapsedSeconds))
        : qsTr("Screen Record")
    iconSource: recordingActive
        ? "media-playback-stop-symbolic"
        : "media-record-symbolic"
    active: recordingActive

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
        Component.onCompleted: {
            run("mkdir -p \"$HOME/Videos\"");
        }
    }

    TaskManager.ScreencastingRequest {
        id: screencast
        onNodeIdChanged: {
            if (screencast.nodeId > 0 && tile.recordingActive) {
                recorder.nodeId = screencast.nodeId;
                recorder.start();
            }
        }
    }

    PipeWireRecord.PipeWireRecord {
        id: recorder
        onErrorFound: function(err) {
            console.warn("ScreenRecordToggle error:", err);
            tile.recordingActive = false;
            screencast.outputName = "";
        }
    }

    Timer {
        id: recordTimer
        interval: 1000
        repeat: true
        running: tile.recordingActive
        onTriggered: {
            tile.elapsedSeconds += 1;
        }
    }

    function startRecording() {
        var screens = Qt.application.screens;
        if (!screens || screens.length === 0) {
            return;
        }
        var now = new Date();
        var ts = now.getFullYear()
            + pad2(now.getMonth() + 1)
            + pad2(now.getDate()) + "_"
            + pad2(now.getHours())
            + pad2(now.getMinutes())
            + pad2(now.getSeconds());
        var ext = (recorder.extension && recorder.extension.length > 0) ? recorder.extension : "webm";
        tile.savedFilePath = "~/Videos/ScreenRecord_" + ts + "." + ext;
        tile.elapsedSeconds = 0;
        tile.recordingActive = true;

        recorder.output = tile.savedFilePath;
        screencast.outputName = "";
        screencast.outputName = screens[0].name;
        if (screencast.nodeId > 0) {
            recorder.nodeId = screencast.nodeId;
            recorder.start();
        }
    }

    function stopRecording() {
        tile.recordingActive = false;
        recorder.stop();
        screencast.outputName = "";
        recorder.nodeId = 0;

        if (tile.savedFilePath !== "") {
            execEngine.run(
                "gdbus call --session --dest org.freedesktop.Notifications " +
                "--object-path /org/freedesktop/Notifications " +
                "--method org.freedesktop.Notifications.Notify " +
                "\"Screen Recorder\" 0 \"media-record\" " +
                "\"Rekaman Layar Disimpan\" " +
                "\"Video disimpan di " + tile.savedFilePath + "\" \"[]\" \"{}\" 4000 >/dev/null 2>&1 &"
            );
        }
    }

    onClicked: {
        if (tile.recordingActive) {
            tile.stopRecording();
        } else {
            tile.startRecording();
        }
    }

    tooltipText: recordingActive
        ? qsTr("Klik untuk menghentikan rekaman layar (%1)").arg(tile.savedFilePath)
        : qsTr("Screen Recorder — Klik untuk langsung merekam layar ke ~/Videos")
}

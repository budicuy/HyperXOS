import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: terminalRoot
    anchors.fill: parent
    color: "#0d1117"
    border.color: "#30363d"
    border.width: 1
    radius: 6

    property string rawLog: ""
    property string formattedLog: ""
    property bool autoScroll: true
    property int lastLength: 0
    property int totalLines: 0

    // Colorizer function for terminal syntax highlighting
    function colorize(raw) {
        if (!raw || raw.length === 0) {
            return "<font color='#58a6ff'><b>==> [HyperXOS] Menyiapkan stream log eksekusi langsung...</b></font><br/>" +
                   "<font color='#8b949e'>  -> Menunggu engine instalasi Calamares memulai eksekusi...</font><br/>" +
                   "<font color='#8b949e'>  -> Pacstrap online fresh install, konfigurasi KDE Plasma 6 minimal, deteksi driver otomatis, dan Limine bootloader.</font>";
        }

        var lines = raw.split("\n");
        totalLines = lines.length;
        var html = [];
        var maxLines = 2500; // Keep up to 2500 lines for high performance rendering
        var startIdx = Math.max(0, lines.length - maxLines);

        for (var i = startIdx; i < lines.length; i++) {
            var line = lines[i];
            if (line.length === 0) continue;

            // HTML entity escape
            var esc = line.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
            // Strip ANSI terminal color codes
            esc = esc.replace(/\x1b\[[0-9;]*[a-zA-Z]/g, "");

            if (esc.indexOf("ERROR") !== -1 || esc.indexOf("error:") !== -1 || esc.indexOf("failed") !== -1 || esc.indexOf("[-] ") !== -1) {
                html.push("<font color='#f85149'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("WARNING") !== -1 || esc.indexOf("warning:") !== -1 || esc.indexOf("[!] ") !== -1) {
                html.push("<font color='#d29922'>" + esc + "</font>");
            } else if (esc.indexOf("[HyperXOS-HW]") !== -1) {
                html.push("<font color='#79c0ff'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("[HyperXOS-PostInstall]") !== -1) {
                html.push("<font color='#d2a8ff'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("[HyperXTools]") !== -1) {
                html.push("<font color='#f0883e'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("pacstrap:") !== -1 || esc.indexOf("installing ") !== -1) {
                html.push("<font color='#7ee787'>" + esc + "</font>");
            } else if (esc.indexOf("==>") !== -1 || esc.indexOf("  ->") !== -1 || esc.indexOf("[INFO]") !== -1) {
                html.push("<font color='#58a6ff'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("[✓]") !== -1 || esc.indexOf("SUCCESS") !== -1 || esc.indexOf("PASSED") !== -1 || esc.indexOf("completed") !== -1) {
                html.push("<font color='#3fb950'><b>" + esc + "</b></font>");
            } else if (esc.indexOf("target_env_call") !== -1 || esc.indexOf("Running command") !== -1 || esc.indexOf("mkfs") !== -1 || esc.indexOf("subvolume") !== -1) {
                html.push("<font color='#bc8cff'>" + esc + "</font>");
            } else {
                html.push("<font color='#c9d1d9'>" + esc + "</font>");
            }
        }
        return html.join("<br/>");
    }

    // Multi-source log poller
    function pollLogs() {
        var sources = [
            "file:///tmp/calamares-launch.log",
            "file:///root/.cache/calamares/session.log",
            "file:///home/hyperxos/.cache/calamares/session.log",
            "file:///var/log/Calamares.log"
        ];

        function tryNext(idx) {
            if (idx >= sources.length) return;
            var xhr = new XMLHttpRequest();
            xhr.open("GET", sources[idx]);
            xhr.onreadystatechange = function() {
                if (xhr.readyState === XMLHttpRequest.DONE) {
                    if ((xhr.status === 200 || xhr.status === 0) && xhr.responseText && xhr.responseText.length > 0) {
                        var text = xhr.responseText;
                        if (text.length !== lastLength) {
                            lastLength = text.length;
                            rawLog = text;
                            formattedLog = colorize(text);
                            if (autoScroll) {
                                scrollTimer.restart();
                            }
                        }
                    } else {
                        tryNext(idx + 1);
                    }
                }
            };
            xhr.send();
        }
        tryNext(0);
    }

    // Periodic live log fetcher (every 350ms)
    Timer {
        id: pollTimer
        interval: 350
        running: true
        repeat: true
        onTriggered: pollLogs()
    }

    // Delayed smooth auto-scroll to bottom
    Timer {
        id: scrollTimer
        interval: 40
        running: false
        repeat: false
        onTriggered: {
            if (autoScroll) {
                flickableArea.contentY = Math.max(0, logTextEdit.contentHeight - flickableArea.height + 25);
            }
        }
    }

    Component.onCompleted: {
        formattedLog = colorize("");
        pollLogs();
    }

    // ==========================================
    // Top Terminal Header Bar
    // ==========================================
    Rectangle {
        id: terminalHeader
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 38
        color: "#161b22"
        radius: 6

        Rectangle {
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            height: 6
            color: "#161b22"
        }

        Rectangle {
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            height: 1
            color: "#30363d"
        }

        // Window decorative buttons
        Row {
            id: windowButtons
            anchors.left: parent.left
            anchors.leftMargin: 12
            anchors.verticalCenter: parent.verticalCenter
            spacing: 7

            Rectangle { width: 11; height: 11; radius: 5.5; color: "#ff5f56"; border.color: "#e0443e"; border.width: 1 }
            Rectangle { width: 11; height: 11; radius: 5.5; color: "#ffbd2e"; border.color: "#dea123"; border.width: 1 }
            Rectangle { width: 11; height: 11; radius: 5.5; color: "#27c93f"; border.color: "#1aab29"; border.width: 1 }
        }

        // Terminal Title & Line Counter
        Row {
            anchors.left: windowButtons.right
            anchors.leftMargin: 16
            anchors.verticalCenter: parent.verticalCenter
            spacing: 8

            Text {
                text: "HyperXOS Installer Terminal — Live Execution Log"
                color: "#e6edf3"
                font.bold: true
                font.pixelSize: 12
                font.family: "monospace"
            }

            Rectangle {
                width: lineCountText.implicitWidth + 10
                height: 18
                radius: 3
                color: "#21262d"
                border.color: "#30363d"
                border.width: 1
                anchors.verticalCenter: parent.verticalCenter

                Text {
                    id: lineCountText
                    anchors.centerIn: parent
                    text: totalLines + " baris"
                    color: "#8b949e"
                    font.pixelSize: 10
                    font.family: "monospace"
                }
            }
        }

        // Control Tools on Right Side
        Row {
            anchors.right: parent.right
            anchors.rightMargin: 12
            anchors.verticalCenter: parent.verticalCenter
            spacing: 8

            // Live Pulse Badge
            Rectangle {
                width: 62
                height: 22
                radius: 4
                color: "#1f2937"
                border.color: "#3fb950"
                border.width: 1
                anchors.verticalCenter: parent.verticalCenter

                Row {
                    anchors.centerIn: parent
                    spacing: 5
                    Rectangle {
                        width: 7
                        height: 7
                        radius: 3.5
                        color: "#3fb950"
                        anchors.verticalCenter: parent.verticalCenter
                    }
                    Text {
                        text: "LIVE"
                        color: "#3fb950"
                        font.bold: true
                        font.pixelSize: 10
                        font.family: "monospace"
                    }
                }
            }

            // Auto-Scroll Toggle Button
            Rectangle {
                width: 125
                height: 24
                radius: 4
                color: autoScroll ? "#238636" : "#21262d"
                border.color: autoScroll ? "#2ea043" : "#30363d"
                border.width: 1
                anchors.verticalCenter: parent.verticalCenter

                Text {
                    anchors.centerIn: parent
                    text: autoScroll ? "✓ Auto-Scroll ON" : "⏸ Auto-Scroll OFF"
                    color: autoScroll ? "#ffffff" : "#8b949e"
                    font.bold: true
                    font.pixelSize: 11
                    font.family: "monospace"
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        autoScroll = !autoScroll;
                        if (autoScroll) {
                            scrollTimer.restart();
                        }
                    }
                }
            }

            // Scroll to Bottom Button
            Rectangle {
                width: 75
                height: 24
                radius: 4
                color: "#21262d"
                border.color: "#30363d"
                border.width: 1
                anchors.verticalCenter: parent.verticalCenter

                Text {
                    anchors.centerIn: parent
                    text: "▼ Bawah"
                    color: "#58a6ff"
                    font.bold: true
                    font.pixelSize: 11
                    font.family: "monospace"
                }

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        autoScroll = true;
                        scrollTimer.restart();
                    }
                }
            }
        }
    }

    // ==========================================
    // Terminal Log Scrollable Body
    // ==========================================
    Flickable {
        id: flickableArea
        anchors.top: terminalHeader.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 4
        contentWidth: logTextEdit.width
        contentHeight: logTextEdit.height + 25
        clip: true
        boundsBehavior: Flickable.StopAtBounds

        TextEdit {
            id: logTextEdit
            width: flickableArea.width - 24
            text: formattedLog
            textFormat: TextEdit.RichText
            readOnly: true
            selectByMouse: true
            color: "#c9d1d9"
            font.family: "JetBrains Mono", "Hack", "Fira Code", "Consolas", "monospace"
            font.pixelSize: 11
            wrapMode: TextEdit.WrapAnywhere
            padding: 10
        }

        // Custom High-Contrast Scrollbar
        ScrollBar.vertical: ScrollBar {
            id: vertBar
            policy: ScrollBar.AlwaysOn
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: 10

            contentItem: Rectangle {
                implicitWidth: 8
                radius: 4
                color: vertBar.pressed ? "#58a6ff" : (vertBar.hovered ? "#388bfd" : "#30363d")
            }
            background: Rectangle {
                implicitWidth: 10
                color: "#0d1117"
            }

            onPositionChanged: {
                if (vertBar.position < 1.0 - vertBar.size - 0.05) {
                    if (vertBar.pressed) {
                        autoScroll = false;
                    }
                } else {
                    if (vertBar.pressed) {
                        autoScroll = true;
                    }
                }
            }
        }

        // Mouse Wheel Scroll Detection
        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.NoButton
            onWheel: function(wheel) {
                var delta = wheel.angleDelta.y;
                flickableArea.contentY = Math.max(0, Math.min(flickableArea.contentHeight - flickableArea.height, flickableArea.contentY - delta));
                if (delta > 0) {
                    // Scrolling up disables auto-scroll so user can read log
                    autoScroll = false;
                } else if (flickableArea.contentY >= flickableArea.contentHeight - flickableArea.height - 35) {
                    // Reaching bottom re-enables auto-scroll
                    autoScroll = true;
                }
                wheel.accepted = true;
            }
        }
    }
}

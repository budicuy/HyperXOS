import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.plasma.components as PlasmaComponents3
import org.kde.plasma.private.brightnesscontrolplugin
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.plasma.workspace.dbus as DBus
import org.kde.kcmutils
import "../lib" as Lib

Lib.Page {
    id: page

    title: qsTr("Night Light")
    contentFillsHeight: false

    property bool configActive: false
    property int configTemp: 3800
    readonly property bool isNightLightOn: (nightLight.enabled && !nightLight.inhibited) || configActive

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
        readonly property int currentTemperature: properties.currentTemperature || page.configTemp
        readonly property int targetTemperature: properties.targetTemperature || page.configTemp
    }

    Plasma5Support.DataSource {
        id: configReader
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
            if (data && data["stdout"] !== undefined) {
                var parts = data["stdout"].trim().split("|");
                if (parts.length >= 2) {
                    page.configActive = (parts[0].toLowerCase() === "true" || parts[0] === "1");
                    var parsedTemp = parseInt(parts[1], 10);
                    if (!isNaN(parsedTemp) && parsedTemp >= 2500 && parsedTemp <= 6500) {
                        page.configTemp = parsedTemp;
                        if (!tempSlider.pressed) {
                            tempSlider.value = parsedTemp;
                        }
                    }
                }
            }
        }
        function refresh() {
            connectSource("echo \"$(kreadconfig6 --file kwinrc --group NightColor --key Active --default false)|$(kreadconfig6 --file kwinrc --group NightColor --key NightTemperature --default 3800)\"");
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
            configReader.refresh();
        }
        function run(cmd) {
            connectSource(cmd);
        }
    }

    footer: Lib.MoreSettingsLink {
        text: qsTr("More night light settings")
        onClicked: KCMLauncher.openSystemSettings("kcm_nightlight")
    }

    ColumnLayout {
        Layout.fillWidth: true
        spacing: 16

        // Status Card
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 52
            radius: 6
            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.04)
            border.width: 1
            border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.08)

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 14
                anchors.rightMargin: 14

                Kirigami.Icon {
                    source: page.isNightLightOn ? "redshift-status-on-symbolic" : "redshift-status-off-symbolic"
                    width: 22
                    height: 22
                    color: page.isNightLightOn ? Kirigami.Theme.highlightColor : Kirigami.Theme.textColor
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2
                    PlasmaComponents3.Label {
                        text: qsTr("Night Light")
                        font.pixelSize: 12
                        font.bold: true
                        color: Kirigami.Theme.textColor
                    }
                    PlasmaComponents3.Label {
                        text: page.isNightLightOn ? qsTr("Aktif (Filter Warna Hangat)") : qsTr("Nonaktif")
                        font.pixelSize: 10
                        opacity: 0.6
                        color: Kirigami.Theme.textColor
                    }
                }

                PlasmaComponents3.Switch {
                    checked: page.isNightLightOn
                    onToggled: {
                        var willEnable = checked;
                        page.configActive = willEnable;
                        if (willEnable) {
                            execEngine.run("kwriteconfig6 --file kwinrc --group NightColor --key Mode Constant; "
                                         + "kwriteconfig6 --file kwinrc --group NightColor --key NightTemperature " + Math.round(tempSlider.value) + "; "
                                         + "kwriteconfig6 --file kwinrc --group NightColor --key Active true; "
                                         + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true; "
                                         + "qdbus6 org.kde.KWin.NightLight /org/kde/KWin/NightLight org.kde.KWin.NightLight.preview " + Math.round(tempSlider.value) + " 2>/dev/null || true");
                            if (nightLight.inhibited) {
                                NightLightInhibitor.toggleInhibition();
                            }
                        } else {
                            execEngine.run("kwriteconfig6 --file kwinrc --group NightColor --key Active false; "
                                         + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true; "
                                         + "qdbus6 org.kde.KWin.NightLight /org/kde/KWin/NightLight org.kde.KWin.NightLight.stopPreview 2>/dev/null || true");
                        }
                    }
                }
            }
        }

        // Temperature Slider Section
        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            RowLayout {
                Layout.fillWidth: true
                PlasmaComponents3.Label {
                    text: qsTr("Suhu Warna Layar (Color Temperature)")
                    font.pixelSize: 11
                    font.bold: true
                    color: Kirigami.Theme.textColor
                }
                Item { Layout.fillWidth: true }
                PlasmaComponents3.Label {
                    text: Math.round(tempSlider.value) + " K"
                    font.pixelSize: 11
                    font.bold: true
                    color: Kirigami.Theme.highlightColor
                }
            }

            PlasmaComponents3.Slider {
                id: tempSlider
                Layout.fillWidth: true
                from: 2500
                to: 6500
                stepSize: 100
                value: page.configTemp

                onMoved: {
                    execEngine.run("qdbus6 org.kde.KWin.NightLight /org/kde/KWin/NightLight org.kde.KWin.NightLight.preview " + Math.round(tempSlider.value) + " 2>/dev/null || true");
                }

                onPressedChanged: {
                    if (!pressed) {
                        var t = Math.round(tempSlider.value);
                        page.configTemp = t;
                        execEngine.run("kwriteconfig6 --file kwinrc --group NightColor --key Mode Constant; "
                                     + "kwriteconfig6 --file kwinrc --group NightColor --key NightTemperature " + t + "; "
                                     + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true");
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                PlasmaComponents3.Label {
                    text: qsTr("Hangat (2500 K)")
                    font.pixelSize: 9
                    opacity: 0.6
                    color: Kirigami.Theme.textColor
                }
                Item { Layout.fillWidth: true }
                PlasmaComponents3.Label {
                    text: qsTr("Dingin (6500 K)")
                    font.pixelSize: 9
                    opacity: 0.6
                    color: Kirigami.Theme.textColor
                }
            }
        }
    }
}

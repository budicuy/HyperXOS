pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts

import org.kde.kcmutils as KCMUtils
import org.kde.kirigami as Kirigami
import org.kde.plasma.plasmoid

KCMUtils.ScrollViewKCM {
    id: root

    property list<string> cfg_tilesOrder: []
    property bool cfg_showNetwork: true
    property bool cfg_showBluetooth: true
    property bool cfg_showAirplane: true
    property bool cfg_showBatterySaver: true
    property bool cfg_showNightLight: true
    property bool cfg_showColorScheme: true
    property bool cfg_showDnd: true
    property bool cfg_showMicMute: true
    property bool cfg_showHotspot: true
    property bool cfg_showInhibitSleep: true
    property bool cfg_showScreenshot: true
    property bool cfg_showHyperXTools: true
    property bool cfg_showDisplayConfig: true
    property bool cfg_showTaskManager: true
    property bool cfg_showVault: true
    property bool cfg_showWeather: true
    property bool cfg_showSoundSettings: true
    property bool cfg_showAccessibility: true
    property bool cfg_showGameMode: true
    property bool cfg_showClipboard: true
    property bool cfg_showScreenRecord: true
    property bool cfg_showColorPicker: true
    property bool cfg_showMagnifier: true
    property bool cfg_showCalculator: true
    property bool cfg_showTerminal: true
    property bool cfg_showEmojiPicker: true
    property bool cfg_showLockScreen: true
    property bool cfg_showTouchpad: true

    readonly property var allTilesMeta: ({
        "network":       { "name": i18n("Network (Wi-Fi / LAN)"), "icon": "network-wireless" },
        "bluetooth":     { "name": i18n("Bluetooth"),             "icon": "bluetooth" },
        "airplane":      { "name": i18n("Airplane Mode"),         "icon": "airplane-mode" },
        "battery":       { "name": i18n("Power Mode / Battery"),  "icon": "battery-profile-balanced-symbolic" },
        "nightlight":    { "name": i18n("Night Light"),           "icon": "redshift-status-on-symbolic" },
        "colorscheme":   { "name": i18n("Dark / Light Mode"),     "icon": "contrast" },
        "dnd":           { "name": i18n("Do Not Disturb / Notif"),"icon": "notifications" },
        "micmute":       { "name": i18n("Microphone Mute"),       "icon": "audio-input-microphone-muted" },
        "hotspot":       { "name": i18n("Mobile Hotspot"),        "icon": "network-wireless-hotspot" },
        "inhibitsleep":  { "name": i18n("No Sleep (Inhibit)"),    "icon": "system-lock-screen" },
        "screenshot":    { "name": i18n("Snipping Tool"),         "icon": "accessories-screenshot-tool" },
        "hyperxtools":   { "name": i18n("HyperX Control Hub"),    "icon": "preferences-system" },
        "display":       { "name": i18n("Display / Project"),     "icon": "video-display" },
        "taskmanager":   { "name": i18n("Task Manager"),          "icon": "utilities-system-monitor" },
        "vault":         { "name": i18n("Plasma Vaults"),         "icon": "plasmavault" },
        "weather":       { "name": i18n("Weather"),               "icon": "weather-clouds" },
        "soundsettings": { "name": i18n("Sound Mixer"),           "icon": "audio-volume-high" },
        "accessibility": { "name": i18n("Accessibility"),         "icon": "preferences-desktop-accessibility" },
        "gamemode":      { "name": i18n("Game Mode"),             "icon": "input-gaming" },
        "clipboard":     { "name": i18n("Clipboard History"),     "icon": "klipper" },
        "screenrecord":  { "name": i18n("Screen Recorder"),       "icon": "media-record" },
        "colorpicker":   { "name": i18n("Color Picker"),          "icon": "color-picker" },
        "magnifier":     { "name": i18n("Magnifier / Zoom"),      "icon": "zoom-in" },
        "calculator":    { "name": i18n("Calculator"),            "icon": "accessories-calculator" },
        "terminal":      { "name": i18n("Terminal (Konsole)"),    "icon": "utilities-terminal" },
        "emojipicker":   { "name": i18n("Emoji Picker"),          "icon": "preferences-desktop-emoticons" },
        "lockscreen":    { "name": i18n("Lock Screen"),           "icon": "system-lock-screen" },
        "touchpad":      { "name": i18n("Touchpad (Auto-Detect)"),"icon": "input-touchpad" }
    })

    readonly property var defaultOrder: [
        "network", "bluetooth", "airplane", "battery", "nightlight", "colorscheme",
        "dnd", "micmute", "hotspot", "inhibitsleep", "screenshot", "hyperxtools",
        "display", "taskmanager", "vault", "weather", "soundsettings", "accessibility",
        "gamemode", "clipboard", "screenrecord", "colorpicker", "magnifier", "calculator",
        "terminal", "emojipicker", "lockscreen", "touchpad"
    ]

    property var workingOrder: []

    Component.onCompleted: {
        initWorkingOrder();
    }

    function initWorkingOrder() {
        var list = [];
        if (cfg_tilesOrder && cfg_tilesOrder.length > 0) {
            for (var i = 0; i < cfg_tilesOrder.length; ++i) {
                if (allTilesMeta[cfg_tilesOrder[i]]) {
                    list.push(cfg_tilesOrder[i]);
                }
            }
        }
        for (var j = 0; j < defaultOrder.length; ++j) {
            if (list.indexOf(defaultOrder[j]) === -1) {
                list.push(defaultOrder[j]);
            }
        }
        workingOrder = list;
    }

    function moveUp(index) {
        if (index <= 0) return;
        var list = [...workingOrder];
        var item = list.splice(index, 1)[0];
        list.splice(index - 1, 0, item);
        workingOrder = list;
        cfg_tilesOrder = list;
    }

    function moveDown(index) {
        if (index >= workingOrder.length - 1) return;
        var list = [...workingOrder];
        var item = list.splice(index, 1)[0];
        list.splice(index + 1, 0, item);
        workingOrder = list;
        cfg_tilesOrder = list;
    }

    function resetOrder() {
        workingOrder = [...defaultOrder];
        cfg_tilesOrder = [...defaultOrder];
    }

    function isTileEnabled(id) {
        switch (id) {
        case "network": return cfg_showNetwork;
        case "bluetooth": return cfg_showBluetooth;
        case "airplane": return cfg_showAirplane;
        case "battery": return cfg_showBatterySaver;
        case "nightlight": return cfg_showNightLight;
        case "colorscheme": return cfg_showColorScheme;
        case "dnd": return cfg_showDnd;
        case "micmute": return cfg_showMicMute;
        case "hotspot": return cfg_showHotspot;
        case "inhibitsleep": return cfg_showInhibitSleep;
        case "screenshot": return cfg_showScreenshot;
        case "hyperxtools": return cfg_showHyperXTools;
        case "display": return cfg_showDisplayConfig;
        case "taskmanager": return cfg_showTaskManager;
        case "vault": return cfg_showVault;
        case "weather": return cfg_showWeather;
        case "soundsettings": return cfg_showSoundSettings;
        case "accessibility": return cfg_showAccessibility;
        case "gamemode": return cfg_showGameMode;
        case "clipboard": return cfg_showClipboard;
        case "screenrecord": return cfg_showScreenRecord;
        case "colorpicker": return cfg_showColorPicker;
        case "magnifier": return cfg_showMagnifier;
        case "calculator": return cfg_showCalculator;
        case "terminal": return cfg_showTerminal;
        case "emojipicker": return cfg_showEmojiPicker;
        case "lockscreen": return cfg_showLockScreen;
        case "touchpad": return cfg_showTouchpad;
        default: return true;
        }
    }

    function setTileEnabled(id, val) {
        switch (id) {
        case "network": cfg_showNetwork = val; break;
        case "bluetooth": cfg_showBluetooth = val; break;
        case "airplane": cfg_showAirplane = val; break;
        case "battery": cfg_showBatterySaver = val; break;
        case "nightlight": cfg_showNightLight = val; break;
        case "colorscheme": cfg_showColorScheme = val; break;
        case "dnd": cfg_showDnd = val; break;
        case "micmute": cfg_showMicMute = val; break;
        case "hotspot": cfg_showHotspot = val; break;
        case "inhibitsleep": cfg_showInhibitSleep = val; break;
        case "screenshot": cfg_showScreenshot = val; break;
        case "hyperxtools": cfg_showHyperXTools = val; break;
        case "display": cfg_showDisplayConfig = val; break;
        case "taskmanager": cfg_showTaskManager = val; break;
        case "vault": cfg_showVault = val; break;
        case "weather": cfg_showWeather = val; break;
        case "soundsettings": cfg_showSoundSettings = val; break;
        case "accessibility": cfg_showAccessibility = val; break;
        case "gamemode": cfg_showGameMode = val; break;
        case "clipboard": cfg_showClipboard = val; break;
        case "screenrecord": cfg_showScreenRecord = val; break;
        case "colorpicker": cfg_showColorPicker = val; break;
        case "magnifier": cfg_showMagnifier = val; break;
        case "calculator": cfg_showCalculator = val; break;
        case "terminal": cfg_showTerminal = val; break;
        case "emojipicker": cfg_showEmojiPicker = val; break;
        case "lockscreen": cfg_showLockScreen = val; break;
        case "touchpad": cfg_showTouchpad = val; break;
        }
    }

    header: ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            type: Kirigami.MessageType.Information
            text: i18n("Tip: Anda juga dapat mengedit urutan dan menampilkan/menyembunyikan ubin secara langsung di dalam popup panel dengan mengklik ikon Pensil (✏️).")
            visible: true
        }

        RowLayout {
            Layout.fillWidth: true
            Item { Layout.fillWidth: true }
            QQC2.Button {
                text: i18n("Reset ke Urutan Standar")
                icon.name: "edit-undo"
                onClicked: root.resetOrder()
            }
        }
    }

    view: ListView {
        id: listView
        model: root.workingOrder
        clip: true
        spacing: 4

        delegate: Rectangle {
            id: delegateRoot
            required property int index
            required property string modelData

            width: listView.width
            height: 48
            radius: 6
            color: index < 9
                ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.08)
                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.04)
            border.width: 1
            border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 12
                anchors.rightMargin: 12
                spacing: 10

                QQC2.CheckBox {
                    id: tileCheck
                    checked: root.isTileEnabled(delegateRoot.modelData)
                    onToggled: root.setTileEnabled(delegateRoot.modelData, checked)
                }

                Kirigami.Icon {
                    width: 22
                    height: 22
                    source: root.allTilesMeta[delegateRoot.modelData] ? root.allTilesMeta[delegateRoot.modelData].icon : "preferences-system"
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2
                    QQC2.Label {
                        text: root.allTilesMeta[delegateRoot.modelData] ? root.allTilesMeta[delegateRoot.modelData].name : delegateRoot.modelData
                        font.bold: true
                        font.pixelSize: 12
                    }
                    QQC2.Label {
                        text: i18n("Halaman %1 (Grid 3x3)").arg(Math.floor(delegateRoot.index / 9) + 1)
                        font.pixelSize: 10
                        opacity: 0.6
                    }
                }

                QQC2.ToolButton {
                    icon.name: "go-up"
                    enabled: delegateRoot.index > 0
                    onClicked: root.moveUp(delegateRoot.index)
                    QQC2.ToolTip.text: i18n("Pindah ke Atas")
                    QQC2.ToolTip.visible: hovered
                }

                QQC2.ToolButton {
                    icon.name: "go-down"
                    enabled: delegateRoot.index < root.workingOrder.length - 1
                    onClicked: root.moveDown(delegateRoot.index)
                    QQC2.ToolTip.text: i18n("Pindah ke Bawah")
                    QQC2.ToolTip.visible: hovered
                }
            }
        }
    }
}

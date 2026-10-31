/*
    Action Panel — quick-toggle tiles + sliders shown in the expander popup.

    Features:
    - Bottom horizontal pagination dots + arrows with smooth left/right slide animation
    - 3 full 3x3 pages (27 shortcuts + auto-detected touchpad)
    - Clean In-Panel Edit Mode (Clean Edit Card with Click-to-Swap, < / > step buttons,
      unclipped ✕ remove badge, in-panel + Tambah drawer, and proportional action buttons).
*/
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts

import org.kde.kirigami as Kirigami
import org.kde.plasma.components as PlasmaComponents
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.plasma.plasmoid

import "components" as Components
import "js/colorType.js" as ColorType

ColumnLayout {
    id: actionPanel

    signal requestPage(string name)

    readonly property real scale: Plasmoid.configuration.scale / 100
    property int currentPage: 0
    property bool hasTouchpad: false
    property bool editMode: false
    property bool addingTileMode: false
    property string selectedSwapTile: ""

    onEditModeChanged: {
        if (!editMode) {
            addingTileMode = false;
            selectedSwapTile = "";
        }
    }

    Plasma5Support.DataSource {
        id: tpDetector
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
            if (data && data["stdout"]) {
                actionPanel.hasTouchpad = (data["stdout"].trim() === "1");
            }
        }
        Component.onCompleted: {
            connectSource("grep -iqE 'touchpad|trackpad' /proc/bus/input/devices && echo 1 || echo 0");
        }
    }

    readonly property var defaultOrder: [
        "network", "bluetooth", "airplane", "battery", "nightlight", "colorscheme",
        "dnd", "micmute", "hotspot", "inhibitsleep", "screenshot", "hyperxtools",
        "display", "taskmanager", "vault", "weather", "soundsettings", "accessibility",
        "gamemode", "clipboard", "screenrecord", "colorpicker", "magnifier", "calculator",
        "terminal", "emojipicker", "lockscreen", "touchpad"
    ]

    readonly property var tileMeta: ({
        "network":       { "label": qsTr("Ethernet"),      "icon": "network-wired-symbolic" },
        "bluetooth":     { "label": qsTr("Bluetooth"),     "icon": "preferences-system-bluetooth-symbolic" },
        "airplane":      { "label": qsTr("Airplane"),      "icon": "network-flightmode-on-symbolic" },
        "battery":       { "label": qsTr("Power Mode"),    "icon": "battery-profile-balanced-symbolic" },
        "nightlight":    { "label": qsTr("Night Light"),   "icon": "redshift-status-on-symbolic" },
        "colorscheme":   { "label": qsTr("Dark Mode"),     "icon": "weather-clear-night-symbolic" },
        "dnd":           { "label": qsTr("Do Not Disturb"),"icon": "notifications-disabled-symbolic" },
        "micmute":       { "label": qsTr("Mic Mute"),      "icon": "audio-input-microphone-muted-symbolic" },
        "hotspot":       { "label": qsTr("Hotspot"),       "icon": "network-wireless-hotspot-symbolic" },
        "inhibitsleep":  { "label": qsTr("No Sleep"),      "icon": "system-suspend-inhibited-symbolic" },
        "screenshot":    { "label": qsTr("Snipping Tool"), "icon": "edit-cut-symbolic" },
        "hyperxtools":   { "label": qsTr("HyperX Hub"),    "icon": "preferences-system-symbolic" },
        "display":       { "label": qsTr("Project"),       "icon": "video-display-symbolic" },
        "taskmanager":   { "label": qsTr("Task Manager"),  "icon": "speedometer" },
        "vault":         { "label": qsTr("Vaults"),        "icon": "folder-locked-symbolic" },
        "weather":       { "label": qsTr("Weather"),       "icon": "weather-clouds-symbolic" },
        "soundsettings": { "label": qsTr("Sound Mixer"),   "icon": "audio-volume-high-symbolic" },
        "accessibility": { "label": qsTr("Accessibility"), "icon": "accessibility-symbolic" },
        "gamemode":      { "label": qsTr("Game Mode"),     "icon": "input-gaming-symbolic" },
        "clipboard":     { "label": qsTr("Clipboard"),     "icon": "edit-paste-symbolic" },
        "screenrecord":  { "label": qsTr("Screen Record"), "icon": "media-record-symbolic" },
        "colorpicker":   { "label": qsTr("Color Picker"),  "icon": "color-picker-symbolic" },
        "magnifier":     { "label": qsTr("Magnifier"),     "icon": "zoom-in-symbolic" },
        "calculator":    { "label": qsTr("Calculator"),    "icon": "accessories-calculator-symbolic" },
        "terminal":      { "label": qsTr("Terminal"),      "icon": "utilities-terminal-symbolic" },
        "emojipicker":   { "label": qsTr("Emoji Picker"),  "icon": "face-smile-symbolic" },
        "lockscreen":    { "label": qsTr("Lock Screen"),   "icon": "system-lock-screen-symbolic" },
        "touchpad":      { "label": qsTr("Touchpad"),      "icon": "input-touchpad-symbolic" }
    })

    function isTileSupported(id) {
        if (id === "touchpad") return actionPanel.hasTouchpad;
        return Boolean(tileMeta[id]);
    }

    function isTileVisible(id) {
        if (!isTileSupported(id)) return false;
        switch (id) {
        case "network": return Plasmoid.configuration.showNetwork;
        case "bluetooth": return Plasmoid.configuration.showBluetooth;
        case "airplane": return Plasmoid.configuration.showAirplane;
        case "battery": return Plasmoid.configuration.showBatterySaver;
        case "nightlight": return Plasmoid.configuration.showNightLight;
        case "colorscheme": return Plasmoid.configuration.showColorScheme;
        case "dnd": return Plasmoid.configuration.showDnd;
        case "micmute": return Plasmoid.configuration.showMicMute;
        case "hotspot": return Plasmoid.configuration.showHotspot;
        case "inhibitsleep": return Plasmoid.configuration.showInhibitSleep;
        case "screenshot": return Plasmoid.configuration.showScreenshot;
        case "hyperxtools": return Plasmoid.configuration.showHyperXTools;
        case "display": return Plasmoid.configuration.showDisplayConfig;
        case "taskmanager": return Plasmoid.configuration.showTaskManager;
        case "vault": return Plasmoid.configuration.showVault;
        case "weather": return Plasmoid.configuration.showWeather;
        case "soundsettings": return Plasmoid.configuration.showSoundSettings;
        case "accessibility": return Plasmoid.configuration.showAccessibility;
        case "gamemode": return Plasmoid.configuration.showGameMode;
        case "clipboard": return Plasmoid.configuration.showClipboard;
        case "screenrecord": return Plasmoid.configuration.showScreenRecord;
        case "colorpicker": return Plasmoid.configuration.showColorPicker;
        case "magnifier": return Plasmoid.configuration.showMagnifier;
        case "calculator": return Plasmoid.configuration.showCalculator;
        case "terminal": return Plasmoid.configuration.showTerminal;
        case "emojipicker": return Plasmoid.configuration.showEmojiPicker;
        case "lockscreen": return Plasmoid.configuration.showLockScreen;
        case "touchpad": return Plasmoid.configuration.showTouchpad;
        default: return false;
        }
    }

    function setTileVisible(id, val) {
        if (!val && selectedSwapTile === id) {
            selectedSwapTile = "";
        }
        switch (id) {
        case "network": Plasmoid.configuration.showNetwork = val; break;
        case "bluetooth": Plasmoid.configuration.showBluetooth = val; break;
        case "airplane": Plasmoid.configuration.showAirplane = val; break;
        case "battery": Plasmoid.configuration.showBatterySaver = val; break;
        case "nightlight": Plasmoid.configuration.showNightLight = val; break;
        case "colorscheme": Plasmoid.configuration.showColorScheme = val; break;
        case "dnd": Plasmoid.configuration.showDnd = val; break;
        case "micmute": Plasmoid.configuration.showMicMute = val; break;
        case "hotspot": Plasmoid.configuration.showHotspot = val; break;
        case "inhibitsleep": Plasmoid.configuration.showInhibitSleep = val; break;
        case "screenshot": Plasmoid.configuration.showScreenshot = val; break;
        case "hyperxtools": Plasmoid.configuration.showHyperXTools = val; break;
        case "display": Plasmoid.configuration.showDisplayConfig = val; break;
        case "taskmanager": Plasmoid.configuration.showTaskManager = val; break;
        case "vault": Plasmoid.configuration.showVault = val; break;
        case "weather": Plasmoid.configuration.showWeather = val; break;
        case "soundsettings": Plasmoid.configuration.showSoundSettings = val; break;
        case "accessibility": Plasmoid.configuration.showAccessibility = val; break;
        case "gamemode": Plasmoid.configuration.showGameMode = val; break;
        case "clipboard": Plasmoid.configuration.showClipboard = val; break;
        case "screenrecord": Plasmoid.configuration.showScreenRecord = val; break;
        case "colorpicker": Plasmoid.configuration.showColorPicker = val; break;
        case "magnifier": Plasmoid.configuration.showMagnifier = val; break;
        case "calculator": Plasmoid.configuration.showCalculator = val; break;
        case "terminal": Plasmoid.configuration.showTerminal = val; break;
        case "emojipicker": Plasmoid.configuration.showEmojiPicker = val; break;
        case "lockscreen": Plasmoid.configuration.showLockScreen = val; break;
        case "touchpad": Plasmoid.configuration.showTouchpad = val; break;
        }
    }

    readonly property var fullNormalizedOrder: {
        var order = Plasmoid.configuration.tilesOrder || [];
        var list = [];
        for (var i = 0; i < order.length; ++i) {
            if (tileMeta[order[i]] && list.indexOf(order[i]) === -1) {
                list.push(order[i]);
            }
        }
        for (var j = 0; j < defaultOrder.length; ++j) {
            if (list.indexOf(defaultOrder[j]) === -1) {
                list.push(defaultOrder[j]);
            }
        }
        return list;
    }

    readonly property var activeTilesList: {
        return fullNormalizedOrder.filter(function(id) {
            return actionPanel.isTileVisible(id);
        });
    }

    readonly property var hiddenTilesList: {
        return fullNormalizedOrder.filter(function(id) {
            return actionPanel.isTileSupported(id) && !actionPanel.isTileVisible(id);
        });
    }

    readonly property int totalPages: Math.max(1, Math.ceil(activeTilesList.length / 9))

    onTotalPagesChanged: {
        if (currentPage >= totalPages) {
            currentPage = Math.max(0, totalPages - 1);
        }
    }

    function pageTilesAt(pageIndex) {
        return activeTilesList.slice(pageIndex * 9, (pageIndex + 1) * 9);
    }

    function swapTwoTiles(idA, idB) {
        if (!idA || !idB || idA === idB) return;
        var full = [...fullNormalizedOrder];
        var idxA = full.indexOf(idA);
        var idxB = full.indexOf(idB);
        if (idxA !== -1 && idxB !== -1) {
            var tmp = full[idxA];
            full[idxA] = full[idxB];
            full[idxB] = tmp;
            Plasmoid.configuration.tilesOrder = full;
        }
    }

    function moveActiveTile(id, delta) {
        var active = [...activeTilesList];
        var idx = active.indexOf(id);
        if (idx === -1) return;
        var targetIdx = idx + delta;
        if (targetIdx < 0 || targetIdx >= active.length) return;

        swapTwoTiles(id, active[targetIdx]);

        var targetPage = Math.floor(targetIdx / 9);
        if (targetPage !== currentPage) {
            currentPage = targetPage;
        }
    }

    function resetAllTiles() {
        Plasmoid.configuration.tilesOrder = [...defaultOrder];
        for (var i = 0; i < defaultOrder.length; ++i) {
            setTileVisible(defaultOrder[i], true);
        }
        currentPage = 0;
        addingTileMode = false;
        selectedSwapTile = "";
    }

    Component { id: compNetwork; Components.NetworkToggle { onArrowClicked: actionPanel.requestPage("network") } }
    Component { id: compBluetooth; Components.BluetoothToggle { onArrowClicked: actionPanel.requestPage("bluetooth") } }
    Component { id: compAirplane; Components.AirplaneToggle {} }
    Component { id: compBattery; Components.BatterySaverToggle { onArrowClicked: actionPanel.requestPage("battery") } }
    Component { id: compNightlight; Components.NightLightToggle { onArrowClicked: actionPanel.requestPage("nightlight") } }
    Component { id: compColorscheme; Components.ColorSchemeToggle {} }
    Component { id: compDnd; Components.DndToggle { onArrowClicked: actionPanel.requestPage("notifications") } }
    Component { id: compMicmute; Components.MicMuteToggle {} }
    Component { id: compHotspot; Components.HotspotToggle {} }
    Component { id: compInhibitsleep; Components.InhibitSleepToggle {} }
    Component { id: compScreenshot; Components.ScreenshotToggle {} }
    Component { id: compHyperxtools; Components.HyperXToolsToggle {} }
    Component { id: compDisplay; Components.DisplayConfigToggle { onArrowClicked: actionPanel.requestPage("display") } }
    Component { id: compTaskmanager; Components.TaskManagerToggle {} }
    Component { id: compVault; Components.VaultToggle { onArrowClicked: actionPanel.requestPage("vault") } }
    Component { id: compWeather; Components.WeatherToggle { onArrowClicked: actionPanel.requestPage("weather"); onClicked: actionPanel.requestPage("weather") } }
    Component { id: compSoundsettings; Components.SoundSettingsToggle { onArrowClicked: actionPanel.requestPage("volume") } }
    Component { id: compAccessibility; Components.AccessibilityToggle {} }
    Component { id: compGamemode; Components.GameModeToggle {} }
    Component { id: compClipboard; Components.ClipboardToggle { onArrowClicked: actionPanel.requestPage("clipboard"); onClicked: actionPanel.requestPage("clipboard") } }
    Component { id: compScreenrecord; Components.ScreenRecordToggle {} }
    Component { id: compColorpicker; Components.ColorPickerToggle {} }
    Component { id: compMagnifier; Components.MagnifierToggle {} }
    Component { id: compCalculator; Components.CalculatorToggle {} }
    Component { id: compTerminal; Components.TerminalToggle {} }
    Component { id: compEmojipicker; Components.EmojiPickerToggle {} }
    Component { id: compLockscreen; Components.LockScreenToggle {} }
    Component { id: compTouchpad; Components.TouchpadToggle {} }

    function getTileComponent(id) {
        switch (id) {
        case "network": return compNetwork;
        case "bluetooth": return compBluetooth;
        case "airplane": return compAirplane;
        case "battery": return compBattery;
        case "nightlight": return compNightlight;
        case "colorscheme": return compColorscheme;
        case "dnd": return compDnd;
        case "micmute": return compMicmute;
        case "hotspot": return compHotspot;
        case "inhibitsleep": return compInhibitsleep;
        case "screenshot": return compScreenshot;
        case "hyperxtools": return compHyperxtools;
        case "display": return compDisplay;
        case "taskmanager": return compTaskmanager;
        case "vault": return compVault;
        case "weather": return compWeather;
        case "soundsettings": return compSoundsettings;
        case "accessibility": return compAccessibility;
        case "gamemode": return compGamemode;
        case "clipboard": return compClipboard;
        case "screenrecord": return compScreenrecord;
        case "colorpicker": return compColorpicker;
        case "magnifier": return compMagnifier;
        case "calculator": return compCalculator;
        case "terminal": return compTerminal;
        case "emojipicker": return compEmojipicker;
        case "lockscreen": return compLockscreen;
        case "touchpad": return compTouchpad;
        default: return null;
        }
    }

    Layout.fillWidth: true
    spacing: 6 * actionPanel.scale

    // ─── Subtle Edit Mode Helper Banner (Only visible in Edit Mode) ──────────
    RowLayout {
        visible: actionPanel.editMode
        Layout.fillWidth: true
        Layout.leftMargin: 16 * actionPanel.scale
        Layout.rightMargin: 16 * actionPanel.scale
        Layout.topMargin: 4 * actionPanel.scale

        PlasmaComponents.Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            text: actionPanel.addingTileMode
                ? qsTr("Pilih shortcut untuk ditambahkan ke panel")
                : (actionPanel.selectedSwapTile !== ""
                    ? qsTr("Klik ubin tujuan untuk menukar posisi")
                    : qsTr("Klik ubin untuk tukar posisi, gunakan < >, atau klik ✕"))
            font.pixelSize: 10
            font.bold: true
            color: Kirigami.Theme.highlightColor
        }
    }

    // ─── MAIN TILES VIEWPORT (With Safe Padding so ✕ Badge Never Clips!) ─────
    Item {
        id: tilesViewport
        readonly property real pad: 8 * actionPanel.scale
        readonly property real gridWidth: (96 * 3 + 12 * 2) * actionPanel.scale
        readonly property real gridHeight: (68 * 3 + 12 * 2) * actionPanel.scale
        readonly property real pageWidth: gridWidth + pad * 2
        readonly property real pageGap: 16 * actionPanel.scale

        Layout.alignment: Qt.AlignHCenter
        Layout.topMargin: actionPanel.editMode ? 0 : (4 * actionPanel.scale)
        Layout.preferredWidth: pageWidth
        Layout.minimumWidth: pageWidth
        Layout.maximumWidth: pageWidth
        Layout.preferredHeight: gridHeight + pad * 2
        implicitWidth: pageWidth
        implicitHeight: gridHeight + pad * 2
        clip: true

        // Wheel gesture handler
        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.NoButton
            enabled: !actionPanel.addingTileMode
            onWheel: wheel => {
                if (actionPanel.totalPages <= 1) return;
                var delta = wheel.angleDelta.y !== 0 ? wheel.angleDelta.y : wheel.angleDelta.x;
                if (delta < 0 && actionPanel.currentPage < actionPanel.totalPages - 1) {
                    actionPanel.currentPage += 1;
                } else if (delta > 0 && actionPanel.currentPage > 0) {
                    actionPanel.currentPage -= 1;
                }
            }
        }

        // Horizontally sliding pages container
        Row {
            id: pagesRow
            visible: !actionPanel.addingTileMode
            y: 0
            height: tilesViewport.gridHeight + tilesViewport.pad * 2
            spacing: tilesViewport.pageGap
            x: -actionPanel.currentPage * (tilesViewport.pageWidth + tilesViewport.pageGap)

            Behavior on x {
                NumberAnimation {
                    duration: 280
                    easing.type: Easing.OutCubic
                }
            }

            Repeater {
                model: actionPanel.totalPages
                delegate: Item {
                    id: pageContainer
                    required property int index
                    readonly property int pageIndex: index
                    readonly property var tilesForThisPage: actionPanel.pageTilesAt(pageIndex)

                    width: tilesViewport.pageWidth
                    height: tilesViewport.gridHeight + tilesViewport.pad * 2

                    GridLayout {
                        x: tilesViewport.pad
                        y: tilesViewport.pad
                        width: tilesViewport.gridWidth
                        height: tilesViewport.gridHeight
                        columns: 3
                        uniformCellWidths: true
                        uniformCellHeights: true
                        rowSpacing: 12 * actionPanel.scale
                        columnSpacing: 12 * actionPanel.scale

                        Repeater {
                            model: pageContainer.tilesForThisPage
                            delegate: Item {
                                id: tileCell
                                required property string modelData
                                required property int index
                                readonly property int globalIndex: pageContainer.pageIndex * 9 + index
                                readonly property bool isSelectedForSwap: actionPanel.selectedSwapTile === modelData

                                Layout.fillWidth: true
                                Layout.preferredWidth: 96 * actionPanel.scale
                                Layout.minimumWidth: 96 * actionPanel.scale
                                Layout.maximumWidth: 96 * actionPanel.scale
                                Layout.preferredHeight: 68 * actionPanel.scale
                                Layout.minimumHeight: 68 * actionPanel.scale
                                Layout.maximumHeight: 68 * actionPanel.scale

                                // Normal Live Tile (hidden during editMode so icons/arrows never overlap!)
                                Loader {
                                    id: tileLoader
                                    anchors.fill: parent
                                    visible: !actionPanel.editMode
                                    enabled: !actionPanel.editMode
                                    sourceComponent: actionPanel.getTileComponent(tileCell.modelData)
                                }

                                // ─── CLEAN EDIT CARD (Shown in editMode — Zero Icon Overlap!) ───
                                ColumnLayout {
                                    anchors.fill: parent
                                    visible: actionPanel.editMode
                                    spacing: 4

                                    Rectangle {
                                        id: editCardBg
                                        Layout.fillWidth: true
                                        Layout.preferredHeight: 48 * actionPanel.scale
                                        Layout.minimumHeight: 48 * actionPanel.scale
                                        Layout.maximumHeight: 48 * actionPanel.scale
                                        height: 48 * actionPanel.scale
                                        radius: 6
                                        color: tileCell.isSelectedForSwap
                                            ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.22)
                                            : (cardSwapMA.containsMouse
                                                ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.09)
                                                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.05))
                                        border.width: tileCell.isSelectedForSwap ? 2 : 1
                                        border.color: tileCell.isSelectedForSwap
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.18)

                                        // Click center card to Select / Swap position with another tile
                                        MouseArea {
                                            id: cardSwapMA
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                if (actionPanel.selectedSwapTile === "") {
                                                    actionPanel.selectedSwapTile = tileCell.modelData;
                                                } else if (actionPanel.selectedSwapTile === tileCell.modelData) {
                                                    actionPanel.selectedSwapTile = "";
                                                } else {
                                                    actionPanel.swapTwoTiles(actionPanel.selectedSwapTile, tileCell.modelData);
                                                    actionPanel.selectedSwapTile = "";
                                                }
                                            }
                                        }

                                        // Move Left (<) button neatly on left inside card
                                        Rectangle {
                                            anchors.left: parent.left
                                            anchors.leftMargin: 4
                                            anchors.verticalCenter: parent.verticalCenter
                                            width: 20 * actionPanel.scale
                                            height: 28 * actionPanel.scale
                                            radius: 4
                                            color: moveLeftMA.containsMouse
                                                ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25)
                                                : "transparent"
                                            opacity: tileCell.globalIndex > 0 ? 0.85 : 0.2

                                            Kirigami.Icon {
                                                anchors.centerIn: parent
                                                width: 12 * actionPanel.scale
                                                height: 12 * actionPanel.scale
                                                source: "go-previous"
                                                color: Kirigami.Theme.textColor
                                            }

                                            MouseArea {
                                                id: moveLeftMA
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: tileCell.globalIndex > 0 ? Qt.PointingHandCursor : Qt.ArrowCursor
                                                onClicked: {
                                                    if (tileCell.globalIndex > 0) {
                                                        actionPanel.selectedSwapTile = "";
                                                        actionPanel.moveActiveTile(tileCell.modelData, -1);
                                                    }
                                                }
                                            }
                                        }

                                        // Clean Center Tile Icon (No overlapping divider or arrow!)
                                        Kirigami.Icon {
                                            anchors.centerIn: parent
                                            width: 20 * actionPanel.scale
                                            height: 20 * actionPanel.scale
                                            source: actionPanel.tileMeta[tileCell.modelData]
                                                ? actionPanel.tileMeta[tileCell.modelData].icon
                                                : "preferences-system"
                                            color: tileCell.isSelectedForSwap
                                                ? Kirigami.Theme.highlightColor
                                                : Kirigami.Theme.textColor
                                            isMask: true
                                        }

                                        // Move Right (>) button neatly on right inside card
                                        Rectangle {
                                            anchors.right: parent.right
                                            anchors.rightMargin: 4
                                            anchors.verticalCenter: parent.verticalCenter
                                            width: 20 * actionPanel.scale
                                            height: 28 * actionPanel.scale
                                            radius: 4
                                            color: moveRightMA.containsMouse
                                                ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25)
                                                : "transparent"
                                            opacity: tileCell.globalIndex < actionPanel.activeTilesList.length - 1 ? 0.85 : 0.2

                                            Kirigami.Icon {
                                                anchors.centerIn: parent
                                                width: 12 * actionPanel.scale
                                                height: 12 * actionPanel.scale
                                                source: "go-next"
                                                color: Kirigami.Theme.textColor
                                            }

                                            MouseArea {
                                                id: moveRightMA
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: tileCell.globalIndex < actionPanel.activeTilesList.length - 1 ? Qt.PointingHandCursor : Qt.ArrowCursor
                                                onClicked: {
                                                    if (tileCell.globalIndex < actionPanel.activeTilesList.length - 1) {
                                                        actionPanel.selectedSwapTile = "";
                                                        actionPanel.moveActiveTile(tileCell.modelData, 1);
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    PlasmaComponents.Label {
                                        Layout.fillWidth: true
                                        Layout.preferredWidth: 96 * actionPanel.scale
                                        Layout.maximumWidth: 96 * actionPanel.scale
                                        Layout.preferredHeight: 16 * actionPanel.scale
                                        Layout.alignment: Qt.AlignHCenter
                                        text: actionPanel.tileMeta[tileCell.modelData]
                                            ? actionPanel.tileMeta[tileCell.modelData].label
                                            : tileCell.modelData
                                        color: tileCell.isSelectedForSwap
                                            ? Kirigami.Theme.highlightColor
                                            : Kirigami.Theme.textColor
                                        font.pixelSize: 11
                                        font.bold: tileCell.isSelectedForSwap
                                        elide: Text.ElideRight
                                        maximumLineCount: 1
                                        horizontalAlignment: Text.AlignHCenter
                                    }
                                }

                                // Unpin / Remove badge (✕) — Fully inside padded viewport so NEVER clipped!
                                Rectangle {
                                    visible: actionPanel.editMode && actionPanel.activeTilesList.length > 1
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.topMargin: -5 * actionPanel.scale
                                    anchors.rightMargin: -5 * actionPanel.scale
                                    width: 18 * actionPanel.scale
                                    height: 18 * actionPanel.scale
                                    radius: 9 * actionPanel.scale
                                    color: removeMA.containsMouse ? "#EF4444" : "#DC2626"
                                    border.width: 1.5
                                    border.color: Kirigami.Theme.backgroundColor
                                    z: 10

                                    PlasmaComponents.Label {
                                        anchors.centerIn: parent
                                        text: "✕"
                                        font.pixelSize: 9
                                        font.bold: true
                                        color: "#FFFFFF"
                                    }

                                    MouseArea {
                                        id: removeMA
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: actionPanel.setTileVisible(tileCell.modelData, false)
                                    }
                                }
                            }
                        }

                        // Fill remaining slots up to 9 so grid cells stay 100% uniform
                        Repeater {
                            model: Math.max(0, 9 - pageContainer.tilesForThisPage.length)
                            delegate: Item {
                                Layout.fillWidth: true
                                Layout.preferredWidth: 96 * actionPanel.scale
                                Layout.minimumWidth: 96 * actionPanel.scale
                                Layout.maximumWidth: 96 * actionPanel.scale
                                Layout.preferredHeight: 68 * actionPanel.scale
                                Layout.minimumHeight: 68 * actionPanel.scale
                                Layout.maximumHeight: 68 * actionPanel.scale
                            }
                        }
                    }
                }
            }
        }

        // ─── IN-PANEL "+ ADD QUICK SETTING" DRAWER ─────────────────────────────
        Rectangle {
            anchors.fill: parent
            anchors.margins: tilesViewport.pad
            visible: actionPanel.addingTileMode
            radius: 8
            color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.04)
            border.width: 1
            border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 8
                spacing: 6

                PlasmaComponents.Label {
                    visible: actionPanel.hiddenTilesList.length === 0
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                    text: qsTr("Semua shortcut sudah ditampilkan di panel.")
                    font.pixelSize: 11
                    opacity: 0.7
                    color: Kirigami.Theme.textColor
                }

                QQC2.ScrollView {
                    visible: actionPanel.hiddenTilesList.length > 0
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true

                    ListView {
                        model: actionPanel.hiddenTilesList
                        spacing: 4

                        delegate: Rectangle {
                            id: addRow
                            required property string modelData
                            width: ListView.view.width
                            height: 36 * actionPanel.scale
                            radius: 6
                            color: addRowMA.containsMouse
                                ? Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16)
                                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.05)
                            border.width: 1
                            border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.08)

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 10
                                anchors.rightMargin: 10
                                spacing: 10

                                Kirigami.Icon {
                                    Layout.preferredWidth: 18 * actionPanel.scale
                                    Layout.preferredHeight: 18 * actionPanel.scale
                                    source: actionPanel.tileMeta[addRow.modelData] ? actionPanel.tileMeta[addRow.modelData].icon : "preferences-system"
                                    color: Kirigami.Theme.textColor
                                }

                                PlasmaComponents.Label {
                                    Layout.fillWidth: true
                                    text: actionPanel.tileMeta[addRow.modelData] ? actionPanel.tileMeta[addRow.modelData].label : addRow.modelData
                                    font.pixelSize: 11
                                    font.bold: true
                                    color: Kirigami.Theme.textColor
                                }

                                PlasmaComponents.Label {
                                    text: qsTr("+ Tambah")
                                    font.pixelSize: 10
                                    font.bold: true
                                    color: Kirigami.Theme.highlightColor
                                }
                            }

                            MouseArea {
                                id: addRowMA
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: {
                                    actionPanel.setTileVisible(addRow.modelData, true);
                                    if (actionPanel.hiddenTilesList.length === 0) {
                                        actionPanel.addingTileMode = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // ─── BOTTOM PAGINATION DOTS (Centered Below Tiles Grid, Above Sliders) ───
    RowLayout {
        visible: !actionPanel.addingTileMode && actionPanel.totalPages > 1
        Layout.alignment: Qt.AlignHCenter
        Layout.bottomMargin: 2 * actionPanel.scale
        spacing: 6

        // Left Page Arrow (<)
        Rectangle {
            Layout.preferredWidth: 18 * actionPanel.scale
            Layout.preferredHeight: 18 * actionPanel.scale
            width: 18 * actionPanel.scale
            height: 18 * actionPanel.scale
            radius: 4
            color: prevPageMA.containsMouse
                ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)
                : "transparent"
            opacity: actionPanel.currentPage > 0 ? 1.0 : 0.3

            Kirigami.Icon {
                anchors.centerIn: parent
                width: 12 * actionPanel.scale
                height: 12 * actionPanel.scale
                source: "go-previous"
                color: Kirigami.Theme.textColor
            }

            MouseArea {
                id: prevPageMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: actionPanel.currentPage > 0 ? Qt.PointingHandCursor : Qt.ArrowCursor
                onClicked: {
                    if (actionPanel.currentPage > 0) {
                        actionPanel.currentPage -= 1;
                    }
                }
            }
        }

        // Horizontal Page Dots
        Repeater {
            model: actionPanel.totalPages
            delegate: Rectangle {
                required property int index
                readonly property bool isCurrent: actionPanel.currentPage === index
                Layout.alignment: Qt.AlignVCenter
                Layout.preferredWidth: isCurrent ? 16 * actionPanel.scale : 6 * actionPanel.scale
                Layout.preferredHeight: 6 * actionPanel.scale
                width: isCurrent ? 16 * actionPanel.scale : 6 * actionPanel.scale
                height: 6 * actionPanel.scale
                radius: 3 * actionPanel.scale
                color: isCurrent ? Kirigami.Theme.highlightColor : Kirigami.Theme.textColor
                opacity: isCurrent ? 1.0 : (dotMA.containsMouse ? 0.6 : 0.3)

                Behavior on Layout.preferredWidth { NumberAnimation { duration: 200; easing.type: Easing.OutCubic } }
                Behavior on width { NumberAnimation { duration: 200; easing.type: Easing.OutCubic } }
                Behavior on opacity { NumberAnimation { duration: 150 } }
                Behavior on color { ColorAnimation { duration: 150 } }

                MouseArea {
                    id: dotMA
                    anchors.fill: parent
                    anchors.margins: -4
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: actionPanel.currentPage = index
                }
            }
        }

        // Right Page Arrow (>)
        Rectangle {
            Layout.preferredWidth: 18 * actionPanel.scale
            Layout.preferredHeight: 18 * actionPanel.scale
            width: 18 * actionPanel.scale
            height: 18 * actionPanel.scale
            radius: 4
            color: nextPageMA.containsMouse
                ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.1)
                : "transparent"
            opacity: actionPanel.currentPage < actionPanel.totalPages - 1 ? 1.0 : 0.3

            Kirigami.Icon {
                anchors.centerIn: parent
                width: 12 * actionPanel.scale
                height: 12 * actionPanel.scale
                source: "go-next"
                color: Kirigami.Theme.textColor
            }

            MouseArea {
                id: nextPageMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: actionPanel.currentPage < actionPanel.totalPages - 1 ? Qt.PointingHandCursor : Qt.ArrowCursor
                onClicked: {
                    if (actionPanel.currentPage < actionPanel.totalPages - 1) {
                        actionPanel.currentPage += 1;
                    }
                }
            }
        }
    }

    // ─── SLIDERS SECTION (Brightness & Volume) ─────────────────────────────────
    GridLayout {
        Layout.fillWidth: true
        Layout.leftMargin: 16 * actionPanel.scale
        Layout.rightMargin: 16 * actionPanel.scale
        Layout.bottomMargin: 4 * actionPanel.scale
        columns: 3
        rowSpacing: 12 * actionPanel.scale
        columnSpacing: 0
        opacity: actionPanel.editMode ? 0.45 : 1.0
        enabled: !actionPanel.editMode

        Components.BrightnessSlider {
            Layout.fillWidth: true
            Layout.columnSpan: 3
            visible: Plasmoid.configuration.showBrightness
            showArrow: true
            panelScreenGeometry: Plasmoid.screenGeometry
            panelScreenIndex: Plasmoid.containment.screen
            onArrowClicked: actionPanel.requestPage("brightness")
        }
        Components.VolumeSlider {
            Layout.fillWidth: true
            Layout.columnSpan: 3
            Layout.preferredHeight: 36
            visible: Plasmoid.configuration.showVolume
            onArrowClicked: actionPanel.requestPage("volume")
        }
    }

    // ─── BOTTOM ACTION BAR (In-Panel Edit Bar & System Settings) ───────────────
    RowLayout {
        Layout.fillWidth: true
        Layout.leftMargin: 16 * actionPanel.scale
        Layout.rightMargin: 16 * actionPanel.scale
        Layout.bottomMargin: 10 * actionPanel.scale
        spacing: 8

        // "+ Add" button (only visible in editMode)
        Rectangle {
            visible: actionPanel.editMode
            Layout.preferredHeight: 28 * actionPanel.scale
            Layout.preferredWidth: addBtnRow.implicitWidth + 18
            radius: 6
            color: addBtnMA.containsMouse
                ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12)
                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.06)
            border.width: 1
            border.color: actionPanel.addingTileMode
                ? Kirigami.Theme.highlightColor
                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.14)

            RowLayout {
                id: addBtnRow
                anchors.centerIn: parent
                spacing: 5

                Kirigami.Icon {
                    Layout.preferredWidth: 14 * actionPanel.scale
                    Layout.preferredHeight: 14 * actionPanel.scale
                    source: actionPanel.addingTileMode ? "go-previous" : "list-add"
                    color: Kirigami.Theme.textColor
                }

                PlasmaComponents.Label {
                    text: actionPanel.addingTileMode
                        ? qsTr("Kembali")
                        : qsTr("Tambah (%1)").arg(actionPanel.hiddenTilesList.length)
                    font.pixelSize: 11
                    font.bold: true
                    color: Kirigami.Theme.textColor
                }
            }

            MouseArea {
                id: addBtnMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: actionPanel.addingTileMode = !actionPanel.addingTileMode
            }
        }

        // "Reset" button (only visible in editMode)
        Rectangle {
            visible: actionPanel.editMode
            Layout.preferredHeight: 28 * actionPanel.scale
            Layout.preferredWidth: resetBtnRow.implicitWidth + 16
            radius: 6
            color: resetBtnMA.containsMouse
                ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12)
                : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.06)
            border.width: 1
            border.color: Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.14)

            RowLayout {
                id: resetBtnRow
                anchors.centerIn: parent
                spacing: 5

                Kirigami.Icon {
                    Layout.preferredWidth: 14 * actionPanel.scale
                    Layout.preferredHeight: 14 * actionPanel.scale
                    source: "edit-undo"
                    color: Kirigami.Theme.textColor
                }

                PlasmaComponents.Label {
                    text: qsTr("Reset")
                    font.pixelSize: 11
                    color: Kirigami.Theme.textColor
                }
            }

            MouseArea {
                id: resetBtnMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: actionPanel.resetAllTiles()
            }
        }

        Item { Layout.fillWidth: true }

        // Edit quick settings (Pencil icon OR "✓ Selesai" button in editMode)
        Rectangle {
            Layout.preferredHeight: 28 * actionPanel.scale
            Layout.preferredWidth: actionPanel.editMode ? (doneBtnRow.implicitWidth + 20) : (28 * actionPanel.scale)
            radius: 6
            color: actionPanel.editMode
                ? Kirigami.Theme.highlightColor
                : (editMA.containsPress ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12)
                 : (editMA.containsMouse ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.06) : "transparent"))
            border.width: 1
            border.color: actionPanel.editMode
                ? "transparent"
                : (editMA.containsMouse ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12) : "transparent")

            Kirigami.Icon {
                visible: !actionPanel.editMode
                anchors.centerIn: parent
                width: 14 * actionPanel.scale
                height: 14 * actionPanel.scale
                source: "document-edit"
                color: Kirigami.Theme.textColor
                opacity: editMA.containsMouse ? 1.0 : 0.75
            }

            RowLayout {
                id: doneBtnRow
                visible: actionPanel.editMode
                anchors.centerIn: parent
                spacing: 5

                Kirigami.Icon {
                    Layout.preferredWidth: 14 * actionPanel.scale
                    Layout.preferredHeight: 14 * actionPanel.scale
                    source: "dialog-ok-apply"
                    color: ColorType.isDark(Kirigami.Theme.backgroundColor) ? "#1E1E1E" : "#FFFFFF"
                    isMask: true
                }

                PlasmaComponents.Label {
                    text: qsTr("Selesai")
                    font.pixelSize: 11
                    font.bold: true
                    color: ColorType.isDark(Kirigami.Theme.backgroundColor) ? "#1E1E1E" : "#FFFFFF"
                }
            }

            MouseArea {
                id: editMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    actionPanel.editMode = !actionPanel.editMode;
                }
            }

            PlasmaComponents.ToolTip {
                text: actionPanel.editMode
                    ? i18n("Selesai mengedit Quick Settings")
                    : i18n("Edit Quick Settings langsung di panel")
            }
        }

        // Open All Settings (gear icon)
        Rectangle {
            visible: !actionPanel.editMode
            width: 28 * actionPanel.scale
            height: 28 * actionPanel.scale
            radius: 6
            color: gearMA.containsPress ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12)
                 : (gearMA.containsMouse ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.06) : "transparent")
            border.width: 1
            border.color: gearMA.containsMouse ? Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12) : "transparent"

            Kirigami.Icon {
                anchors.centerIn: parent
                width: 14 * actionPanel.scale
                height: 14 * actionPanel.scale
                source: "configure"
                color: Kirigami.Theme.textColor
                opacity: gearMA.containsMouse ? 1.0 : 0.75
            }

            Plasma5Support.DataSource {
                id: settingsExec
                engine: "executable"
                connectedSources: []
                onNewData: function (sourceName, data) {
                    disconnectSource(sourceName);
                }
            }

            MouseArea {
                id: gearMA
                anchors.fill: parent
                hoverEnabled: true
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    settingsExec.connectSource("systemsettings &");
                }
            }

            PlasmaComponents.ToolTip {
                text: i18n("All settings")
            }
        }
    }
}

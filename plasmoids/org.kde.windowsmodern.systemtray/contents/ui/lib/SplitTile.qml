import QtQuick
import QtQuick.Layouts
import org.kde.plasma.core as PlasmaCore
import org.kde.kirigami as Kirigami
import org.kde.plasma.components as PlasmaComponents3
import "../js/colorType.js" as ColorType

ColumnLayout {
    id: tile

    property string iconSource
    property string label
    property string tooltipText: ""
    property bool active: false

    signal clicked
    signal arrowClicked
    signal rightClicked
    signal middleClicked

    activeFocusOnTab: true
    focus: true

    Keys.onReturnPressed: tile.clicked()
    Keys.onSpacePressed: tile.clicked()

    readonly property color accent: Kirigami.Theme.highlightColor
    readonly property color fg: active ? (ColorType.isDark(Kirigami.Theme.backgroundColor) ? "#1E1E1E" : "#FFFFFF") : Kirigami.Theme.textColor

    spacing: 4
    implicitWidth: 96
    implicitHeight: 68
    Layout.fillWidth: true
    Layout.preferredWidth: 96
    Layout.minimumWidth: 96
    Layout.maximumWidth: 96
    Layout.preferredHeight: 68
    Layout.minimumHeight: 68
    Layout.maximumHeight: 68

    Rectangle {
        id: bg
        Layout.fillWidth: true
        Layout.preferredHeight: 48
        Layout.minimumHeight: 48
        Layout.maximumHeight: 48
        height: 48
        radius: 6
        border.width: 1
        border.color: active ? "transparent" : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12)
        color: {
            if (active)
                return accent;
            if (toggleMA.containsPress || arrowMA.containsPress)
                return Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.12);
            if (toggleMA.containsMouse || arrowMA.containsMouse)
                return Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.08);
            return Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.05);
        }

        Behavior on color {
            ColorAnimation {
                duration: Kirigami.Units.shortDuration
            }
        }

        PlasmaCore.ToolTipArea {
            anchors.fill: parent
            mainText: tile.tooltipText
            subText: ""
            textFormat: Text.PlainText
        }

        MouseArea {
            id: toggleMA
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: parent.width * 0.68
            acceptedButtons: Qt.LeftButton | Qt.RightButton | Qt.MiddleButton
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: function (mouse) {
                if (mouse.button === Qt.RightButton) {
                    tile.rightClicked();
                } else if (mouse.button === Qt.MiddleButton) {
                    tile.middleClicked();
                } else {
                    tile.clicked();
                }
            }
        }

        MouseArea {
            id: arrowMA
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: parent.width * 0.32
            hoverEnabled: true
            cursorShape: Qt.PointingHandCursor
            onClicked: tile.arrowClicked()
        }

        Kirigami.Icon {
            x: parent.width * 0.34 - width / 2
            anchors.verticalCenter: parent.verticalCenter
            width: 20
            height: 20
            source: tile.iconSource
            color: tile.fg
            isMask: true
        }

        Rectangle {
            anchors.verticalCenter: parent.verticalCenter
            x: parent.width * 0.68
            width: 1
            height: parent.height * 0.5
            color: active ? Qt.rgba(1, 1, 1, 0.35) : Qt.rgba(Kirigami.Theme.textColor.r, Kirigami.Theme.textColor.g, Kirigami.Theme.textColor.b, 0.14)
        }

        Kirigami.Icon {
            anchors.right: parent.right
            anchors.rightMargin: 8
            anchors.verticalCenter: parent.verticalCenter
            width: 14
            height: 14
            source: "go-next"
            color: tile.fg
            isMask: true
        }
    }

    PlasmaComponents3.Label {
        Layout.fillWidth: true
        Layout.preferredWidth: 96
        Layout.maximumWidth: 96
        Layout.preferredHeight: 16
        Layout.alignment: Qt.AlignHCenter
        text: tile.label
        color: Kirigami.Theme.textColor
        font.pixelSize: 11
        elide: Text.ElideRight
        maximumLineCount: 1
        horizontalAlignment: Text.AlignHCenter
    }
}

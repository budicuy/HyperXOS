import QtQuick
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Clipboard")
    iconSource: "edit-paste-symbolic"
    active: false

    tooltipText: qsTr("Clipboard History")
}

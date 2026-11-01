import QtQuick
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Vaults")
    iconSource: "folder-locked-symbolic"
    active: false

    tooltipText: qsTr("Plasma Vaults — Encrypted Storage")
}

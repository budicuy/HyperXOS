import QtQuick
import "../lib" as Lib

Lib.SplitTile {
    id: tile

    label: qsTr("Weather")
    iconSource: "weather-clouds-symbolic"
    active: false

    tooltipText: qsTr("Weather Forecast")
}

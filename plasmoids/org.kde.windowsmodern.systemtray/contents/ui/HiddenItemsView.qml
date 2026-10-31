/*
    SPDX-FileCopyrightText: 2016 Marco Martin <mart@kde.org>
    SPDX-FileCopyrightText: 2020 Konrad Materka <materka@gmail.com>
    SPDX-FileCopyrightText: 2020 Nate Graham <nate@kde.org>

    SPDX-License-Identifier: LGPL-2.0-or-later
*/
pragma ComponentBehavior: Bound

import QtQuick

import org.kde.kirigami as Kirigami
import org.kde.plasma.components as PlasmaComponents3
import org.kde.plasma.extras as PlasmaExtras

PlasmaComponents3.ScrollView {
    id: hiddenTasksView

    property alias layout: hiddenTasks

    hoverEnabled: true
    background: null
    implicitHeight: hiddenTasks.calculatedHeight

    GridView {
        id: hiddenTasks

        readonly property int columns: 3
        readonly property real scale: Plasmoid.configuration.scale / 100
        readonly property int cellSpacing: 6 * scale

        cellWidth: Math.floor(hiddenTasksView.availableWidth / columns)
        cellHeight: Math.floor(cellWidth / 1.4)
        readonly property int rows: Math.ceil(itemCount / columns)
        readonly property real calculatedHeight: itemCount > 0 ? (rows * cellHeight + Kirigami.Units.smallSpacing) : 0
        implicitHeight: calculatedHeight

        currentIndex: -1
        highlight: PlasmaExtras.Highlight {}
        highlightMoveDuration: 0

        pixelAligned: true

        readonly property int itemCount: model.count

        model: root.hiddenModel
        delegate: ItemLoader {
            width: hiddenTasks.cellWidth - hiddenTasks.cellSpacing
            height: hiddenTasks.cellHeight - hiddenTasks.cellSpacing
            x: hiddenTasks.cellSpacing / 2
            y: hiddenTasks.cellSpacing / 2
            Accessible.role: Accessible.ListItem
        }

        keyNavigationEnabled: true
        activeFocusOnTab: true

        KeyNavigation.up: hiddenTasksView.KeyNavigation.up

        onActiveFocusChanged: {
            if (activeFocus && currentIndex === -1) {
                currentIndex = 0
            } else if (!activeFocus && currentIndex >= 0) {
                currentIndex = -1
            }
        }
    }
}

import QtQuick
import org.kde.plasma.plasmoid
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.kirigami as Kirigami
import "../lib" as Lib
import "../js/colorType.js" as ColorType

Lib.Tile {
    id: tile

    readonly property bool themeIsDark: ColorType.isDark(Kirigami.Theme.backgroundColor)
    property int overrideState: -1 // -1 = follow themeIsDark, 0 = light, 1 = dark
    readonly property bool darkMode: overrideState !== -1 ? (overrideState === 1) : themeIsDark

    onThemeIsDarkChanged: {
        overrideState = -1;
    }

    label: darkMode ? qsTr("Dark Mode") : qsTr("Light Mode")
    iconSource: darkMode ? "weather-clear-night-symbolic" : "weather-clear-symbolic"
    active: darkMode

    function applyHyperXTheme(toDark) {
        var lnf = toDark ? "org.hyperxos.fluent.dark" : "org.hyperxos.fluent.light";
        var colorScheme = toDark ? "HyperXFluentDark" : "HyperXFluentLight";
        var fallbackScheme = toDark ? "BreezeDark" : "BreezeLight";
        var desktopTheme = toDark ? "HyperX-Fluent-Dark" : "HyperX-Fluent-Light";
        var fallbackDesktop = toDark ? "breeze-dark" : "breeze-light";
        var iconTheme = toDark ? "Fluent-yellow-dark" : "Fluent-yellow-light";

        var cmd = "rm -f ~/.cache/icon-cache.kcache; "
                + "kwriteconfig6 --file kdeglobals --group KDE --key LookAndFeelPackage " + lnf + "; "
                + "(plasma-apply-colorscheme " + colorScheme + " || plasma-apply-colorscheme " + fallbackScheme + "); "
                + "(plasma-apply-desktoptheme " + desktopTheme + " || plasma-apply-desktoptheme " + fallbackDesktop + "); "
                + "kwriteconfig6 --file plasmarc --group Theme --key name " + desktopTheme + "; "
                + "(/usr/lib/plasma-changeicons " + iconTheme + " 2>/dev/null || kwriteconfig6 --file kdeglobals --group Icons --key Theme " + iconTheme + "); "
                + "qdbus6 org.kde.KWin /KWin reconfigure 2>/dev/null || true";
        colorschemeExec.exec(cmd);
    }

    onClicked: {
        var nextDark = !darkMode;
        overrideState = nextDark ? 1 : 0;
        applyHyperXTheme(nextDark);
    }

    onMiddleClicked: {
        var nextDark = !darkMode;
        overrideState = nextDark ? 1 : 0;
        applyHyperXTheme(nextDark);
    }

    tooltipText: darkMode ? qsTr("Dark Mode — Active (Click for Light Mode)") : qsTr("Light Mode — Active (Click for Dark Mode)")

    Plasma5Support.DataSource {
        id: colorschemeExec
        engine: "executable"
        connectedSources: []
        onNewData: function (sourceName, data) {
            disconnectSource(sourceName);
        }
        function exec(cmd) {
            connectSource(cmd);
        }
    }
}

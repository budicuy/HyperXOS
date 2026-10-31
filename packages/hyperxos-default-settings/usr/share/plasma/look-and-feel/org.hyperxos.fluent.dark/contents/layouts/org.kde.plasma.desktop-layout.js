var desktopsArray = desktopsForActivity(currentActivity());
for (var j = 0; j < desktopsArray.length; j++) {
    var d = desktopsArray[j];
    d.wallpaperPlugin = "org.kde.image";
    d.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];
    d.writeConfig("Image", "/usr/share/wallpapers/hyperxos-default.png");
}

// Recreate HyperXOS Windows 11 Centered Floating Panel if layout is applied
var panel = new Panel("org.kde.panel");
panel.location = "bottom";
panel.height = 48;
panel.floating = true;

var leftSpacer = panel.addWidget("org.kde.plasma.panelspacer");
leftSpacer.currentConfigGroup = ["General"];
leftSpacer.writeConfig("expanding", true);

var launcher = panel.addWidget("menu.11.enhanced");
launcher.currentConfigGroup = ["General"];
launcher.writeConfig("displayPosition", 2);
launcher.writeConfig("favoriteApps", "preferred://browser,applications:org.kde.dolphin.desktop,applications:hyper-x-tools.desktop,applications:systemsettings.desktop,applications:org.kde.konsole.desktop,applications:org.kde.discover.desktop,applications:org.kde.kwrite.desktop,applications:org.kde.kcalc.desktop,applications:org.kde.spectacle.desktop");
launcher.writeConfig("icon", "fluent-start");
launcher.writeConfig("numberColumns", 6);
launcher.writeConfig("numberRows", 3);
launcher.writeConfig("showAtCenter", true);
launcher.writeConfig("showRecentDocuments", true);

var tasks = panel.addWidget("org.kde.plasma.icontasks");
tasks.currentConfigGroup = ["General"];
tasks.writeConfig("groupedTaskVisualization", 1);
tasks.writeConfig("launchers", "applications:org.kde.dolphin.desktop,applications:firefox.desktop,applications:org.kde.konsole.desktop,applications:hyper-x-tools.desktop");

var rightSpacer = panel.addWidget("org.kde.plasma.panelspacer");
rightSpacer.currentConfigGroup = ["General"];
rightSpacer.writeConfig("expanding", true);

var systray = panel.addWidget("org.kde.windowsmodern.systemtray");
systray.currentConfigGroup = ["General"];
systray.writeConfig("extraItems", "org.kde.plasma.networkmanagement,org.kde.plasma.volume,org.kde.plasma.battery,org.kde.plasma.bluetooth");
systray.writeConfig("hiddenItems", "org.kde.plasma.volume,org.kde.plasma.brightness");
systray.writeConfig("shownItems", "org.kde.plasma.networkmanagement,org.kde.plasma.battery");

var clock = panel.addWidget("org.kde.plasma.digitalclock");
clock.currentConfigGroup = ["Appearance"];
clock.writeConfig("dateFormat", "shortDate");
clock.writeConfig("showDate", true);
clock.writeConfig("showSeconds", false);

panel.addWidget("org.kde.plasma.showdesktop");

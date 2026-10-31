// KWin script to horizontally center Kickoff start menu popup (Windows 11 Fluent style)
function center(w) {
    if (!w || !w.output) return;
    const screen = w.output.geometry;
    const geo = w.frameGeometry;
    if (!screen || !geo) return;
    const mid = geo.x + geo.width / 2;
    // Do not center system tray popups positioned on the far right (x > 82% of screen width)
    if (mid > screen.x + screen.width * 0.82) {
        return;
    }
    const targetX = Math.round(screen.x + (screen.width - geo.width) / 2);
    if (Math.abs(geo.x - targetX) < 2) {
        return;
    }
    w.frameGeometry = { x: targetX, y: geo.y, width: geo.width, height: geo.height };
}

function watch(w) {
    if (!w || !w.appletPopup) {
        return;
    }
    center(w);
    w.frameGeometryChanged.connect(function () {
        center(w);
    });
}

workspace.windowList().forEach(watch);
workspace.windowAdded.connect(watch);

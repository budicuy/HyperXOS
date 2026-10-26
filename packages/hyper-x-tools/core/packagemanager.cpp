#include "packagemanager.h"
#include <QDir>
#include <QFileInfo>
#include <QDebug>
#include <unistd.h>

PackageManager::PackageManager(QObject *parent)
    : QObject(parent), m_process(new QProcess(this)), m_isBusy(false)
{
    connect(m_process, &QProcess::readyReadStandardOutput, this, &PackageManager::onReadyReadStandardOutput);
    connect(m_process, &QProcess::readyReadStandardError, this, &PackageManager::onReadyReadStandardError);
    connect(m_process, QOverload<int, QProcess::ExitStatus>::of(&QProcess::finished),
            this, &PackageManager::onProcessFinished);

    initPackageDefinitions();
    refreshInstalledCache();
}

PackageManager::~PackageManager()
{
    if (m_process->state() != QProcess::NotRunning) {
        m_process->kill();
        m_process->waitForFinished(1000);
    }
}

void PackageManager::initPackageDefinitions()
{
    m_packageDefinitions.clear();

    // 1. Web Browsers
    m_packageDefinitions.append({
        "firefox", "Mozilla Firefox",
        "Peramban web open-source cepat, aman, dan kaya ekstensi privasi.",
        "browser", {"firefox"}, false
    });
    m_packageDefinitions.append({
        "chromium", "Chromium Web Browser",
        "Peramban web open-source berbasis Chromium/Blink engine.",
        "browser", {"chromium"}, false
    });
    m_packageDefinitions.append({
        "brave", "Brave Browser",
        "Peramban berorientasi privasi dengan ad-blocker dan perlindungan pelacak bawaan.",
        "browser", {"brave-bin"}, false
    });
    m_packageDefinitions.append({
        "chrome", "Google Chrome",
        "Peramban web populer resmi dari Google dengan integrasi akun Google penuh.",
        "browser", {"google-chrome"}, false
    });

    // 2. Office & Productivity
    m_packageDefinitions.append({
        "libreoffice", "LibreOffice Fresh Suite",
        "Paket aplikasi perkantoran lengkap (Writer, Calc, Impress, Draw, Math).",
        "office", {"libreoffice-fresh"}, false
    });
    m_packageDefinitions.append({
        "onlyoffice", "OnlyOffice Desktop Editors",
        "Editor dokumen modern dengan kompatibilitas tinggi format Microsoft Office.",
        "office", {"onlyoffice-bin"}, false
    });
    m_packageDefinitions.append({
        "okular", "Okular Document Viewer",
        "Penampil dokumen universal kaya fitur (PDF, ePub, DjVu) khas KDE.",
        "office", {"okular"}, false
    });
    m_packageDefinitions.append({
        "ark", "Ark Archive Manager",
        "Utilitas pengarsipan dan kompresi berkas grafis (ZIP, TAR, 7Z, RAR).",
        "office", {"ark"}, false
    });
    m_packageDefinitions.append({
        "kcalc", "KCalc Scientific Calculator",
        "Kalkulator ilmiah, statistik, dan finansial native KDE Plasma.",
        "office", {"kcalc"}, false
    });

    // 3. Development Suite & AI
    m_packageDefinitions.append({
        "base_devel", "Base Development Toolchain",
        "Kompiler C/C++ lengkap (GCC, Make, Autoconf, Automake, Bison, Flex, Patch, Fakeroot).",
        "dev", {"base-devel"}, false
    });
    m_packageDefinitions.append({
        "linux_headers", "Linux Kernel Headers",
        "File header kernel Linux untuk mengompilasi modul DKMS dan driver eksternal.",
        "dev", {"linux-headers"}, false
    });
    m_packageDefinitions.append({
        "net_tools", "Wget & Rsync Network Utilities",
        "Alat pengunduh berkas CLI (Wget) dan sinkronisasi file remote/lokal (Rsync).",
        "dev", {"wget", "rsync"}, false
    });
    m_packageDefinitions.append({
        "code_oss", "Code OSS (VS Code Open Source)",
        "Editor kode sumber modern berbasis VS Code versi murni open-source.",
        "dev", {"code"}, false
    });
    m_packageDefinitions.append({
        "vscode", "Visual Studio Code (Official)",
        "Editor resmi Microsoft VS Code dengan Marketplace ekstensi lengkap.",
        "dev", {"visual-studio-code-bin"}, false
    });
    m_packageDefinitions.append({
        "docker", "Docker & Docker Compose",
        "Platform virtualisasi kontainer aplikasi dan alat orkestrasi.",
        "dev", {"docker", "docker-compose"}, false
    });
    m_packageDefinitions.append({
        "neovim", "Neovim Modal Text Editor",
        "Editor teks terminal berbasis Vim generasi baru berkinerja ultra-tinggi.",
        "dev", {"neovim"}, false
    });
    m_packageDefinitions.append({
        "git", "Git Version Control System",
        "Sistem kontrol versi terdistribusi standar industri untuk pengembang.",
        "dev", {"git"}, false
    });
    m_packageDefinitions.append({
        "arch_install_scripts", "Arch Install Scripts",
        "Skrip instalasi dan pemeliharaan Arch Linux (pacstrap, arch-chroot, genfstab).",
        "dev", {"arch-install-scripts"}, false
    });
    m_packageDefinitions.append({
        "ollama", "Ollama AI Local Runtime",
        "Platform eksekusi model AI lokal (Llama 3, DeepSeek, Qwen, Mistral).",
        "dev", {"ollama"}, false
    });

    // 4. Gaming Suite
    m_packageDefinitions.append({
        "steam", "Steam Gaming Platform",
        "Platform distribusi game PC terbesar dengan dukungan Proton gaming Linux.",
        "gaming", {"steam"}, false
    });
    m_packageDefinitions.append({
        "lutris", "Lutris Game Manager",
        "Manajer game terbuka untuk game native Linux, Windows, Steam, dan emulator.",
        "gaming", {"lutris"}, false
    });
    m_packageDefinitions.append({
        "heroic", "Heroic Games Launcher",
        "Peluncur game native untuk Epic Games Store dan GOG di Linux.",
        "gaming", {"heroic-games-launcher-bin"}, false
    });
    m_packageDefinitions.append({
        "protonup", "ProtonUp-Qt",
        "Utilitas GUI untuk memasang dan mengelola Proton-GE dan Wine-GE kustom.",
        "gaming", {"protonup-qt"}, false
    });
    m_packageDefinitions.append({
        "wine", "Wine Staging & Winetricks",
        "Lapisan kompatibilitas untuk menjalankan aplikasi dan game Windows di Linux.",
        "gaming", {"wine-staging", "winetricks"}, false
    });
    m_packageDefinitions.append({
        "mangohud", "MangoHud Overlay & GameMode",
        "Overlay pemantau performa FPS, suhu hardware, dan pengoptimal GameMode.",
        "gaming", {"mangohud", "gamemode"}, false
    });

    // 5. System Monitoring & Diagnostics
    m_packageDefinitions.append({
        "btop", "Btop Resource Monitor",
        "Pemantau penggunaan CPU, RAM, disk, proses, dan jaringan bergaya modern.",
        "monitoring", {"btop"}, false
    });
    m_packageDefinitions.append({
        "fastfetch", "Fastfetch System Info",
        "Utilitas pengambil spesifikasi sistem berbasis C berkinerja tinggi.",
        "monitoring", {"fastfetch"}, false
    });
    m_packageDefinitions.append({
        "htop", "Htop Process Viewer",
        "Penampil proses sistem interaktif klasik berbasis terminal.",
        "monitoring", {"htop"}, false
    });
    m_packageDefinitions.append({
        "nvtop", "Nvtop GPU Task Monitor",
        "Monitor beban GPU real-time untuk grafis NVIDIA, AMD, dan Intel.",
        "monitoring", {"nvtop"}, false
    });
    m_packageDefinitions.append({
        "kinfocenter", "KInfoCenter (KDE)",
        "Pusat informasi hardware dan perangkat grafis native KDE Plasma.",
        "monitoring", {"kinfocenter"}, false
    });
    m_packageDefinitions.append({
        "inxi", "Inxi Hardware Reporter",
        "Pembuat laporan diagnostik hardware dan lingkungan desktop lengkap.",
        "monitoring", {"inxi"}, false
    });
    m_packageDefinitions.append({
        "reflector", "Reflector Mirror Ranking Tool",
        "Utilitas penguji dan pembaruan otomatis daftar mirror Arch Linux tercepat.",
        "monitoring", {"reflector"}, false
    });
    m_packageDefinitions.append({
        "snapper", "Snapper Btrfs Snapshot Manager",
        "Utilitas manajemen dan otomatisasi snapshot subvolume untuk sistem file Btrfs.",
        "monitoring", {"snapper"}, false
    });

    // 6. Printing & Scanning
    m_packageDefinitions.append({
        "cups", "CUPS Printing Stack",
        "Server dan sistem pencetakan standar untuk printer jaringan & USB.",
        "printing", {"cups", "cups-pdf"}, false
    });
    m_packageDefinitions.append({
        "hplip", "HP Printer Drivers (HPLIP)",
        "Driver dan pustaka resmi HP untuk printer dan scanner DeskJet/LaserJet.",
        "printing", {"hplip"}, false
    });
    m_packageDefinitions.append({
        "sane", "SANE Document Scanner",
        "Backend antarmuka pemindaian dokumen untuk berbagai scanner.",
        "printing", {"sane"}, false
    });
    m_packageDefinitions.append({
        "printer_gui", "Printer Settings GUI",
        "Panel grafis untuk menambah dan mengonfigurasi perangkat printer.",
        "printing", {"system-config-printer"}, false
    });

    // 7. Multimedia & Content Creation
    m_packageDefinitions.append({
        "vlc", "VLC Media Player",
        "Pemutar multimedia universal tangguh yang mendukung hampir semua format.",
        "multimedia", {"vlc"}, false
    });
    m_packageDefinitions.append({
        "mpv", "MPV Minimalist Player",
        "Pemutar video ultra-ringan dengan akselerasi hardware decoding penuh.",
        "multimedia", {"mpv"}, false
    });
    m_packageDefinitions.append({
        "obs", "OBS Studio",
        "Perangkat lunak perekam layar dan streaming video siaran langsung profesional.",
        "multimedia", {"obs-studio"}, false
    });
    m_packageDefinitions.append({
        "kdenlive", "Kdenlive Video Editor",
        "Editor video non-linear mutakhir dengan efek audio dan video lengkap.",
        "multimedia", {"kdenlive"}, false
    });
    m_packageDefinitions.append({
        "gimp", "GIMP Image Editor",
        "Editor gambar dan ilustrasi foto profesional alternatif open-source.",
        "multimedia", {"gimp"}, false
    });
    m_packageDefinitions.append({
        "blender", "Blender 3D Suite",
        "Aplikasi kreasi 3D terkemuka untuk pemodelan, animasi, dan rendering.",
        "multimedia", {"blender"}, false
    });
    m_packageDefinitions.append({
        "pw_jack", "PipeWire JACK Pro Audio",
        "Plugin protokol audio latency rendah untuk produksi musik dan aplikasi DAW (Ardour, Reaper).",
        "multimedia", {"pipewire-jack"}, false
    });
    m_packageDefinitions.append({
        "ocean_sounds", "KDE Ocean Sound Theme",
        "Koleksi efek suara interaksi desktop resmi untuk lingkungan KDE Plasma.",
        "multimedia", {"ocean-sound-theme"}, false
    });

    // 8. Social Media & Communication
    m_packageDefinitions.append({
        "telegram", "Telegram Desktop",
        "Aplikasi pesan instan cepat, aman, dan kaya fitur sinkronisasi cloud.",
        "social", {"telegram-desktop"}, false
    });
    m_packageDefinitions.append({
        "discord", "Discord Client",
        "Platform obrolan suara, video, dan teks untuk komunitas dan gamer.",
        "social", {"discord"}, false
    });
    m_packageDefinitions.append({
        "spotify", "Spotify Music Player",
        "Aplikasi resmi streaming musik dan podcast digital global.",
        "social", {"spotify"}, false
    });
}

void PackageManager::refreshInstalledCache()
{
    m_installedPackagesCache.clear();

    // 1. Blazing fast direct reading of pacman local db directory
    QDir localDir("/var/lib/pacman/local");
    if (localDir.exists()) {
        QStringList entries = localDir.entryList(QDir::Dirs | QDir::NoDotAndDotDot);
        for (const QString &entry : entries) {
            // Entry format: <pkgname>-<version>-<pkgrel>
            int lastDash = entry.lastIndexOf('-');
            if (lastDash > 0) {
                int secondLastDash = entry.lastIndexOf('-', lastDash - 1);
                if (secondLastDash > 0) {
                    QString pkgName = entry.left(secondLastDash);
                    m_installedPackagesCache.insert(pkgName);
                }
            }
        }
    }

    // 2. Fallback to pacman -Qq if cache is unexpectedly empty
    if (m_installedPackagesCache.isEmpty()) {
        QProcess proc;
        proc.start("pacman", QStringList() << "-Qq");
        if (proc.waitForFinished(3000)) {
            QString out = QString::fromUtf8(proc.readAllStandardOutput());
            const QStringList lines = out.split('\n', Qt::SkipEmptyParts);
            for (const QString &l : lines) {
                m_installedPackagesCache.insert(l.trimmed());
            }
        }
    }

    // Update package items status
    for (PackageItem &item : m_packageDefinitions) {
        item.isInstalled = isPackageInstalled(item.packages);
    }

    emit cacheRefreshed();
}

bool PackageManager::isPackageInstalled(const QStringList &packages) const
{
    if (packages.isEmpty()) return false;
    // Considered installed if primary package (packages[0]) is in cache
    return m_installedPackagesCache.contains(packages.first());
}

QList<PackageItem> PackageManager::getAllPackages() const
{
    return m_packageDefinitions;
}

QList<PackageItem> PackageManager::getPackagesByCategory(const QString &category) const
{
    QList<PackageItem> result;
    for (const PackageItem &item : m_packageDefinitions) {
        if (item.category == category) {
            result.append(item);
        }
    }
    return result;
}

QList<PackageItem> PackageManager::searchPackages(const QString &query, const QString &category) const
{
    QString q = query.trimmed().toLower();
    QList<PackageItem> result;
    for (const PackageItem &item : m_packageDefinitions) {
        if (!category.isEmpty() && item.category != category) {
            continue;
        }
        if (q.isEmpty() || item.name.toLower().contains(q) || item.description.toLower().contains(q) || item.id.toLower().contains(q)) {
            result.append(item);
        }
    }
    return result;
}

QStringList PackageManager::getCategories() const
{
    return {"browser", "office", "dev", "gaming", "monitoring", "printing", "multimedia", "social"};
}

QString PackageManager::getCategoryDisplayName(const QString &category) const
{
    if (category == "browser") return "Web Browsers";
    if (category == "office") return "Office & Produktivitas";
    if (category == "dev") return "Development & AI";
    if (category == "gaming") return "Gaming & Hiburan";
    if (category == "monitoring") return "Monitoring & Diagnostik";
    if (category == "printing") return "Printing & Scanning";
    if (category == "multimedia") return "Multimedia & Kreator";
    if (category == "social") return "Sosial & Komunikasi";
    return category;
}

void PackageManager::executeInstall(const QStringList &packagesToInstall)
{
    if (m_isBusy || packagesToInstall.isEmpty()) return;

    m_isBusy = true;
    m_currentOperation = "install";

    QStringList flatPackages;
    for (const QString &pkgKey : packagesToInstall) {
        for (const PackageItem &item : m_packageDefinitions) {
            if (item.id == pkgKey || item.packages.contains(pkgKey)) {
                flatPackages.append(item.packages);
                break;
            }
        }
    }
    flatPackages.removeDuplicates();

    emit processStarted(QString("Memasang %1 paket terpilih...").arg(flatPackages.size()));
    emit logReceived(QString("==> [HyperXTools] Memulai pemasangan paket: %1\n").arg(flatPackages.join(" ")));

    QString program;
    QStringList args;

    if (getuid() == 0) {
        program = "pacman";
        args << "-S" << "--needed" << "--noconfirm" << flatPackages;
    } else {
        program = "pkexec";
        args << "/usr/bin/pacman" << "-S" << "--needed" << "--noconfirm" << flatPackages;
    }

    m_process->start(program, args);
}

void PackageManager::executeUninstall(const QStringList &packagesToUninstall)
{
    if (m_isBusy || packagesToUninstall.isEmpty()) return;

    m_isBusy = true;
    m_currentOperation = "uninstall";

    QStringList flatPackages;
    for (const QString &pkgKey : packagesToUninstall) {
        for (const PackageItem &item : m_packageDefinitions) {
            if (item.id == pkgKey || item.packages.contains(pkgKey)) {
                // For uninstall, uninstall the primary package
                flatPackages.append(item.packages.first());
                break;
            }
        }
    }
    flatPackages.removeDuplicates();

    emit processStarted(QString("Menghapus %1 paket terpilih...").arg(flatPackages.size()));
    emit logReceived(QString("==> [HyperXTools] Memulai penghapusan paket: %1\n").arg(flatPackages.join(" ")));

    QString program;
    QStringList args;

    if (getuid() == 0) {
        program = "pacman";
        args << "-Rns" << "--noconfirm" << flatPackages;
    } else {
        program = "pkexec";
        args << "/usr/bin/pacman" << "-Rns" << "--noconfirm" << flatPackages;
    }

    m_process->start(program, args);
}

bool PackageManager::isBusy() const
{
    return m_isBusy;
}

void PackageManager::onReadyReadStandardOutput()
{
    QByteArray data = m_process->readAllStandardOutput();
    emit logReceived(QString::fromUtf8(data));
}

void PackageManager::onReadyReadStandardError()
{
    QByteArray data = m_process->readAllStandardError();
    emit logReceived(QString::fromUtf8(data));
}

void PackageManager::onProcessFinished(int exitCode, QProcess::ExitStatus exitStatus)
{
    m_isBusy = false;
    bool success = (exitCode == 0 && exitStatus == QProcess::NormalExit);

    if (success) {
        emit logReceived(QString("\n[✓] Operasi %1 berhasil diselesaikan!\n").arg(m_currentOperation));
        refreshInstalledCache();
        emit processFinished(true, QString("Operasi %1 berhasil!").arg(m_currentOperation));
    } else {
        emit logReceived(QString("\n[-] Operasi %1 gagal dengan kode keluar %2.\n").arg(m_currentOperation).arg(exitCode));
        emit processFinished(false, QString("Operasi %1 gagal (Exit code %2).").arg(m_currentOperation).arg(exitCode));
    }
}

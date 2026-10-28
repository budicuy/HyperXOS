# HyperXOS Linux

<p align="center">
  <strong>Distribusi Arch Linux Modern, Performa Tinggi, dan Siap Pakai dengan HyperX Desktop Environment (Pure Rust + Wayland) & Native Installer</strong>
</p>

---

## 🚀 Ringkasan Proyek

**HyperXOS** adalah distribusi sistem operasi independen berbasis Arch Linux (x86_64) yang dirancang untuk kebutuhan desktop harian, pengembang peranti lunak, dan gamer. HyperXOS menggabungkan model rilis bergulir (*rolling release*) terkini dengan desktop **Windows 11 Fluent Suite (Hyprland + Waybar + Wofi + SwayNC)**, penginstal grafis native **Pure Rust Installer (`hyper-x-installer`)**, bootloader modern **Limine**, serta arsitektur deteksi driver grafis otomatis (**NVIDIA Open**, driver legacy, Intel, dan AMD).

### Fitur Utama:
- **Desktop Modern Windows 11 Fluent Suite (Hyprland):**
  - Compositor Wayland modern bertenaga **Hyprland** dengan efek kaca Dual-Kawase Blur, sudut membulat, bayangan halus, dan animasi transisi cepat.
  - Floating Centered Taskbar & System Tray bertenaga **Waybar**.
  - Start Menu & Pencarian bergaya Windows 11 bertenaga **Wofi**.
  - Action Center & Notification Hub bertenaga **SwayNC**.
  - Shortcut Windows 11 native: `Win` (Start Menu), `Win + E` (Explorer), `Win + V` (Clipboard History), `Win + Shift + S` (Snipping Tool), `Win + L` (Lock Screen), `Alt + F4`, dan snapping jendela.
- **Pure Rust Native Installer (`hyper-x-installer`):**
  - Dibangun 100% menggunakan Rust murni dengan arsitektur UI Glassmorphic modern (Iced 0.14 + WGPU).
  - Skema partisi otomatis: Partisi boot ESP FAT32 sebesar 4096 MiB (4 GB) di `/boot` dan partisi root **Btrfs** dengan subvolume terpisah (`@`, `@home`, `@snapshots`, `@var`, `@opt`) dengan kompresi `zstd:1`.
  - Bootloader default eksklusif **Limine** dengan tema visual HyperXOS dan registrasi otomatis ke UEFI NVRAM.
  - Mode instalasi *Full Online* yang memasang paket dasar Arch/CachyOS dan desktop HyperX-DE terbaru langsung dari repositori resmi dalam satu transaksi atomic `pacstrap`.
  - Terminal eksekusi real-time terintegrasi untuk pemantauan proses transparan.
- **Deteksi Driver GPU Multi-Tier (`hyper-x-hw`):**
  - **NVIDIA Turing ke atas (RTX 20xx+, GTX 16xx+):** Modul kernel terbuka `nvidia-open` + early KMS (`nvidia_drm.modeset=1 nvidia_drm.fbdev=1`).
  - **NVIDIA Pascal & Maxwell (GTX 10xx, GTX 9xx):** Driver legacy prebuilt branch 580 (`nvidia-580xx-dkms` + `nvidia-580xx-utils`).
  - **NVIDIA Kepler & Lawas (GTX 6xx/7xx):** Driver open-source kernel `nouveau` + `mesa` untuk stabilitas Wayland.
  - **Intel & AMD:** Mesa + Vulkan native (RADV / ANV).
  - **Laptop Hybrid:** PRIME offloading (`prime-run`) dan dynamic power management.
  - **Mesin Virtual:** Auto-konfigurasi `qemu-guest-agent`.
- **Audio Modern:** PipeWire dan WirePlumber dengan applet kontrol suara terintegrasi.

---

## 📋 Kebutuhan Sistem Minimum (Pengguna Akhir)

| Komponen | Spesifikasi Minimum | Rekomendasi |
| :--- | :--- | :--- |
| **Prosesor** | 64-bit Dual-core (x86_64) | Quad-core modern atau lebih tinggi |
| **Memori (RAM)** | 2 GB RAM (Minimal) | 4 GB - 8 GB RAM atau lebih |
| **Penyimpanan** | 20 GB ruang kosong | 40 GB+ SSD / NVMe |
| **Kartu Grafis** | VGA controller dengan dukungan OpenGL 3.3+ | GPU Intel HD / AMD Radeon / NVIDIA GeForce |
| **Firmware** | UEFI (Direkomendasikan) atau Legacy BIOS | UEFI 64-bit (Secure Boot dimatikan saat instalasi) |

---

## 💻 Panduan Pengembangan (Development Guide Step-by-Step)

HyperXOS ditenagai oleh toolchain mandiri **100% Pure Rust Native** melalui runner **`./hyperx`**. Lingkungan pengujian virtual machine (`hyperx-vm`) telah dirancang **100% Self-Contained & Hermetis**, sehingga pengembang **tidak perlu menginstal QEMU, edk2-ovmf, atau libvirt di sistem host**.

### 1. Kebutuhan Sistem Pengembang (Host Requirements)
Sebelum memulai, pastikan komputer pengembang memenuhi kebutuhan dasar berikut:
* **Sistem Operasi:** Linux x86_64 apa saja (Arch Linux, CachyOS, Fedora, Ubuntu, Debian, dll.).
* **Virtualisasi Perangkat Keras CPU:** Akses ke node `/dev/kvm`.
  ```bash
  # Pastikan user Anda memiliki akses ke grup kvm:
  sudo usermod -aG kvm $USER
  ```
* **Rust Toolchain:** `rustc` dan `cargo` versi terbaru (instalasi standar via [rustup.rs](https://rustup.rs)):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* **Utilitas Dasar:** `curl`, `tar`, dan `zstd`.
* > [!NOTE]
  > **Zero Host QEMU Dependency:** Anda **TIDAK PERLU** menginstal paket QEMU atau firmware UEFI di sistem operasi host Anda. Seluruh binary VMM, driver grafis OpenGL, ROM BIOS, dan UEFI OVMF dipaketkan secara hermetis oleh `hyperx-vm`.

---

### 2. Langkah 1: Clone Repositori
Clone repositori HyperXOS ke direktori lokal Anda:
```bash
git clone https://github.com/budicuy/HyperXOS.git
cd HyperXOS
```

---

### 3. Langkah 2: Setup Awal & Validasi Lingkungan
Jalankan perintah setup dan check untuk memvalidasi kesiapan sistem:
```bash
# 1. Menyiapkan cache direktori dan modul loop kernel:
./hyperx setup

# 2. Memeriksa integritas virtualisasi, KVM, dan akselerasi GPU host:
./hyperx hyperx-vm check
```
*(Pada eksekusi pertama, skrip `./hyperx` akan otomatis mengompilasi toolchain CLI Rust dan mem-provisioning bundel VMM hermetis tanpa perlu konfigurasi manual).*

---

### 4. Langkah 3: Membangun ISO Development (Fast Build)
Untuk keperluan pengembangan sehari-hari, bangun ISO menggunakan mode development (menggunakan kompresi cepat `zstd-1` dan build-caching):
```bash
./hyperx build
```
Image ISO hasil kompilasi akan otomatis disimpan di dalam direktori `out/` (misalnya `out/hyperxos-YYYY.MM.DD-x86_64.iso`).

---

### 5. Langkah 4: Menjalankan Pengujian Virtual Machine (`hyperx-vm`)
HyperXOS menyediakan alur pengujian virtual machine sejati dengan akselerasi GPU host (NVIDIA/Intel/AMD OpenGL):

#### A. Uji Coba Instalasi Bersih Interaktif (Clean 40GB Virtual Disk)
Mereset disk virtual 40GB dan NVRAM, mem-boot Live ISO langsung ke antarmuka GUI installer `hyper-x-installer` untuk pengujian interaktif:
```bash
./hyperx vm install
```
*(atau shortcut: `./hyperx install`)*

#### B. Uji Coba Instalasi Otomatis Headless (CI/CD Automated Install Test)
Menjalankan pengujian instalasi end-to-end tanpa interaksi via QEMU Guest Agent:
```bash
./hyperx vm install --auto
```
*(atau shortcut: `./hyperx install --auto`)*

#### C. Uji Coba Booting Sistem Terinstall
Mem-boot sistem HyperXOS yang sudah terinstal di dalam disk virtual 40GB tanpa menggunakan file ISO:
```bash
./hyperx vm run
```
*(atau shortcut: `./hyperx run`)*

#### D. Pengujian Otomatis Non-Interactive (QA Smoke Test)
Menjalankan pengujian otomatis headless selama 25 detik untuk memvalidasi kernel booting dan konsol:
```bash
./hyperx test
```

---

### 6. Langkah 5: Membangun ISO Produksi (Production Build & Release)
Saat fitur selesai diuji dan siap dirilis ke publik, bangun ISO versi produksi resmi:

#### A. Kompilasi ISO Produksi (Batas Ketat < 2.0 GB & zstd-22)
```bash
./hyperx build --prod
# atau:
./hyperx production
```
*Mode produksi menerapkan optimasi kompresi maksimal (`zstd -22 --ultra`), initramfs terkompresi tinggi (`xz -9e`), dan validasi ketat ukuran file.*

#### B. Menghasilkan Checksum Kriptografis Resmi
Hasilkan berkas hash SHA256 dan BLAKE2b (B2SUM) untuk verifikasi integritas ISO:
```bash
./hyperx checksum
```
Berkas `SHA256SUMS` dan `B2SUMS` akan dibuat secara otomatis di direktori `out/`.

#### C. Mempersiapkan Release GitHub
Siapkan tag dan aset rilis:
```bash
./hyperx release
```

---

### 7. Langkah 6: Pembersihan Workspace (Cleanup)
Untuk membersihkan berkas build dan mount sementara:
```bash
# Membersihkan folder kerja sementara:
./hyperx clean

# Membersihkan seluruh file sementara, ISO lama, dan disk virtual:
./hyperx clean --all
```

---

## 🛠️ Referensi Perintah CLI (`./hyperx`)

### 1. Perintah Inti (Core Workflow)
| Perintah | Deskripsi |
| :--- | :--- |
| `./hyperx build` | Membangun ISO HyperXOS dalam mode development (cepat, default) |
| `./hyperx build --prod` | Membangun ISO HyperXOS dalam mode produksi resmi (< 2.0 GB, ultra-kompresi) |
| `./hyperx test` | Menjalankan pengujian QA otomatis (ISO 9660 header, SHA256, 25s headless smoke test) |
| `./hyperx clean` | Membersihkan folder build sementara |
| `./hyperx clean --all` | Membersihkan folder build, ISO lama di `out/`, dan disk virtual VM |

### 2. Lingkungan Virtualisasi (`hyperx vm`)
| Perintah | Deskripsi |
| :--- | :--- |
| `./hyperx vm install` *(atau `./hyperx install`)* | Menjalankan instalasi manual interaktif pada virtual disk 40 GB |
| `./hyperx vm install --auto` | Menjalankan pengujian instalasi 100% otomatis via QEMU Guest Agent |
| `./hyperx vm run` *(atau `./hyperx run`)* | Menjalankan sistem HyperXOS terinstal langsung dari virtual disk 40 GB |
| `./hyperx vm clean` | Membersihkan disk virtual dan socket runtime VM |

### 3. Repositori & Host
| Perintah | Deskripsi |
| :--- | :--- |
| `./hyperx check` | Memvalidasi kesiapan host (KVM hardware acceleration, mkarchiso, OVMF, PKGBUILDs) |
| `./hyperx setup` | Menyiapkan dependensi host dan direktori kerja |
| `./hyperx setup --cli` | Menyiapkan host sekaligus memasang launcher `hyperx` global ke `/usr/local/bin` |
| `./hyperx repo build` | Mengompilasi dan memaketkan seluruh paket Rust lokal (`--prod` untuk release) |
| `./hyperx repo sync` | Memperbarui database repositori `hyperxos.db` lokal dan menyinkronkan ke `airootfs` |
| `./hyperx repo check` | Memeriksa ketersediaan seluruh 9 paket resmi HyperXOS |

### 4. Peluncur Aplikasi Host & Rilis
| Perintah | Deskripsi |
| :--- | :--- |
| `./hyperx launch tools` | Membuka aplikasi GUI HyperXTools secara langsung di host |
| `./hyperx launch installer` | Membuka aplikasi GUI HyperXInstaller secara langsung di host |
| `./hyperx launch hw` | Menjalankan modul diagnostik hardware & GPU (`hyper-x-hw`) |
| `./hyperx release` | Mempersiapkan publikasi rilis GitHub (tagging, checksums, verifikasi ISO) |
| `./hyperx release --checksum-only` | Menghasilkan hash SHA256SUMS dan B2SUMS resmi untuk ISO di `out/` |

---

## 🔒 Catatan Keamanan & Secure Boot

- **Secure Boot:** Modul kernel terbuka NVIDIA (`nvidia-open`) dan DKMS tidak ditandatangani oleh Microsoft 3rd Party CA. Nonaktifkan Secure Boot di menu BIOS/UEFI sebelum mem-boot media USB HyperXOS. Jika ingin mengaktifkan Secure Boot pasca-instalasi, gunakan alat `sbctl` untuk mendaftarkan kunci kustom Anda ke database NVRAM.
- **Target Sanitasi:** Seluruh hak akses live tanpa password (`/etc/sudoers.d/00-hyperxos-live`) dan konfigurasi autologin SDDM dihapus secara otomatis dari sistem target saat instalasi selesai.

---

## 🙏 Ucapan Terima Kasih & Kredit (Acknowledgments)

HyperXOS tidak akan terwujud tanpa dedikasi luar biasa dari komunitas open-source upstream:
- **[Arch Linux](https://archlinux.org):** Fondasi distribusi Linux yang fleksibel, cepat, dan modern.
- **[CachyOS](https://cachyos.org):** Repositori teroptimasi performa tinggi (`x86_64_v3`).
- **[Hyprland](https://hyprland.org):** Wayland compositor dinamis modern dengan akselerasi GPU, animasi halus, dan efek kaca kustom.
- **[Waybar](https://github.com/Alexays/Waybar):** Highly customizable Wayland bar untuk taskbar dan system tray.
- **[Iced](https://iced.rs):** Cross-platform GUI library berbasis Rust dengan arsitektur reaktif dan akselerasi WGPU.
- **[Limine Bootloader](https://limine-bootloader.org):** Bootloader modern yang tangguh, ringan, dan cepat.

---

## 📄 Lisensi & Kontribusi

HyperXOS didistribusikan di bawah lisensi GPL-3.0. Seluruh komponen upstream Arch Linux dan Rust crates tunduk pada lisensi masing-masing.
Laporkan isu dan kontribusi melalui repositori GitHub resmi HyperXOS: [https://github.com/budicuy/HyperXOS](https://github.com/budicuy/HyperXOS).

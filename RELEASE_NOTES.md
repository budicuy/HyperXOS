# HyperXOS 2026.10.26 — Official Production Release

Selamat datang di rilis resmi pertama **HyperXOS** (`2026.10.26`).

HyperXOS adalah distribusi sistem operasi Linux modern, ringan, dan berorientasi performa tinggi yang dibangun di atas fondasi **Arch Linux** dan **CachyOS**, ditenagai oleh lingkungan desktop **Windows 11 Fluent Suite (Hyprland + Waybar + Wofi + SwayNC)**, antarmuka penginstal grafis **HyperXOS Native Installer (Pure Rust)**, serta utilitas sistem terintegrasi **HyperXTools (Pure Rust + Iced + WGPU)**.

---

## 🌟 Sorotan Fitur Utama (Highlights)

- **Fondasi Arch Linux & Kernel Terbaru:**
  - Mengadopsi arsitektur *rolling-release* dengan repositori resmi Arch Linux dan repositori teroptimasi CachyOS.
  - Manajemen paket berkecepatan tinggi dengan `pacman`.

- **HyperX Windows 11 Fluent Desktop (Hyprland):**
  - Sesi desktop Wayland bertenaga Hyprland dengan efek kaca Dual-Kawase Blur, sudut membulat, bayangan halus, dan animasi cepat.
  - Floating Centered Taskbar & System Tray bertenaga Waybar.
  - Start Menu & App Search bertenaga Wofi.
  - Action Center & Notification Hub bertenaga SwayNC.
  - Shortcut Windows 11 native: `Win` (Start Menu), `Win + E` (Explorer), `Win + V` (Clipboard), `Win + Shift + S` (Snipping Tool), `Win + L` (Lock Screen), `Alt + F4`, dan window snapping.

- **Peramban Web Firefox Terintegrasi:**
  - Mozilla Firefox langsung tersedia di media Live USB dan secara otomatis terpasang pada instalasi target.
  - Didaftarkan sebagai peramban web *default* di lingkungan desktop.

- **Installer Grafis Pure Rust Mandiri (`hyper-x-installer`):**
  - Dibangun 100% menggunakan Rust murni tanpa ketergantungan pada Python, C++, atau shell scripts.
  - Antarmuka visual modern bertema Glassmorphism menggunakan Iced 0.14 dan WGPU rendering.
  - Skema partisi standar terotomatisasi: Boot partition FAT32 4096 MiB (4 GB) di `/boot` dan Root partition Btrfs dengan subvolume terisolasi (`@` dan `@home`).
  - Bootloader default eksklusif **Limine** terdaftar langsung pada UEFI NVRAM.
  - Fresh online pacstrap langsung dari mirror upstream dalam satu transaksi tunggal.

- **HyperXTools (Pure Rust + Iced + WGPU):**
  - Aplikasi utilitas desktop eksklusif HyperXOS untuk pemantauan CPU/RAM/Disk, snapshot Btrfs/Timeshift, gaming tweaks, manajemen kernel CachyOS, dan app store package.

- **Dukungan Grafis & Driver:**
  - Driver grafis *open-source* Mesa 64-bit lengkap dengan akselerasi Vulkan: AMD (RADV), Intel (ANV), dan NVIDIA (NVK).
  - Modul pendeteksi perangkat keras & GPU otomatis Pure Rust (`hyper-x-hw`) untuk konfigurasi driver optimal.

- **Efisiensi & Ukuran ISO:**
  - Dibangun dengan kompresi produksi tingkat tinggi (SquashFS Zstandard level 22 & Initramfs XZ -9e) sehingga ukuran ISO tetap di bawah **2.0 GB**.

---

## 💾 Berkas Rilis & Checksum

Seluruh berkas ISO dan checksum dikompilasi secara otomatis dan bersih langsung di lingkungan CI/CD resmi GitHub Actions:

| Nama Berkas | Deskripsi | Verifikasi |
| :--- | :--- | :--- |
| `hyperxos-2026.10.26-x86_64.iso` | ISO Image Live & Instalasi Resmi (Production) | Checksum otomatis dari GitHub Runner |
| `hyperxos-2026.10.26-x86_64.iso.sha256` | Berkas Verifikasi Integritas SHA-256 | `sha256sum -c hyperxos-*.iso.sha256` |
| `hyperxos-2026.10.26-x86_64.iso.b2sum` | Berkas Verifikasi Integritas BLAKE2b | `b2sum -c hyperxos-*.iso.b2sum` |

### Cara Verifikasi Checksum

Sebelum melakukan flashing ke USB flashdisk, pastikan berkas ISO tidak mengalami korupsi data selama proses unduhan:

```bash
# Verifikasi SHA256
sha256sum -c hyperxos-2026.10.26-x86_64.iso.sha256

# Atau verifikasi BLAKE2b
b2sum -c hyperxos-2026.10.26-x86_64.iso.b2sum
```

Output harus menyatakan: `hyperxos-2026.10.26-x86_64.iso: OK`.

---

## 🚀 Panduan Pembuatan Media USB Bootable

Gunakan salah satu metode berikut untuk membuat USB installer:

### Metode 1: Menggunakan Perintah `dd` (Linux / macOS)
```bash
sudo dd if=hyperxos-2026.10.26-x86_64.iso of=/dev/sdX bs=4M status=progress oflag=sync
```
*(Ganti `/dev/sdX` dengan *device node* flashdisk Anda, pastikan bukan partisi seperti `sdX1`)*.

### Metode 2: Menggunakan Rufus (Windows)
1. Buka Rufus dan pilih flashdisk target.
2. Pilih berkas `hyperxos-2026.10.26-x86_64.iso`.
3. Skema partisi: **GPT** (untuk UEFI) atau **MBR** (untuk BIOS/Legacy).
4. Saat dialog peringatan muncul, pilih mode **Write in DD Image mode**.

### Metode 3: Menggunakan BalenaEtcher / Raspberry Pi Imager
- Pilih berkas ISO -> Pilih USB Drive -> Klik **Flash!**.

---

## ⚙️ Persyaratan Sistem Minimum & Rekomendasi

| Komponen | Spesifikasi Minimum | Rekomendasi |
| :--- | :--- | :--- |
| **Prosesor** | CPU 64-bit Dual-Core x86_64 | CPU 64-bit Quad-Core atau lebih tinggi |
| **Memori (RAM)** | 2 GB RAM (Mode teks/instalasi) | 4 GB+ RAM (Untuk pengalaman HyperX-DE optimal) |
| **Penyimpanan (Disk)** | 20 GB ruang kosong | 40 GB+ ruang penyimpanan SSD/NVMe |
| **Tampilan / GPU** | Resolusi 1024x768 | Resolusi 1920x1080 dengan akselerasi OpenGL/Vulkan |
| **Koneksi Jaringan** | Opsional (Tersedia mode offline) | Koneksi internet aktif saat instalasi online |

---

## 🔒 Catatan Keamanan & Secure Boot
- Nonaktifkan **Secure Boot** pada firmware BIOS/UEFI sebelum mem-boot media instalasi.
- Jika Anda ingin menggunakan Secure Boot setelah instalasi selesai, pasang dan gunakan utilitas `sbctl` untuk mendaftarkan kunci kustom Anda ke database NVRAM UEFI.

---

## 🤝 Kontribusi & Laporan Isu
Jika Anda menemukan bug, kendala perangkat keras, atau ingin berkontribusi:
- Repositori Utama: [https://github.com/budicuy/HyperXOS](https://github.com/budicuy/HyperXOS)
- Pelaporan Isu: [https://github.com/budicuy/HyperXOS/issues](https://github.com/budicuy/HyperXOS/issues)
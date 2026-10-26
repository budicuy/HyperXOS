# HyperXOS 2026.10.26 — Official Production Release

Selamat datang di rilis resmi pertama **HyperXOS** (`2026.10.26`).

HyperXOS adalah distribusi sistem operasi Linux modern, ringan, dan berorientasi performa tinggi yang dibangun di atas fondasi **Arch Linux** dan **CachyOS**, ditenagai oleh lingkungan desktop **KDE Plasma 6 (Wayland)**, antarmuka penginstal grafis **Calamares**, serta utilitas sistem terintegrasi **HyperXTools (Qt 6)**.

---

## 🌟 Sorotan Fitur Utama (Highlights)

- **Fondasi Arch Linux & Kernel Terbaru:**
  - Mengadopsi arsitektur *rolling-release* dengan repositori resmi Arch Linux dan repositori teroptimasi CachyOS.
  - Manajemen paket berkecepatan tinggi dengan `pacman` dan AUR helper `paru`.

- **KDE Plasma 6 Wayland Desktop:**
  - Sesi desktop Wayland murni yang responsif dan hemat sumber daya.
  - Skema warna gelap kustom *HyperXOS Dark* dengan integrasi Breeze Dark.
  - Shortcut taskbar telah terkonfigurasi: **Calamares**, **Firefox**, **Dolphin**, **Konsole**, dan **HyperXTools**.

- **Peramban Web Firefox Terintegrasi:**
  - Mozilla Firefox langsung tersedia di media Live USB dan secara otomatis terpasang pada instalasi target.
  - Didaftarkan sebagai peramban web *default* di lingkungan desktop.

- **Installer Grafis Calamares Mandiri:**
  - Disesuaikan khusus untuk HyperXOS dengan *branding* khas dan skema warna senada.
  - Menggunakan modul `pacstrap` dinamis untuk instalasi sistem bersih langsung dari repositori resmi.
  - Penanganan otomatis DNS fallback (`1.1.1.1`, `8.8.8.8`) dan inisialisasi pacman keyring di sistem target.

- **HyperXTools (C++ & Qt 6):**
  - Aplikasi utilitas desktop eksklusif HyperXOS untuk pemantauan CPU/RAM/Disk, pemeringkatan cermin repositori (Reflector ranking), pembersihan cache pacman, dan status keamanan sistem.

- **Dukungan Grafis & Driver:**
  - Driver grafis *open-source* Mesa 64-bit lengkap dengan akselerasi Vulkan: AMD (RADV), Intel (ANV), dan NVIDIA (NVK).
  - Skrip pendeteksi perangkat keras otomatis (`hw-detect-gpu`) untuk konfigurasi driver optimal.

- **Efisiensi & Ukuran ISO:**
  - Dibangun dengan kompresi produksi tingkat tinggi (SquashFS Zstandard level 22 & Initramfs XZ -9e) sehingga ukuran ISO tetap di bawah **2.0 GB**.

---

## 💾 Berkas Rilis & Checksum

| Nama Berkas | Deskripsi | Ukuran | Checksum SHA-256 |
| :--- | :--- | :--- | :--- |
| `hyperxos-2026.10.26-x86_64.iso` | ISO Image Live & Instalasi Resmi (Production) | **1678 MB** (1,760,542,720 bytes) | `5f20654d764c8a146c4e8b3e234e997c995e81f75c3d54cfdca172a1cdc9a88d` |
| `hyperxos-2026.10.26-x86_64.iso.sha256` | Verifikasi Integritas SHA256 | 97 B | — |
| `hyperxos-2026.10.26-x86_64.iso.b2sum` | Verifikasi Integritas BLAKE2b | 161 B | — |
| `hyperxos-2026.10.26-x86_64.pkglist` | Daftar Lengkap Paket Software | ~16 KB | — |

### Checksum Resmi
- **SHA-256:** `5f20654d764c8a146c4e8b3e234e997c995e81f75c3d54cfdca172a1cdc9a88d`
- **BLAKE2b:** `794844c9cbb8eed3e033fb47bea173fe43496678f983cbf0ceedd5b182941cab9747fa0628b6ee9b73fa93298bcaebff84067d08ab17f5b70ed898ae3ad3d7d8`

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
| **Memori (RAM)** | 2 GB RAM (Mode teks/instalasi) | 4 GB+ RAM (Untuk pengalaman KDE Plasma 6 optimal) |
| **Penyimpanan (Disk)** | 20 GB ruang kosong | 40 GB+ ruang penyimpanan SSD/NVMe |
| **Tampilan / GPU** | Resolusi 1024x768 | Resolusi 1920x1080 dengan akselerasi OpenGL/Vulkan |
| **Koneksi Jaringan** | Opsional (Tersedia mode offline) | Koneksi internet aktif saat instalasi Calamares online |

---

## 🔒 Catatan Keamanan & Secure Boot
- Nonaktifkan **Secure Boot** pada firmware BIOS/UEFI sebelum mem-boot media instalasi.
- Jika Anda ingin menggunakan Secure Boot setelah instalasi selesai, pasang dan gunakan utilitas `sbctl` untuk mendaftarkan kunci kustom Anda ke database NVRAM UEFI.

---

## 🤝 Kontribusi & Laporan Isu
Jika Anda menemukan bug, kendala perangkat keras, atau ingin berkontribusi:
- Repositori Utama: [https://github.com/budicuy/HyperXOS](https://github.com/budicuy/HyperXOS)
- Pelaporan Isu: [https://github.com/budicuy/HyperXOS/issues](https://github.com/budicuy/HyperXOS/issues)

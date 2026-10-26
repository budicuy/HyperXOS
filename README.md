# HyperXOS Linux

<p align="center">
  <strong>Distribusi Arch Linux Modern, Performa Tinggi, dan Siap Pakai dengan KDE Plasma 6 & Calamares Installer</strong>
</p>

---

## 🚀 Ringkasan Proyek

**HyperXOS** adalah distribusi sistem operasi independen berbasis Arch Linux (x86_64) yang dirancang untuk kebutuhan desktop harian, pengembang peranti lunak, dan gamer. HyperXOS menggabungkan model rilis bergulir (*rolling release*) terkini dengan desktop **KDE Plasma 6 (Wayland native)**, framework instalasi grafis **Calamares**, AUR helper **`paru`** bawaan, serta arsitektur deteksi driver grafis fleksibel (**NVIDIA Open**, driver legacy, Intel, dan AMD).

### Fitur Utama:
- **Desktop Modern:** KDE Plasma 6 berbasis Wayland dengan tema kustom *HyperXOS Dark*, akselerasi grafis terpadu, dan rendering responsif.
- **Installer Grafis Calamares:**
  - Skema partisi default **Btrfs** dengan subvolume terisolasi (`@`, `@home`, `@log`, `@cache`, `@snapshots`) dan kompresi transparan `zstd:1`.
  - Bootloader default **`systemd-boot`** (ESP di `/boot` berukuran 1024 MiB) dengan fallback otomatis ke **GRUB** untuk BIOS atau pilihan pengguna.
  - Alur pemilihan paket opsional (*Gaming Suite*, *Office & Productivity*, *Development Tools*, *Minimal*).
  - Alur pemilihan bootloader interaktif sebelum halaman ringkasan (*Summary*).
  - Sanitasi sistem target tuntas (penghapusan akun dan konfigurasi live).
- **Deteksi Driver GPU Multi-Tier (`hw-detect-gpu`):**
  - **NVIDIA Turing ke atas (RTX 20xx+, GTX 16xx+):** Modul kernel terbuka `nvidia-open` + early KMS (`nvidia_drm.modeset=1 nvidia_drm.fbdev=1`).
  - **NVIDIA Pascal & Maxwell (GTX 10xx, GTX 9xx):** Driver legacy prebuilt branch 580 (`nvidia-580xx-dkms` + `nvidia-580xx-utils`).
  - **NVIDIA Kepler & Lawas (GTX 6xx/7xx):** Driver open-source kernel `nouveau` + `mesa` untuk stabilitas Wayland.
  - **Intel & AMD:** Mesa + Vulkan native (RADV / ANV).
  - **Laptop Hybrid:** PRIME offloading (`prime-run`) dan dynamic power management.
  - **Mesin Virtual:** Auto-konfigurasi `qemu-guest-agent` dan `spice-vdagent`.
- **AUR Helper Bawaan:** `paru` terpasang langsung out-of-the-box lengkap dengan `base-devel` dan `linux-headers`.
- **Audio Modern:** PipeWire dan WirePlumber dengan applet kontrol suara terintegrasi.

---

## 📋 Kebutuhan Sistem Minimum

| Komponen | Spesifikasi Minimum | Rekomendasi |
| :--- | :--- | :--- |
| **Prosesor** | 64-bit Dual-core (x86_64) | Quad-core modern atau lebih tinggi |
| **Memori (RAM)** | 2 GB RAM (Minimal) | 4 GB - 8 GB RAM atau lebih |
| **Penyimpanan** | 20 GB ruang kosong | 40 GB+ SSD / NVMe |
| **Kartu Grafis** | VGA controller dengan dukungan OpenGL 3.3+ | GPU Intel HD / AMD Radeon / NVIDIA GeForce |
| **Firmware** | UEFI (Direkomendasikan) atau Legacy BIOS | UEFI 64-bit (Secure Boot dimatikan saat instalasi) |

---

## 🛠️ Panduan Build & Otomasi (`Makefile`)

HyperXOS menyediakan orkestrasi perintah Makefile yang konsisten:

```bash
# 1. Menampilkan seluruh perintah yang tersedia
make help

# 2. Memeriksa dependensi host dan konfigurasi /dev/kvm
make setup

# 3. Memvalidasi integritas konfigurasi profiledef, Calamares, dan paket
make check

# 4. Membangun ISO versi Development (kompresi cepat zstd level 3)
make build

# 5. Membangun ISO versi Produksi (kompresi ultra zstd level 22 & sanitasi tuntas)
make build-production

# 6. Menghasilkan checksum SHA256 dan B2SUM untuk ISO
make checksum

# 7. Menjalankan pengujian otomatis QA dan non-interactive smoke test
make test

# 8. Membersihkan direktori kerja build sementara
make clean
```

---

## 🖥️ Pengujian Virtualisasi (QEMU/KVM)

```bash
# Menjalankan ISO secara Live murni tanpa menginstal
make dev

# Menjalankan pengujian instalasi Calamares ke virtual disk 40 GB (disk otomatis di-reset bersih)
make install
# (atau 'make vm-install')

# Menjalankan sistem operasi yang sudah terinstal di dalam virtual disk 40 GB
make run
```

---

## 🔒 Catatan Keamanan & Secure Boot

- **Secure Boot:** Modul kernel terbuka NVIDIA (`nvidia-open`) dan DKMS tidak ditandatangani oleh Microsoft 3rd Party CA. Nonaktifkan Secure Boot di menu BIOS/UEFI sebelum mem-boot media USB HyperXOS. Jika ingin mengaktifkan Secure Boot pasca-instalasi, gunakan alat `sbctl` untuk mendaftarkan kunci kustom Anda ke database NVRAM.
- **Target Sanitasi:** Seluruh hak akses live tanpa password (`/etc/sudoers.d/00-hyperxos-live`) dan konfigurasi autologin SDDM dihapus secara otomatis dari sistem target saat instalasi selesai.

---

## 📄 Lisensi & Kontribusi

HyperXOS didistribusikan di bawah lisensi GPL-3.0. Seluruh komponen upstream Arch Linux dan KDE tunduk pada lisensi masing-masing.
Laporkan isu dan kontribusi melalui repositori GitHub resmi HyperXOS: [https://github.com/budicuy/HyperXOS](https://github.com/budicuy/HyperXOS).


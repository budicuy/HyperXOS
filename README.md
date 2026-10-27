# HyperXOS Linux

<p align="center">
  <strong>Distribusi Arch Linux Modern, Performa Tinggi, dan Siap Pakai dengan KDE Plasma 6 & Pure Rust Native Installer</strong>
</p>

---

## 🚀 Ringkasan Proyek

**HyperXOS** adalah distribusi sistem operasi independen berbasis Arch Linux (x86_64) yang dirancang untuk kebutuhan desktop harian, pengembang peranti lunak, dan gamer. HyperXOS menggabungkan model rilis bergulir (*rolling release*) terkini dengan desktop **KDE Plasma 6 (Wayland native)**, penginstal grafis native **Pure Rust Installer (`hyper-x-installer`)**, bootloader modern **Limine**, AUR helper **`paru`** bawaan, serta arsitektur deteksi driver grafis fleksibel (**NVIDIA Open**, driver legacy, Intel, dan AMD).

### Fitur Utama:
- **Desktop Modern:** KDE Plasma 6 berbasis Wayland dengan tema kustom *HyperXOS Dark*, akselerasi grafis terpadu, dan rendering responsif.
- **Pure Rust Native Installer (`hyper-x-installer`):**
  - Dibangun 100% menggunakan Rust murni dengan arsitektur UI Glassmorphic modern (Iced 0.14 + WGPU).
  - Skema partisi otomatis: Partisi boot ESP FAT32 sebesar 4096 MiB (4 GB) di `/boot` dan partisi root **Btrfs** dengan subvolume terpisah (`@` untuk root dan `@home` untuk user data) dengan kompresi `zstd:1`.
  - Bootloader default eksklusif **Limine** dengan tema visual HyperXOS dan registrasi otomatis ke UEFI NVRAM.
  - Mode instalasi *Full Online* yang memasang paket dasar Arch/CachyOS dan desktop KDE Plasma 6 terbaru langsung dari mirror dalam satu transaksi atomic `pacstrap`.
  - Terminal eksekusi real-time terintegrasi untuk pemantauan proses transparan.
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

## 🛠️ Panduan Developer Toolchain & Build (`hyperx` / `Makefile`)

HyperXOS menyediakan developer toolchain **100% Pure Rust Native (`hyperx`)** serta wrapper `Makefile` untuk orkestrasi perintah yang konsisten:

```bash
# 1. Menampilkan seluruh perintah yang tersedia
hyperx help
# (atau 'make help')

# 2. Memeriksa dependensi host dan konfigurasi /dev/kvm
hyperx setup
# (atau 'make setup')

# 3. Memvalidasi integritas konfigurasi profiledef dan paket
hyperx check
# (atau 'make check')

# 4. Membangun ISO versi Development (kompresi cepat zstd-1 & build caching)
hyperx build
# (atau 'make build')

# 5. Membangun ISO versi Produksi (kompresi ultra zstd-22, initramfs xz-9e, batas ketat <2.0GB)
hyperx build --prod
# (atau 'make build-production')

# 6. Menghasilkan checksum resmi SHA256 dan B2SUM untuk ISO
hyperx checksum
# (atau 'make checksum')

# 7. Menjalankan pengujian otomatis QA dan non-interactive smoke test QEMU
hyperx test
# (atau 'make test')

# 8. Membersihkan direktori kerja build sementara
hyperx clean
# (atau 'make clean' / 'hyperx clean --all')

# 9. Menjalankan modul pendeteksi hardware & GPU secara langsung
hyperx hw
# (atau 'make hw')

# 10. Menguji aplikasi GUI HyperXTools dan HyperXInstaller secara langsung
hyperx tools
hyperx installer
```

---

## 🖥️ Pengujian Virtualisasi (QEMU/KVM)

```bash
# Menjalankan ISO secara Live murni tanpa menginstal
hyperx dev
# (atau 'make dev')

# Menjalankan pengujian instalasi Native Installer ke virtual disk 40 GB (disk otomatis di-reset bersih)
hyperx install
# (atau 'make install' / 'hyperx vm-install')

# Menjalankan sistem operasi yang sudah terinstal di dalam virtual disk 40 GB
hyperx run
# (atau 'make run')
```

---

## 🔒 Catatan Keamanan & Secure Boot

- **Secure Boot:** Modul kernel terbuka NVIDIA (`nvidia-open`) dan DKMS tidak ditandatangani oleh Microsoft 3rd Party CA. Nonaktifkan Secure Boot di menu BIOS/UEFI sebelum mem-boot media USB HyperXOS. Jika ingin mengaktifkan Secure Boot pasca-instalasi, gunakan alat `sbctl` untuk mendaftarkan kunci kustom Anda ke database NVRAM.
- **Target Sanitasi:** Seluruh hak akses live tanpa password (`/etc/sudoers.d/00-hyperxos-live`) dan konfigurasi autologin SDDM dihapus secara otomatis dari sistem target saat instalasi selesai.

---

## 🙏 Ucapan Terima Kasih & Kredit (Acknowledgments)

HyperXOS tidak akan terwujud tanpa dedikasi luar biasa dari komunitas open-source upstream:
- **[Arch Linux](https://archlinux.org):** Fondasi distribusi Linux yang fleksibel, cepat, dan modern.
- **[CachyOS](https://cachyos.org):** Repositori teroptimasi performa tinggi (`x86_64_v3`).
- **[Limine Bootloader](https://limine-bootloader.org):** Bootloader modern yang tangguh, ringan, dan cepat.
- **[KDE Community](https://kde.org):** Lingkungan desktop KDE Plasma 6 Wayland yang elegan, modern, dan kaya fitur.

---

## 📄 Lisensi & Kontribusi

HyperXOS didistribusikan di bawah lisensi GPL-3.0. Seluruh komponen upstream Arch Linux dan KDE tunduk pada lisensi masing-masing.
Laporkan isu dan kontribusi melalui repositori GitHub resmi HyperXOS: [https://github.com/budicuy/HyperXOS](https://github.com/budicuy/HyperXOS).


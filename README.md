# HyperXOS Linux

<p align="center">
  <strong>Distribusi Arch Linux Modern, Performa Tinggi, dan Siap Pakai dengan KDE Plasma 6 & Pure Rust Native Installer</strong>
</p>

---

## 🚀 Ringkasan Proyek

**HyperXOS** adalah distribusi sistem operasi independen berbasis Arch Linux (x86_64) yang dirancang untuk kebutuhan desktop harian, pengembang peranti lunak, dan gamer. HyperXOS menggabungkan model rilis bergulir (*rolling release*) terkini dengan desktop **KDE Plasma 6 (Wayland native)**, penginstal grafis native **Pure Rust Installer (`hyper-x-installer`)**, bootloader modern **Limine**, serta arsitektur deteksi driver grafis fleksibel (**NVIDIA Open**, driver legacy, Intel, dan AMD).

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
HyperXOS menyediakan tiga alur pengujian virtual machine sejati dengan akselerasi GPU host (NVIDIA/Intel/AMD OpenGL):

#### A. Uji Coba Cepat Live ISO (Live Environment)
Mem-boot ISO secara langsung ke bootloader Limine UEFI -> systemd -> desktop KDE Plasma 6:
```bash
./hyperx dev
```

#### B. Uji Coba Instalasi Bersih (Clean 40GB Virtual Disk)
Mereset disk virtual 40GB dan NVRAM, mem-boot installer grafis `hyper-x-installer`, dan menguji partisi otomatis Btrfs serta instalasi online:
```bash
./hyperx install
```

#### C. Uji Coba Booting Sistem Terinstall
Mem-boot sistem HyperXOS yang sudah terinstal di dalam disk virtual 40GB tanpa menggunakan file ISO:
```bash
./hyperx run
```

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

## 🛠️ Referensi Perintah Lengkap CLI (`./hyperx`)

| Perintah | Deskripsi |
| :--- | :--- |
| `./hyperx help` | Menampilkan seluruh bantuan perintah CLI yang tersedia |
| `./hyperx setup` | Menyiapkan dependensi host, modul kernel loop, dan direktori cache |
| `./hyperx check` | Memvalidasi konfigurasi profiledef, berkas paket, dan integritas build |
| `./hyperx repo` | Membangun atau menyinkronkan repositori paket pacman lokal |
| `./hyperx build` | Membangun ISO HyperXOS dalam mode development (cepat) |
| `./hyperx build --prod` | Membangun ISO HyperXOS dalam mode produksi resmi (< 2.0 GB) |
| `./hyperx dev` | Menjalankan Live ISO di VM dengan akselerasi direct GPU host |
| `./hyperx install` | Menjalankan pengujian instalasi bersih pada virtual disk 40 GB |
| `./hyperx run` | Menjalankan sistem HyperXOS terinstal langsung dari virtual disk 40 GB |
| `./hyperx test` | Menjalankan 25-detik automated QA smoke test tanpa GUI |
| `./hyperx checksum` | Menghasilkan hash SHA256SUMS dan B2SUMS untuk ISO di `out/` |
| `./hyperx release` | Mempersiapkan publikasi rilis GitHub |
| `./hyperx hw` | Menjalankan modul diagnostik hardware & GPU (`hyper-x-hw`) |
| `./hyperx tools` | Membuka aplikasi GUI HyperXTools secara langsung di host |
| `./hyperx installer` | Membuka aplikasi GUI HyperXInstaller secara langsung di host |
| `./hyperx hyperx-vm check` | Memeriksa kesiapan kapabilitas virtualisasi dan GPU host |
| `./hyperx hyperx-vm provision`| Memaksa provisioning dan verifikasi hash bundel VMM hermetis |
| `./hyperx clean` | Membersihkan folder build sementara |
| `./hyperx clean --all` | Membersihkan folder build, ISO lama, dan disk virtual |

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

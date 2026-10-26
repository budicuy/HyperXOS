# HyperXOS Project Central Orchestration Makefile
# Defines build, configuration, testing, and virtualization targets.

.PHONY: help setup check build build-production test install vm-install run dev clean checksum release hyperx tools installer hw cli repo

SHELL := /usr/bin/env bash
PROJECT_DIR := $(shell pwd)
OUT_DIR := $(PROJECT_DIR)/out
SCRIPTS_DIR := $(PROJECT_DIR)/scripts
ARCHISO_DIR := $(PROJECT_DIR)/archiso
CLI_BIN := $(PROJECT_DIR)/rust/hyperx-cli/target/release/hyperx

# Default target: display help
help:
	@echo "======================================================================"
	@echo "                   HyperXOS Build & Automation System                 "
	@echo "======================================================================"
	@echo "Daftar Perintah (Targets) yang Tersedia:"
	@echo ""
	@echo "  make help             Menampilkan daftar perintah yang tersedia ini"
	@echo "  make setup            Memeriksa dan menyiapkan dependensi host"
	@echo "  make check            Memvalidasi konfigurasi, skrip, dan paket via Rust CLI"
	@echo "  make repo             Memperbarui database repositori paket lokal"
	@echo "  make build            Membangun ISO HyperXOS (Development mode: fast zstd-1)"
	@echo "  make build-production Membangun ISO HyperXOS (Production: zstd-22, clean release)"
	@echo "  make release          Mempersiapkan dan mempublikasikan rilis GitHub"
	@echo "  make test             Menjalankan pengujian otomatis QA"
	@echo "  make install          Menjalankan ISO ke virtual disk 40GB (reset otomatis)"
	@echo "  make vm-install       Alias untuk 'make install'"
	@echo "  make run              Menjalankan sistem terinstal dari virtual disk (QEMU)"
	@echo "  make dev              Menjalankan ISO secara live di QEMU tanpa install"
	@echo "  make clean            Membersihkan direktori kerja build sementara"
	@echo "  make checksum         Menghasilkan checksum resmi ISO (SHA256 & B2SUM)"
	@echo "  make tools            Menjalankan & menguji HyperXTools secara langsung"
	@echo "  make installer        Menjalankan & menguji HyperXInstaller secara langsung"
	@echo "  make hw               Menjalankan modul pendeteksi hardware (hyper-x-hw)"
	@echo "  make cli              Membangun binary CLI hyperx (Pure Rust)"
	@echo "======================================================================"

cli:
	@cargo build --release --manifest-path rust/hyperx-cli/Cargo.toml

setup:
	@echo "==> [HyperXOS] Memeriksa dependensi sistem host..."
	@which mkarchiso >/dev/null 2>&1 || (echo "[-] Paket 'archiso' belum terpasang. Jalankan: sudo pacman -S archiso" && exit 1)
	@which qemu-system-x86_64 >/dev/null 2>&1 || (echo "[-] Paket 'qemu-desktop' belum terpasang. Jalankan: sudo pacman -S qemu-desktop" && exit 1)
	@which xorriso >/dev/null 2>&1 || (echo "[-] Paket 'xorriso' belum terpasang. Jalankan: sudo pacman -S xorriso" && exit 1)
	@test -e /dev/kvm || echo "[!] Peringatan: Modul KVM (/dev/kvm) tidak ditemukan."
	@test -r /dev/kvm -a -w /dev/kvm && echo "[✓] Akselerasi /dev/kvm aktif dan dapat diakses." || echo "[!] Peringatan: /dev/kvm tidak writable oleh user."
	@echo "==> [HyperXOS] Memastikan izin direktori kerja dan cache..."
	@sudo install -d -m 0777 /var/tmp/hyperxos-work
	@mkdir -p out repo/x86_64 work
	@echo "[✓] Lingkungan pengembangan siap."

check: cli
	@$(CLI_BIN) check

repo: cli
	@$(CLI_BIN) repo

build: check
	@echo "==> [HyperXOS] Memulai proses kompilasi ISO (Mode Development)..."
	@$(SCRIPTS_DIR)/build-iso.sh dev

build-production: check
	@echo "==> [HyperXOS] Memulai proses kompilasi ISO (Mode Production)..."
	@$(SCRIPTS_DIR)/build-iso.sh prod
	@$(MAKE) checksum

production: build-production

release:
	@$(SCRIPTS_DIR)/publish-release.sh

test:
	@echo "==> [HyperXOS] Menjalankan pengujian otomatis QA..."
	@$(PROJECT_DIR)/tests/qemu-check.sh

install:
	@echo "==> [HyperXOS] Menjalankan mode instalasi QEMU dengan disk 40GB bersih..."
	@$(SCRIPTS_DIR)/run-qemu.sh install

vm-install: install

run:
	@echo "==> [HyperXOS] Menjalankan sistem operasi terinstal dari disk 40GB..."
	@$(SCRIPTS_DIR)/run-qemu.sh run

dev:
	@echo "==> [HyperXOS] Menjalankan ISO Live di QEMU..."
	@$(SCRIPTS_DIR)/run-qemu.sh dev

clean:
	@echo "==> [HyperXOS] Membersihkan artefak build sementara..."
	@$(SCRIPTS_DIR)/clean.sh

checksum:
	@echo "==> [HyperXOS] Menghitung checksum resmi ISO (SHA256 & B2SUM)..."
	@shopt -s nullglob; \
	for iso in $(OUT_DIR)/hyperxos-*.iso; do \
		echo "  -> SHA256: $$(basename $$iso)..."; \
		(cd $(OUT_DIR) && sha256sum "$$(basename $$iso)" > "$$(basename $$iso).sha256"); \
		echo "  -> B2SUM:  $$(basename $$iso)..."; \
		(cd $(OUT_DIR) && b2sum "$$(basename $$iso)" > "$$(basename $$iso).b2sum"); \
	done
	@echo "[✓] Pembuatan checksum selesai."

tools:
	@echo "==> [HyperXOS] Menjalankan HyperXTools secara langsung..."
	@cd rust/hyper-x-tools && cargo run --release

hyperx: tools

installer:
	@echo "==> [HyperXOS] Menjalankan HyperXInstaller secara langsung..."
	@cd rust/hyper-x-installer && cargo run --release

hw:
	@echo "==> [HyperXOS] Menjalankan Hardware & GPU Detector..."
	@cargo run --release --manifest-path rust/hyper-x-hw/Cargo.toml -- detect

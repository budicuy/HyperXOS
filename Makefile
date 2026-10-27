# HyperXOS Project Central Orchestration
# All build, validation, virtualization, and testing tasks are natively implemented
# in 100% Pure Rust via the 'hyperx' developer CLI (rust/hyperx-cli).
#
# You can run targets using either:
#   1. Native Rust CLI: hyperx <command>
#   2. Classic Make:    make <command>

.PHONY: help setup check repo build build-production production install vm-install run dev test clean checksum release tools hyperx installer hw cli install-cli

SHELL := /usr/bin/env bash
PROJECT_DIR := $(shell pwd)
CLI_BIN := $(PROJECT_DIR)/rust/hyperx-cli/target/release/hyperx

all: help

help: cli
	@$(CLI_BIN) --help

cli:
	@test -x $(CLI_BIN) || cargo build --release --manifest-path rust/hyperx-cli/Cargo.toml

setup check repo build build-production production install vm-install run dev test clean checksum release tools hyperx installer hw: cli
	@$(CLI_BIN) $@

install-cli: cli
	@$(CLI_BIN) install-cli

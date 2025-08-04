# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Relfa is a Rust-based command-line file archival tool that helps users manage digital clutter by automatically organizing old files from an "Inbox" into a structured "Graveyard" archive. It features dual-threshold scanning (soft/hard limits), interactive review, and symlink-based organization.

## Development Environment

This project uses **Nix + Direnv** as the official development environment. The user will drop you in an enviroment where `direnv allow` has been executed and you have at your disposal a complete development environment with Rust toolchain, Git hooks, and all dependencies.

### Available Commands (via devshell)
- `build` - Run cargo build
- `test` - Run cargo test  
- `check` - Run cargo check
- `fmt` - Format all files with treefmt
- `install-hooks` - Install git hooks (auto-installed on devshell entry)

### Direct Cargo Commands
- `cargo build` - Build the project
- `cargo test --verbose` - Run tests (matches CI)
- `cargo clippy --all-targets --all-features -- -D warnings` - Lint (matches CI)
- `cargo fmt --all -- --check` - Check formatting (matches CI)

## Architecture

### Core Components
- **main.rs** - CLI entry point using clap parser
- **cli.rs** - Command-line interface definitions and subcommands
- **commands.rs** - Business logic for all CLI commands (scan, review, archive, etc.)
- **config.rs** - Configuration management with TOML serialization
- **scanner.rs** - File scanning logic with age thresholds
- **archiver.rs** - File archival with symlink organization
- **graveyard.rs** - Archive management, search, and resurrection
- **utils.rs** - Utility functions for file operations

### Key Data Flow
1. **Scanner** reads inbox directory and identifies stale files based on age thresholds
2. **Archiver** moves files to graveyard with date-based organization and symlinks
3. **GraveyardManager** handles search, resurrection, and archive navigation

### Configuration System
- TOML-based config at `~/.config/relfa/config.toml`
- Dual thresholds: `age_threshold_days` (soft) and `auto_archive_threshold_days` (hard)
- Complex path format system supporting original files + symlink views by creation/modification/archive date

## Pre-commit Hooks

Git hooks automatically run on commit:
- `rustfmt` - Code formatting check
- `cargo-clippy` - Linting with warnings as errors
- `cargo-test` - Full test suite
- `cargo-lock` - Regenerate Cargo.lock when Cargo.toml changes

## Home Manager Integration

The project includes a Nix Home Manager module (`modules/home-manager.nix`) for declarative configuration and systemd timer automation.

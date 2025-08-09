# Changelog

## 0.4.1 - 2025-08-09
- Fixed notification system hanging when notification daemon is down

## 0.4.0 - 2025-08-09
- Added minimum notification count requirement before auto-archival
- Added single key press support for review actions
- Added help menu in review mode
- Added touch command in review mode to update file timestamps

## 0.3.0 - 2025-08-09
- Major UX improvements for interactive review mode

## 0.2.8 - 2025-08-04
- Added automated GitHub releases with cross-platform binaries
- Added git hook to automatically update Cargo.lock when Cargo.toml changes
- Fixed Linux binary compatibility by using musl instead of glibc
- Added pre-compiled binaries for Linux x86_64, macOS Intel, and macOS Apple Silicon

## 0.2.6 - 2025-08-04
- Fixed CI workflow to properly handle publish outputs and create GitHub releases

## 0.2.5 - 2025-08-04
- Fixed `relfa config` command output errors
- Fixed Home Manager module settings validation
- Fixed directory timestamp scanning
- Fixed TOML null value serialization
- Added Nix devshell and pre-commit hooks

## 0.2.4 - 2025-08-04
- Enhanced cross-device file operations
- Improved read-only filesystem compatibility

## 0.2.3 - 2025-08-04
- Added Nix flake integration

## 0.2.2 - 2025-08-03
- Fixed cross-device archiving errors
- Fixed symlink handling across filesystems
- Fixed directory permission issues

## 0.2.1 - 2025-08-03
- Fixed systemd timer compatibility
- Added Nix flake with Home Manager module
- Added read-only environment support

## 0.2.0 - 2025-08-03
- Added auto-archive threshold feature
- Added `--no-auto-archive` flag
- Reduced default age threshold from 14 to 3 days
- Enhanced scan output with contextual guidance

## 0.1.2 - 2025-08-03
- Fixed CI workflows and publishing pipeline

## 0.1.1 - 2025-08-03
- Fixed clippy warnings and CI issues

## 0.1.0 - 2025-08-03
- Initial release
- File scanning and interactive review
- Flexible archival system with multiple views
- Search and resurrection functionality
- Epitaph system for file notes
- Desktop integration and notifications
- Cross-platform support

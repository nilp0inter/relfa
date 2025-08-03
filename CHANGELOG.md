# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2025-08-03

### Fixed

- **Cross-device archiving**: Fixed "Invalid cross-device link" error when archiving files across different filesystems
- **Relative symlinks**: All symlinks now use relative paths instead of absolute paths to prevent cross-device issues
- **Directory archiving with symlinks**: Enhanced directory copying to properly handle broken symlinks and Windows-style symlinks
- **Permission handling**: Added robust permission fixing for directories that can't be removed due to restrictive permissions

### Technical

- Added `calculate_relative_path()` method for computing relative symlink targets
- Enhanced `move_item()` with cross-device fallback using copy+remove
- Improved `copy_dir_all()` with proper symlink preservation
- Added `remove_dir_with_permissions()` and `fix_permissions_recursive()` for handling permission issues
- Cross-platform symlink handling for both Unix and Windows

## [0.2.1] - 2025-08-03

### Fixed

- **Systemd timer compatibility**: Fixed config loading to never attempt writing config files during read-only operations (scan, archive commands)
- **Read-only environment support**: Commands now work properly in systemd services and other read-only contexts

### Added

- **Nix flake with Home Manager module**: Complete Nix flake implementation with packages and home-manager integration
- **Systemd timer support**: Home Manager module includes systemd timer configuration for automated execution
- **Multiple package outputs**: Binary and man page are now separate outputs following Nix best practices
- **Comprehensive Nix documentation**: Added detailed Nix usage section to README with examples

### Technical

- Added `Config::load_without_save()` method for read-only config loading
- Updated all non-config commands to use read-only config loading
- Implemented flake-file architecture with modular Nix configuration
- Added systemd user service and timer definitions
- Package derivation now includes man page installation with proper outputs

## [0.2.0] - 2025-08-03

### Added

- **Auto-archive threshold feature**: New `auto_archive_threshold_days` configuration option
- **Smart archive command**: `relfa archive` without arguments now automatically archives files older than the auto-archive threshold
- **Auto-archive prevention**: New `--no-auto-archive` flag to disable automatic archiving behavior
- **Enhanced scan warnings**: Scan command now warns about files eligible for auto-archiving
- **Improved user guidance**: Contextual tips in scan output based on file age categories

### Changed

- **Reduced default thresholds**: Age threshold reduced from 14 to 3 days, auto-archive threshold set to 7 days
- **Simplified scan output**: Consolidated and contextualized action suggestions to reduce verbosity
- **Updated documentation**: README and man page updated with auto-archive feature documentation

### Technical

- Added `scan_auto_archive_eligible()` method to Scanner
- Added `auto_archive_eligible_files()` command function
- Enhanced CLI argument parsing for new flag
- Improved configuration display to show both thresholds

## [0.1.2] - 2025-08-03

### Fixed

- CI workflow improvements and simplification
- Code formatting and clippy warning resolution
- GitHub Actions workflow syntax corrections
- Publishing pipeline fixes

### Changed

- Streamlined CI/CD workflows for better reliability

## [0.1.1] - 2025-08-03

### Fixed

- Resolved clippy warnings and formatting issues
- CI pipeline fixes and improvements

## [0.1.0] - 2025-08-03

### Added

- **Initial release** of Relfa - Your gentle digital gravedigger
- **File scanning**: Detect stale files in Inbox based on configurable age threshold
- **Interactive review**: Process files one by one with multiple action options:
  - Archive with or without notes (epitaphs)
  - Delete with safety confirmation
  - View file contents with configurable pager
  - Open files with default application
  - Skip files for later processing
- **Flexible archival system**: Organized graveyard structure with multiple views:
  - Organization by creation, modification, and archival dates
  - Efficient symlink-based storage to prevent duplication
  - Configurable subdirectory structure
- **Batch operations**: Archive all stale files at once with optional notes
- **Search and resurrection**: Find and restore archived files
  - Pattern-based search in filenames and epitaph content
  - Easy restoration back to Inbox (files remain in graveyard)
- **Epitaph system**: Add explanatory notes when archiving files
  - Searchable content for better file organization
  - Metadata tracking (timestamps, hostname)
- **Desktop integration**:
  - Desktop notifications support
  - File manager integration for opening files
  - Configurable pager support for viewing files
- **Comprehensive configuration**: TOML-based configuration with:
  - Customizable inbox and graveyard paths
  - Adjustable age thresholds
  - Flexible graveyard organization patterns
  - Notification preferences
- **Cross-platform support**: Works on Linux, macOS, and Windows
- **Documentation**: Complete README, man page, and inline help

### Technical Features

- Built in Rust for performance and reliability
- TOML configuration with sensible defaults
- Symlink-based storage for space efficiency
- Pattern matching for file search
- Safe file operations with confirmation prompts
- Comprehensive error handling
- CI/CD pipeline with automated testing and publishing

[0.2.1]: https://github.com/nilp0inter/relfa/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/nilp0inter/relfa/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/nilp0inter/relfa/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/nilp0inter/relfa/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nilp0inter/relfa/releases/tag/v0.1.0

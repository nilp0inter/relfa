# Relfa - GitHub Copilot Instructions

**ALWAYS follow these instructions first and only search for additional context if the information here is incomplete or found to be in error.**

Relfa is a Rust-based command-line file archival tool that helps users manage digital clutter by automatically organizing old files from an "Inbox" into a structured "Graveyard" archive. It features dual-threshold scanning (soft/hard limits), interactive review, and symlink-based organization.

## Working Effectively

### Bootstrap and Build
Run these commands to set up and build the repository:

```bash
# Standard Rust approach (works in any environment)
cargo build  # NEVER CANCEL: Takes ~2 minutes first time with dependencies, ~30 seconds clean rebuild. Set timeout to 5+ minutes.
```

**Alternative Nix Development Environment** (if available):
```bash
# Nix + Direnv approach (official development setup)
direnv allow  # Sets up complete development environment with Rust toolchain and git hooks
build         # Devshell alias for cargo build
```

### Testing and Validation
**CRITICAL: This repository has NO unit tests.** Always validate changes manually:

```bash
cargo test --verbose  # NEVER CANCEL: Takes <5 seconds. Set timeout to 30+ seconds. Returns "0 tests" - this is normal.
```

**Manual Validation is Required** - Always test actual functionality after changes:
```bash
# Create test environment
mkdir -p /tmp/relfa-test/{inbox,graveyard}
echo "test content" > /tmp/relfa-test/inbox/old-file.txt
touch -d "5 days ago" /tmp/relfa-test/inbox/old-file.txt

# Configure relfa for testing
mkdir -p ~/.config/relfa
cat > ~/.config/relfa/config.toml << 'EOF'
inbox = "/tmp/relfa-test/inbox"
graveyard = "/tmp/relfa-test/graveyard"
age_threshold_days = 3
auto_archive_threshold_days = 7
auto_archive_min_scans = 1
notification = "cli"
pager = "less"

[path_format]
date_format = "{hostname}/{year}/{month:02}/{day:02}"

[path_format.created_subdir]
type = "original"
name = "created"

[path_format.modified_subdir]
type = "symlink"
name = "modified"
target = "created"

[path_format.archived_subdir]
type = "symlink"
name = "archived"
target = "created"
EOF

# Test core functionality
./target/debug/relfa --help           # Verify application runs
./target/debug/relfa config           # Verify config system
./target/debug/relfa scan             # Verify file scanning
./target/debug/relfa archive old-file.txt  # Verify archiving
./target/debug/relfa search old       # Verify search functionality
```

### Linting and Formatting
Always run these before committing (matches CI exactly):

```bash
cargo clippy --all-targets --all-features -- -D warnings  # NEVER CANCEL: Takes ~20 seconds. Set timeout to 60+ seconds.
cargo fmt --all -- --check                                 # NEVER CANCEL: Takes <1 second. Set timeout to 30+ seconds.
```

**Alternative with Nix devshell:**
```bash
fmt  # Runs treefmt (includes rustfmt + other formatters)
```

## Validation Scenarios

**ALWAYS run through at least one complete end-to-end scenario after making changes:**

### Basic File Archival Workflow
1. Create test files with different ages in inbox
2. Run `relfa scan` to see stale files detection
3. Run `relfa archive <filename>` to archive specific files
4. Run `relfa search <term>` to find archived files
5. Verify symlink structure in graveyard directory

### Interactive Review Workflow
1. Run `relfa review` to test interactive mode
2. Test each action: (a)rchive, (n)ote+archive, (t)ouch, (s)kip, (q)uit
3. Verify files are processed correctly

## Architecture Overview

### Core Components
- **src/main.rs** - CLI entry point using clap parser
- **src/cli.rs** - Command-line interface definitions and subcommands  
- **src/commands.rs** - Business logic for all CLI commands (scan, review, archive, etc.)
- **src/config.rs** - Configuration management with TOML serialization
- **src/scanner.rs** - File scanning logic with age thresholds
- **src/archiver.rs** - File archival with symlink organization
- **src/graveyard.rs** - Archive management, search, and resurrection
- **src/utils.rs** - Utility functions for file operations

### Key Data Flow
1. **Scanner** reads inbox directory and identifies stale files based on age thresholds
2. **Archiver** moves files to graveyard with date-based organization and symlinks  
3. **GraveyardManager** handles search, resurrection, and archive navigation

### Configuration System
- TOML-based config at `~/.config/relfa/config.toml`
- Dual thresholds: `age_threshold_days` (soft) and `auto_archive_threshold_days` (hard)
- Complex path format system supporting original files + symlink views by creation/modification/archive date

## Development Environment Options

### Standard Rust (Always Available)
```bash
cargo build   # Build the project
cargo test    # Run tests (returns 0 tests - this is normal)
cargo check   # Quick compilation check
cargo clippy  # Linting
cargo fmt     # Code formatting
```

### Nix + Direnv (Official Setup)
```bash
direnv allow     # One-time setup, installs everything including git hooks
build           # cargo build
test            # cargo test  
check           # cargo check
fmt             # treefmt (formats Rust, TOML, and Nix files)
install-hooks   # Install git hooks (auto-installed on devshell entry)
```

## Git Hooks and CI

Pre-commit hooks automatically run (if using Nix setup):
- `rustfmt` - Code formatting check
- `cargo-clippy` - Linting with warnings as errors  
- `cargo-test` - Test suite (though no tests exist)
- `cargo-lock` - Regenerate Cargo.lock when Cargo.toml changes

CI pipeline (.github/workflows/ci.yml) runs:
1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings` 
3. `cargo test --verbose`

## Common Tasks

### Building a Release Binary
```bash
cargo build --release  # NEVER CANCEL: Takes ~1 minute. Set timeout to 5+ minutes.
```

### Adding Dependencies
```bash
# Edit Cargo.toml to add dependencies
cargo build  # This will download and compile new dependencies
```

### Configuration Changes
Always test configuration changes with:
```bash
./target/debug/relfa config  # Verify config loads correctly
# Run manual validation scenario to test functionality
```

## Important Files

### Repository Structure
```
├── src/                    # Rust source code
├── .github/workflows/      # CI configuration
├── modules/               # Nix module definitions
├── Cargo.toml            # Rust project configuration
├── flake.nix             # Nix flake configuration
├── README.md             # User documentation
├── CLAUDE.md             # Development guidance
└── CHANGELOG.md          # Version history
```

### Key Configuration Files
- `Cargo.toml` - Rust dependencies and metadata
- `.github/workflows/ci.yml` - CI pipeline configuration
- `modules/devshell.nix` - Development environment setup
- `modules/git-hooks.nix` - Pre-commit hook configuration

## Troubleshooting

### Build Issues
- **Long compile times**: Normal on first build (~2 minutes). Subsequent builds are much faster.
- **Dependency errors**: Run `cargo clean && cargo build` to rebuild from scratch.

### Runtime Issues
- **Config not found**: Run `./target/debug/relfa config` to generate default config.
- **Permission errors**: Ensure test directories are writable.

### Testing Issues
- **No tests found**: This is normal - the repository currently has no unit tests.
- **Manual testing required**: Always validate functionality using the provided test scenarios.

## Critical Reminders

- **NEVER CANCEL builds or long-running commands** - Set appropriate timeouts (5+ minutes for builds, 60+ seconds for clippy)
- **NO unit tests exist** - Manual validation is required for all changes
- **Always test CLI functionality** - Build success does not guarantee runtime correctness
- **Two development environments available** - Nix/Direnv (official) or standard Rust (always works)
- **Configuration is mandatory** - The application requires valid TOML config to function
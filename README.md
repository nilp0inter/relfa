# 🪦 Relfa - Your Gentle Digital Gravedigger

<div align="center">

![Relfa Banner](https://via.placeholder.com/800x200/2C3E50/ECF0F1?text=🪦+Relfa+-+Digital+File+Management+with+Love)

**Keep your computer's clutter under control with loving care**

[![GitHub release](https://img.shields.io/github/v/release/nilp0inter/relfa?style=flat-square)](https://github.com/nilp0inter/relfa/releases)
[![Crates.io](https://img.shields.io/crates/v/relfa?style=flat-square)](https://crates.io/crates/relfa)
[![CI](https://img.shields.io/github/actions/workflow/status/nilp0inter/relfa/ci.yml?branch=main&style=flat-square)](https://github.com/nilp0inter/relfa/actions)
[![Coverage](https://img.shields.io/codecov/c/github/nilp0inter/relfa?style=flat-square)](https://codecov.io/gh/nilp0inter/relfa)
[![License](https://img.shields.io/github/license/nilp0inter/relfa?style=flat-square)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/relfa?style=flat-square)](https://crates.io/crates/relfa)

[📦 Installation](#-installation) • [🚀 Quick Start](#-quick-start) • [✨ Features](#-features) • [📚 Documentation](#-documentation) • [🤝 Contributing](#-contributing)

_"In Relfa's Graveyard, nothing is truly lost—just waiting in gentle slumber for you, or the next digital archaeologist."_

</div>

---

## 🌟 What is Relfa?

Relfa is your **gentle digital gravedigger** that helps you maintain a clean workspace without the anxiety of permanently losing important files. Instead of aggressively deleting old files, Relfa lovingly archives them to a well-organized digital graveyard where they can rest in peace—and be easily resurrected when needed.

### 🎯 Philosophy

- **🕊️ Gentle Approach**: No aggressive deletions, just loving archival
- **🔍 Smart Detection**: Identifies files that haven't been touched recently
- **🗃️ Organized Storage**: Creates a structured graveyard with multiple time-based views
- **🧟‍♂️ Easy Resurrection**: Bring back archived files when you need them
- **📝 Memory Keeping**: Add epitaphs explaining why files were archived
- **🔗 Efficient Links**: Uses symlinks to provide multiple organizational views without file duplication

## ✨ Features

<table>
<tr>
<td width="50%">

### 🎮 **Interactive Review**

- Multi-action interface: archive, delete, view, open, skip
- Safety confirmations for destructive actions
- Real-time file preview with configurable pager
- Desktop application integration

### 🗃️ **Flexible Archival System**

- Smart organization by creation/modification/archival dates
- Efficient symlink-based storage
- Configurable graveyard structure
- Cross-platform support (Linux, macOS, Windows)

</td>
<td width="50%">

### 🔍 **Powerful Search**

- Filename pattern matching
- Content search within epitaph notes
- Visual indicators for match sources
- Smart deduplication

### 📱 **Modern UX**

- Desktop notifications
- Configurable pager support
- Rich emoji-enhanced CLI output
- Comprehensive man page

</td>
</tr>
</table>

## 🚀 Installation

### 📦 **Quick Install**

<table>
<tr>
<td><strong>🦀 Cargo</strong></td>
<td><code>cargo install relfa</code></td>
</tr>
<tr>
<td><strong>🍺 Homebrew</strong></td>
<td><code>brew install relfa</code></td>
</tr>
<tr>
<td><strong>🐧 Arch Linux</strong></td>
<td><code>yay -S relfa</code></td>
</tr>
<tr>
<td><strong>🐳 Docker</strong></td>
<td><code>docker run --rm -v $(pwd):/workspace ghcr.io/nilp0inter/relfa:latest scan</code></td>
</tr>
</table>

### 📥 **Pre-built Binaries**

Download the latest release for your platform from [GitHub Releases](https://github.com/nilp0inter/relfa/releases):

```bash
# Linux (x86_64)
curl -L https://github.com/nilp0inter/relfa/releases/latest/download/relfa-linux-x86_64.tar.gz | tar xz
sudo mv relfa /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/nilp0inter/relfa/releases/latest/download/relfa-macos-x86_64.tar.gz | tar xz
sudo mv relfa /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/nilp0inter/relfa/releases/latest/download/relfa-macos-aarch64.tar.gz | tar xz
sudo mv relfa /usr/local/bin/

# Windows
# Download relfa-windows-x86_64.zip and extract to your PATH
```

### 📖 **Man Page Installation**

```bash
sudo cp relfa.1 /usr/local/share/man/man1/
sudo mandb
man relfa  # View the manual
```

## 🏃‍♂️ Quick Start

### 1. 🔧 **Initial Setup**

```bash
# View current configuration
relfa config

# Scan your inbox for stale files
relfa scan
```

<details>
<summary>📋 Expected Output</summary>

```
🕷️  Scanning Inbox for dusty files...
☠️  3 items in ~/Inbox are gathering dust:
   📄 "old-document.pdf" (file) - last touched 5 days ago (2025-07-29)
   📄 "project-archive/" (folder) - last touched 4 days ago (2025-07-30)
   📄 "temp-notes.txt" (file) - last touched 10 days ago (2025-07-24)

🤖 1 item is eligible for auto-archiving (older than 7 days):
   📄 "temp-notes.txt" (file) - last touched 10 days ago (2025-07-24)
   ⚠️  These will be automatically archived if you run 'relfa archive' without arguments!

💡 Run 'relfa review' to interactively deal with these items
   or 'relfa archive' to auto-archive old files (or 'relfa archive --all' for all).
```

</details>

### 2. 🔍 **Interactive Review**

```bash
relfa review
```

The interactive review gives you options for each file:

| Command              | Action                  | Description                                |
| -------------------- | ----------------------- | ------------------------------------------ |
| **`(a)rchive`**      | 🗃️ Archive              | Move to graveyard without note             |
| **`(n)ote+archive`** | 📝 Archive with epitaph | Add explanatory note                       |
| **`(d)elete`**       | 🗑️ Delete               | Permanent deletion (requires confirmation) |
| **`(v)iew`**         | 👀 View                 | Preview content with pager                 |
| **`(o)pen`**         | 🚀 Open                 | Open with default application              |
| **`(s)kip`**         | ⏭️ Skip                 | Skip for now                               |
| **`(q)uit`**         | 🚪 Quit                 | Exit review session                        |

### 3. 📦 **Batch Operations**

```bash
# Archive all stale files at once
relfa archive --all

# Archive specific file with explanation
relfa archive old-document.pdf --note "Outdated project specification from Q1"

# Archive with batch note
relfa archive --all --note "Weekly cleanup - $(date +%Y-%m-%d)"
```

### 🤖 **Auto-Archive Feature**

Relfa can automatically archive files that exceed the auto-archive threshold when you run `relfa archive` without arguments:

```bash
# Automatically archive files older than auto_archive_threshold_days (default: 7 days)
relfa archive

# Disable auto-archiving (shows help message instead)
relfa archive --no-auto-archive

# Auto-archive with a note
relfa archive --note "Automated cleanup - $(date +%Y-%m-%d)"
```

The scan command will warn you about files eligible for auto-archiving:

```
🕷️  Scanning Inbox for dusty files...
☠️  2 items in ~/Inbox are gathering dust:
   📄 "document.pdf" (file) - last touched 5 days ago (2025-07-29)
   📄 "old-file.txt" (file) - last touched 10 days ago (2025-07-24)

🤖 1 item is eligible for auto-archiving (older than 7 days):
   📄 "old-file.txt" (file) - last touched 10 days ago (2025-07-24)
   ⚠️  These will be automatically archived if you run 'relfa archive' without arguments!
```

### 5. 🔍 **Search & Resurrection**

```bash
# Search in graveyard (searches filenames AND epitaph content)
relfa search "project"

# Search by epitaph content
relfa search "meeting notes"

# Resurrect files back to inbox
relfa resurrect "important-file"
```

<details>
<summary>📋 Search Output Example</summary>

```
🔍 Searching for 'project' in the Graveyard...
Found 2 matches in the Graveyard:
  📄 created/laptop/2024/07/15/project-alpha-notes.md
     💭 "Meeting notes from project alpha kickoff"
  📄 created/laptop/2024/06/20/project-proposal.pdf
     💭 "Initial proposal document - archived after approval" 🔍
```

The 🔍 emoji indicates the match was found in the epitaph content, not the filename.

</details>

## 🏗️ Graveyard Structure

Relfa creates an elegantly organized graveyard structure:

```
~/Graveyard/
├── created/hostname/2024/08/02/     # Organized by creation date
│   ├── document.pdf                 # 📄 Original file
│   └── document.pdf.epitaph         # 📝 Optional note
├── modified/hostname/2024/07/15/    # Organized by modification date
│   ├── document.pdf@ → ../created/  # 🔗 Symlink to original
│   └── document.pdf.epitaph@ →      # 🔗 Symlink to epitaph
└── archived/hostname/2024/08/02/    # Organized by archival date
    ├── document.pdf@ → ../modified/ # 🔗 Symlink chain
    └── document.pdf.epitaph@ →      # 🔗 Follows same pattern
```

This structure provides:

- 📅 **Browse by creation date**: Find files by when they were originally created
- ✏️ **Browse by modification date**: See files by when they were last changed
- 🗃️ **Browse by archival date**: Review recently archived items
- 💾 **Efficient storage**: Symlinks prevent duplication while providing multiple views
- 🔍 **Consistent organization**: Epitaphs follow the same symlink structure as files

## ⚙️ Configuration

Relfa uses a TOML configuration file at `~/.config/relfa/config.toml`:

```toml
# Basic settings
inbox = "/home/user/Inbox"
graveyard = "/home/user/Graveyard"
age_threshold_days = 3               # Files older than this show as "stale"
auto_archive_threshold_days = 7      # Files older than this auto-archive when running 'relfa archive'
notification = "desktop"             # "desktop" or "cli"
pager = "less"                      # "less", "bat", "more", etc.

[path_format]
date_format = "{hostname}/{year}/{month:02}/{day:02}"

# Flexible subdirectory configuration
[path_format.created_subdir]
type = "original"    # Contains actual files
name = "created"

[path_format.modified_subdir]
type = "symlink"     # Contains symlinks
name = "modified"
target = "created"   # Points to created subdir

[path_format.archived_subdir]
type = "symlink"
name = "archived"
target = "modified"  # Creates chain: archived → modified → created
```

### 🎛️ **Advanced Configuration Options**

<details>
<summary>🔧 Subdirectory Types</summary>

Each subdirectory can be configured as:

- **`original`** - Contains the actual files
- **`symlink`** - Contains symlinks pointing to another subdirectory
- **`nothing`** - Disabled (not created)

Example configurations:

```toml
# Minimal: Only organize by creation date
[path_format.created_subdir]
type = "original"
name = "by-creation"

[path_format.modified_subdir]
type = "nothing"

[path_format.archived_subdir]
type = "nothing"
```

```toml
# Complex chain: created → modified → archived
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
target = "modified"
```

</details>

<details>
<summary>📅 Date Format Options</summary>

The `date_format` supports these placeholders:

- `{hostname}` - Computer hostname
- `{year}` - Full year (2024)
- `{month}` - Month number (8)
- `{month:02}` - Zero-padded month (08)
- `{day}` - Day number (15)
- `{day:02}` - Zero-padded day (15)

Examples:

```toml
date_format = "{year}/{month:02}/{day:02}"                    # 2024/08/15
date_format = "{hostname}/{year}-{month:02}"                  # laptop/2024-08
date_format = "archive-{year}-{month:02}-{day:02}"           # archive-2024-08-15
```

</details>

## ❄️ Nix Flake Usage

Relfa provides a comprehensive Nix flake with packages and Home Manager modules for declarative configuration and automation.

### 📦 **Package Installation**

#### Direct Usage

```bash
# Run relfa directly from the flake
nix run github:nilp0inter/relfa

# Install to user profile
nix profile install github:nilp0inter/relfa

# Use in a development shell
nix shell github:nilp0inter/relfa
```

#### In a Nix System Configuration

```nix
{
  inputs.relfa.url = "github:nilp0inter/relfa";

  # In your system configuration
  environment.systemPackages = [ inputs.relfa.packages.${system}.relfa ];
}
```

### 🏠 **Home Manager Integration**

#### Basic Configuration

```nix
{
  inputs.relfa.url = "github:nilp0inter/relfa";

  # In your home.nix:
  imports = [ inputs.relfa.homeManagerModules.relfa ];

  programs.relfa = {
    enable = true;
    settings = {
      age_threshold_days = 5;
      auto_archive_threshold_days = 14;
      notification = "desktop";
      inbox = "${config.home.homeDirectory}/Downloads";
      graveyard = "${config.home.homeDirectory}/Archive";
    };
  };
}
```

### ⏰ **Automated Execution with Systemd Timer**

#### Hourly Scan and Auto-Archive (Recommended)

```nix
programs.relfa = {
  enable = true;

  # Configure desktop notifications and settings
  settings = {
    notification = "desktop";
    age_threshold_days = 3;
    auto_archive_threshold_days = 7;
    inbox = "${config.home.homeDirectory}/Inbox";
    graveyard = "${config.home.homeDirectory}/Graveyard";
  };

  # Systemd timer configuration
  timer = {
    enable = true;
    frequency = "hourly";                # Run every hour
    command = "scan-then-archive";       # First scan, then auto-archive
    randomizedDelay = "15m";            # Random delay up to 15 minutes
  };
};
```

#### Daily Scan Only (Non-Destructive)

```nix
programs.relfa = {
  enable = true;
  settings.notification = "desktop";

  timer = {
    enable = true;
    frequency = "daily";
    command = "scan";                   # Only scan and notify
    randomizedDelay = "1h";
  };
};
```

#### Custom Schedule Examples

```nix
timer = {
  enable = true;

  # Every 30 minutes
  frequency = "*:0/30";

  # Every 6 hours
  frequency = "0/6:00:00";

  # Weekdays at 9 AM
  frequency = "Mon..Fri 09:00:00";

  # Daily at 2 PM with 2-hour random delay
  frequency = "daily";
  randomizedDelay = "2h";
};
```

### 🎛️ **Configuration Options**

<details>
<summary>📋 Complete Configuration Reference</summary>

```nix
programs.relfa = {
  enable = true;

  # Package override (optional)
  package = inputs.relfa.packages.${pkgs.system}.relfa;

  # Core settings
  settings = {
    inbox = "${config.home.homeDirectory}/Inbox";
    graveyard = "${config.home.homeDirectory}/Graveyard";
    age_threshold_days = 3;              # Files show as "stale"
    auto_archive_threshold_days = 7;     # Files auto-archive
    notification = "desktop";            # "cli" or "desktop"
    pager = "bat";                      # File viewer command

    # Path format configuration (optional - uses defaults if not specified)
    path_format = {
      date_format = "{hostname}/{year}/{month:02}/{day:02}";
      
      created_subdir = {
        type = "original";
        name = "created";
      };
      
      modified_subdir = {
        type = "symlink";
        name = "modified";
        target = "created";
      };
      
      archived_subdir = {
        type = "symlink";
        name = "archived";
        target = "modified";
      };
    };
  };

  # Systemd timer (optional)
  timer = {
    enable = true;
    frequency = "daily";                # systemd OnCalendar format
    command = "scan-then-archive";      # "scan", "archive", "scan-then-archive"
    randomizedDelay = "1h";            # Prevent simultaneous execution
  };
};
```

</details>

### 🔔 **Notification Examples**

#### What You'll See with Timer Enabled:

- **Scan results**: `🔍 Scan Complete: 2 files gathering dust in Inbox`
- **Auto-archive warnings**: `🤖 Auto-archived 1 file to Graveyard (old-document.pdf)`
- **Clean state**: `✨ No files need attention - Inbox is clean!`

### 🚀 **Getting Started with Nix**

1. **Add relfa to your flake inputs:**

   ```nix
   inputs.relfa.url = "github:nilp0inter/relfa";
   ```

2. **Import the Home Manager module:**

   ```nix
   imports = [ inputs.relfa.homeManagerModules.relfa ];
   ```

3. **Enable with basic configuration:**

   ```nix
   programs.relfa.enable = true;
   ```

4. **Rebuild your system:**

   ```bash
   home-manager switch  # For home-manager
   # or
   nixos-rebuild switch  # For NixOS
   ```

5. **Check timer status:**
   ```bash
   systemctl --user status relfa.timer
   systemctl --user status relfa.service
   ```

The Nix flake provides a **zero-configuration** setup that works out of the box, with full **declarative control** over all settings and automation!

## 📝 Epitaphs - Digital Memory Keeping

Epitaphs are optional notes that explain why files were archived, creating a historical record of your digital archaeology:

### ✍️ **Adding Epitaphs**

```bash
# During archival
relfa archive old-logs/ --note "Log files from completed project - kept for reference"

# During interactive review
relfa review
# Choose (n)ote+archive and enter your explanation

# Batch archival with note
relfa archive --all --note "End of semester cleanup - keeping for historical reference"
```

### 📋 **Epitaph Format**

Epitaphs are stored as `.epitaph` files containing structured metadata and your note:

```
# Epitaph for project-notes.md
# Archived: 2024-08-02 15:30:45 UTC
# Created: 2024-01-15 09:22:10 UTC
# Modified: 2024-07-28 16:45:33 UTC
# Hostname: workstation

Project planning notes from Q1 2024. Archived after project completion
but keeping for historical reference and lessons learned documentation.
Contains meeting notes, timeline, and final retrospective.
```

### 🔍 **Searching Epitaphs**

Epitaph content is fully searchable:

```bash
# Find files by epitaph content
relfa search "meeting notes"
relfa search "Q1 2024"
relfa search "retrospective"

# Case-insensitive search
relfa search "PROJECT"  # Finds "project" in epitaphs
```

## 🎨 Usage Examples

### 📊 **Daily Workflow**

```bash
#!/bin/bash
# Daily cleanup routine

echo "🧹 Daily digital cleanup started"

# Check for new clutter
relfa scan

# Interactive review (optional)
echo "Starting interactive review..."
relfa review

# Quick batch cleanup for everything else
relfa archive --all --note "Daily cleanup - $(date +%Y-%m-%d)"

echo "✨ Cleanup complete!"
```

### 🔍 **Finding Archived Content**

```bash
# Search by filename patterns
relfa search "presentation"
relfa search "*.pdf"
relfa search "2024"

# Search by epitaph content
relfa search "project alpha"
relfa search "meeting notes"
relfa search "semester"

# Find recently archived items
relfa search "$(date +%Y-%m-%d)"  # Today's archives
```

### 🧟‍♂️ **File Resurrection**

```bash
# Resurrect specific files
relfa resurrect "important-document"

# Interactive selection for multiple matches
relfa resurrect "presentation"
# Found 3 matches:
#   1. presentation-draft.pptx
#   2. final-presentation.pptx
#   3. presentation-notes.md
# Which file to resurrect? (1-3, or 'q' to quit): 2

# Files are copied back to Inbox (originals remain in graveyard)
```

### 🤖 **Automation Examples**

<details>
<summary>📜 Weekly Cleanup Script</summary>

```bash
#!/bin/bash
# weekly-cleanup.sh

WEEK=$(date +%Y-W%U)

echo "🗓️  Weekly cleanup for week $WEEK"

# Archive everything with weekly note
relfa archive --all --note "Weekly cleanup for $WEEK - routine maintenance"

# Show statistics
echo "📊 Graveyard summary:"
find ~/Graveyard -name "*.epitaph" | wc -l | xargs echo "Total archived items:"
find ~/Graveyard -name "*.epitaph" -newer ~/Graveyard/.last-cleanup 2>/dev/null | wc -l | xargs echo "New this week:"

# Update cleanup timestamp
touch ~/Graveyard/.last-cleanup

echo "✨ Weekly cleanup complete!"
```

</details>

<details>
<summary>📅 Project Archive Script</summary>

```bash
#!/bin/bash
# project-archive.sh PROJECT_NAME

PROJECT_NAME=${1:-"unnamed-project"}
ARCHIVE_NOTE="Project '$PROJECT_NAME' completed on $(date +%Y-%m-%d). Archiving all project files for historical reference."

echo "📦 Archiving project: $PROJECT_NAME"

# Archive project-related files
relfa archive --all --note "$ARCHIVE_NOTE"

echo "🔍 Searching for any remaining project files..."
relfa search "$PROJECT_NAME"

echo "✅ Project archival complete for: $PROJECT_NAME"
```

</details>

## 🎯 Use Cases

<table>
<tr>
<td width="50%">

### 👩‍💼 **Professionals**

- **Downloads cleanup**: Archive old downloads with context
- **Project management**: Organize completed projects by timeline
- **Document versioning**: Archive old versions with change notes
- **Meeting materials**: Archive with meeting context and outcomes

### 🎓 **Students**

- **Assignment organization**: Archive by semester with grades/feedback
- **Research materials**: Keep old research accessible with context
- **Course notes**: Archive by semester with course information
- **Project files**: Maintain academic project history

</td>
<td width="50%">

### 💻 **Developers**

- **Log management**: Archive logs with deployment context
- **Build artifacts**: Archive with version and build information
- **Backup organization**: Structure backups with project context
- **Code samples**: Archive experimental code with learning notes

### 🏠 **Home Users**

- **Photo organization**: Archive old photos with family context
- **Document management**: Keep important docs with life events
- **Media collection**: Archive old media with descriptive context
- **Digital memories**: Maintain family digital history

</td>
</tr>
</table>

## 📚 Documentation

- 📖 **[Complete User Guide](https://nilp0inter.github.io/relfa/)** - Comprehensive documentation
- 📝 **[Man Page](relfa.1)** - Complete command reference (`man relfa`)
- 🏗️ **[API Documentation](https://docs.rs/relfa)** - For developers and contributors
- 💡 **[Examples Repository](examples/)** - Real-world usage examples and scripts
- ❓ **[FAQ & Troubleshooting](https://github.com/nilp0inter/relfa/wiki)** - Common questions and solutions

## 🔧 Development

### 🧪 **Setup Development Environment**

```bash
# Clone repository
git clone https://github.com/nilp0inter/relfa.git
cd relfa

# Build and test
cargo build
cargo test --all-features

# Install locally for testing
cargo install --path .

# Run with test configuration
mkdir -p test-{inbox,graveyard,config}
echo 'inbox = "./test-inbox"
graveyard = "./test-graveyard"
age_threshold_days = 1' > test-config/config.toml

RELFA_CONFIG_DIR=./test-config ./target/debug/relfa scan
```

### 🏗️ **Architecture Overview**

```
src/
├── main.rs           # CLI entry point and command routing
├── cli.rs            # Command-line argument parsing
├── commands.rs       # Business logic for all commands
├── config.rs         # Configuration management
├── scanner.rs        # File scanning and age detection
├── archiver.rs       # File archival and epitaph management
├── graveyard.rs      # Search and resurrection functionality
└── utils.rs          # Utility functions (pager, file ops)
```

### 🧪 **Testing**

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --all-features

# Run benchmarks
cargo bench

# Check code quality
cargo clippy --all-targets --all-features
cargo fmt --check
```

## 🤝 Contributing

We welcome contributions! Here's how you can help make Relfa even better:

### 🐛 **Reporting Issues**

- Use the [issue tracker](https://github.com/nilp0inter/relfa/issues)
- Include your OS, Rust version, and configuration
- Provide steps to reproduce the issue
- Include relevant log output

### ✨ **Submitting Features**

1. Check [existing issues](https://github.com/nilp0inter/relfa/issues) for similar requests
2. Create a detailed feature request with use cases
3. Consider implementing it yourself (we love PRs!)

### 💻 **Code Contributions**

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b amazing-feature`
3. **Implement** your changes with tests
4. **Test** thoroughly: `cargo test --all-features`
5. **Format** code: `cargo fmt && cargo clippy`
6. **Submit** a pull request with detailed description

### 📝 **Documentation**

- Improve README, man page, or code documentation
- Add usage examples or tutorials
- Fix typos or clarify explanations
- Translate documentation to other languages

### 🎨 **Other Ways to Help**

- ⭐ **Star the repository** to show your support
- 🐦 **Share on social media** to help others discover Relfa
- 💰 **[Sponsor the project](https://github.com/sponsors/nilp0inter)** to support development
- 🗣️ **Provide feedback** on your user experience

## 📊 Project Statistics

<div align="center">

![GitHub stars](https://img.shields.io/github/stars/nilp0inter/relfa?style=social)
![GitHub forks](https://img.shields.io/github/forks/nilp0inter/relfa?style=social)
![GitHub watchers](https://img.shields.io/github/watchers/nilp0inter/relfa?style=social)

[![GitHub contributors](https://img.shields.io/github/contributors/nilp0inter/relfa)](https://github.com/nilp0inter/relfa/graphs/contributors)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/nilp0inter/relfa)](https://github.com/nilp0inter/relfa/pulse)
[![GitHub last commit](https://img.shields.io/github/last-commit/nilp0inter/relfa)](https://github.com/nilp0inter/relfa/commits/main)

</div>

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- 💡 **Inspiration**: The endless struggle against digital entropy and the need for gentle file management
- 🦀 **Built with Rust**: Leveraging the amazing Rust ecosystem and community
- 🏗️ **Architecture**: Inspired by the Unix philosophy of small, composable tools
- 🌟 **Community**: Special thanks to all contributors, users, and feedback providers
- 📚 **Influences**: Getting Things Done methodology and digital minimalism principles

## 🔮 Roadmap

<details>
<summary>🛣️ Future Plans</summary>

### 🎯 **Planned Features**

- [ ] **Web interface** for remote graveyard management
- [ ] **Plugin system** for custom archival rules
- [ ] **Cloud backup** integration for graveyard sync
- [ ] **AI-powered** file categorization and epitaph suggestions
- [ ] **Statistics dashboard** showing digital entropy over time
- [ ] **Integration** with popular file managers

### 🎨 **Quality of Life**

- [ ] **Fuzzy search** for file resurrection
- [ ] **Bulk epitaph editing** for multiple files
- [ ] **Themes** for CLI output customization
- [ ] **Multiple inbox** support for different project types
- [ ] **Advanced notifications** with custom triggers

</details>

---

<div align="center">

**[⬆️ Back to Top](#-relfa---your-gentle-digital-gravedigger)**

---

_"For dust thou art, and unto dust shalt thou return."_  
_But maybe you'll want that markdown file again someday!_

**Made with 🪦 and ❤️ by the Relfa community**

_Happy haunting, and tidy archiving!_

</div>

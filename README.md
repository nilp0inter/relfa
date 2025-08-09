<div align="center">

<div id="user-content-toc">
  <ul style="list-style: none;">
    <summary>
      <h1>🪦 Relfa</h1>
    </summary>
  </ul>
</div>

<em>Your gentle digital gravedigger</em>

[![CI](https://img.shields.io/github/actions/workflow/status/nilp0inter/relfa/ci.yml?branch=main&style=flat-square)](https://github.com/nilp0inter/relfa/actions)
[![Crates.io](https://img.shields.io/crates/v/relfa?style=flat-square)](https://crates.io/crates/relfa)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

Relfa scans your inbox for old files, lets you know what's gathering dust, and helps you archive it.

```sh
$ relfa scan
🕷️  Scanning Inbox for dusty files...
☠️  2 items in ~/Inbox are gathering dust:
   📄 "old-document.pdf" (file) - last touched 5 days ago (2025-07-29)
   📄 "project-archive/" (folder) - last touched 4 days ago (2025-07-30)

🤖 1 item is eligible for auto-archiving (older than 7 days):
   📄 "temp-notes.txt" (file) - last touched 10 days ago (2025-07-24)
   ⚠️  These will be automatically archived if you run 'relfa archive' without arguments!

💡 Run 'relfa review' to interactively deal with these items
   or 'relfa archive' to auto-archive old files.
```

---

<!-- Non-technical stuff -->

## Motivation

In our daily digital lives, we constantly download, capture, and create small bursts of information — a PDF here, a draft note there, a temporary screenshot, today’s meeting summary. We save them quickly, intending to return later to tidy things up. But “later” rarely comes, and before long our computers fill with forgotten files and disorganized corners of digital clutter.

Relfa is the gentle gravedigger for your digital workspace. When a file begins to gather dust, it doesn't act hastily. First, it offers a quiet reminder—a chance for you to decide if the file's time has come. If it continues to sit untouched, Relfa then performs its final, gentle duty: respectfully moving the file to a well-organized graveyard. This two-step process ensures that archival is an intentional choice, not an accident, keeping your active workspace for the living, not the forgotten.

This tool isn’t about perfection. It’s about being present for what we’ve discarded and giving ourselves another chance to choose what stays.

## Features

-   **Dual-Threshold System**: Relfa uses two time limits. A "soft" threshold gently notifies you of stale files, while a "hard" threshold automatically archives them, keeping your workspace tidy without constant intervention.
-   **Flexible Archival**: Files are stored in a highly-organized "graveyard." Using symlinks, you can browse archived files by their creation, modification, or archival date, all without duplicating a single file.
-   **Interactive Review**: For files that need a personal touch, the `review` command lets you process them one-by-one with options to archive, delete, view, open, or skip.
-   **Epitaphs for Posterity**: When archiving, you can attach an "epitaph" — a note explaining the file's context. These notes are stored alongside the file and are fully searchable.
-   **Powerful Search & Resurrection**: Easily find archived files by searching filenames or epitaph content. The `resurrect` command brings files back from the graveyard to your inbox.
-   **Declarative & Automated**: Full support for Nix and Home Manager allows for declarative configuration and automated execution with systemd timers.
-   **Desktop Integration**: Get desktop notifications for scan results and open files directly in their default applications.

## Usage

<details>
<summary><strong>Scanning for Stale Files</strong></summary>

To see which files in your inbox have exceeded the `age_threshold_days`, run:

```sh
relfa scan
```

This will print a list of "stale" files and another list of files that are old enough to be auto-archived. This command is read-only and will not modify any files.

</details>

<details>
<summary><strong>Interactive Review</strong></summary>

For a guided, one-by-one review of your stale files, run:

```sh
relfa review
```

For each file, you will be prompted to choose an action:

-   `(a)rchive`: Move the file to the graveyard.
-   `(n)ote+archive`: Archive the file and attach an epitaph (a descriptive note).
-   `(t)ouch`: Update the file's modification time to keep it for another period.
-   `(d)elete`: Permanently delete the file (requires confirmation).
-   `(v)iew`: Preview the file's content using your configured pager.
-   `(o)pen`: Open the file with its default application.
-   `(s)kip`: Do nothing and move to the next file.
-   `(q)uit`: Exit the review session.

</details>

<details>
<summary><strong>Archiving Files</strong></summary>

The `archive` command is flexible and has several modes.

#### Auto-Archiving

To automatically archive all files that have exceeded the `auto_archive_threshold_days`, simply run `archive` with no arguments:

```sh
# Archives all files older than the "hard limit" threshold.
relfa archive

# You can also add a note to all auto-archived files.
relfa archive --note "Weekly automated cleanup"
```

#### Archiving Specific Files or All Stale Files

```sh
# Archive a single, specific item from your inbox.
relfa archive "my-old-document.pdf" --note "Final version, no longer needed."

# Archive all stale files (those older than the "soft limit").
relfa archive --all
```

</details>

<details>
<summary><strong>Searching the Graveyard</strong></summary>

To find files you've already archived, use the `search` command. It searches both filenames and epitaph content.

```sh
relfa search "project-alpha"
```

</details>

<details>
<summary><strong>Resurrecting Files</strong></summary>

To bring a file back from the graveyard to your inbox, use `resurrect`. This copies the file back, leaving the original in the graveyard.

```sh
relfa resurrect "important-document.pdf"
```

If your search term matches multiple files, Relfa will present a list for you to choose from.

</details>

<!-- Technical stuff -->

## Installation

<details>
<summary><strong>From Source (via Nix)</strong></summary>

The recommended way to use Relfa is declaratively through its **Home Manager module**, which allows for easy configuration and automated execution. See the `Home Manager Configuration` section for details.

For quick trials or environments without Home Manager, you can use one of the following `nix` commands:

**Temporary Execution**

To run Relfa without installing it, use `nix run`:

```sh
nix run github:nilp0inter/relfa -- [command]
# Example:
nix run github:nilp0inter/relfa -- scan
```

**Persistent Installation**

To install Relfa into your user profile, making it available in your shell, run:

```sh
nix profile install github:nilp0inter/relfa
```

</details>

<details>
<summary><strong>From Crates.io (via Cargo)</strong></summary>

If you have the Rust toolchain installed on your system, you can install Relfa directly from `crates.io` using `cargo`:

```sh
cargo install relfa
```

This command will download the source code, compile it, and place the `relfa` binary in your Cargo binary path (`~/.cargo/bin/`), which should be in your system's `PATH`.

</details>

<details>
<summary><strong>Pre-compiled Binaries</strong></summary>

Pre-compiled binaries are available for Linux and macOS from the [releases page](https://github.com/nilp0inter/relfa/releases).

**Supported Platforms:**
- Linux x86_64 (`relfa-VERSION-x86_64-unknown-linux-musl`)
- macOS Intel (`relfa-VERSION-x86_64-apple-darwin`)
- macOS Apple Silicon (`relfa-VERSION-aarch64-apple-darwin`)

**Installation:**
1. Download the appropriate binary for your platform from the [latest release](https://github.com/nilp0inter/relfa/releases/latest)
2. Make it executable: `chmod +x relfa-*`
3. Copy it to a directory in your PATH: `cp relfa-* ~/.local/bin/relfa` (or `/usr/local/bin/relfa`)

If you need binaries for other platforms, please **[open an issue on GitHub](https://github.com/nilp0inter/relfa/issues)**.

</details>

## Configuration

<details>
<summary><strong>Manual Configuration</strong></summary>

Relfa is configured via a TOML file located at `~/.config/relfa/config.toml`.

You can generate a configuration file with default values by running:

```sh
relfa config
```

This will create the file if it doesn't exist and print the current settings.

#### Example `config.toml`

```toml
# Path to the directory you want Relfa to monitor.
inbox = "/home/user/Inbox"

# Path to the directory where archived files will be stored.
graveyard = "/home/user/Graveyard"

# (Soft limit) Files older than this (in days) are considered "stale"
# and will be shown during a `scan` or `review`.
age_threshold_days = 3

# (Hard limit) Files older than this (in days) will be automatically
# archived when you run `relfa archive` without any arguments.
auto_archive_threshold_days = 7

# How to deliver notifications. Can be "cli" or "desktop".
notification = "desktop"

# The command to use for viewing files with the `review` command.
# Defaults to your $PAGER environment variable, or "less".
pager = "less"

# Configuration for the graveyard's directory structure.
[path_format]
# A template for creating date-based paths.
# Available variables: {hostname}, {year}, {month}, {day}, {month:02}, {day:02}
date_format = "{hostname}/{year}/{month:02}/{day:02}"

# Defines a subdirectory for organizing files by their creation date.
# `type = "original"` means the actual files are stored here.
[path_format.created_subdir]
type = "original"
name = "created"

# Defines a subdirectory for organizing files by their modification date.
# `type = "symlink"` means this directory will contain symbolic links.
# `target = "created"` means the links will point to the files in the "created" subdirectory.
[path_format.modified_subdir]
type = "symlink"
name = "modified"
target = "created"

# Defines a subdirectory for organizing files by their archival date.
# In this example, this view is disabled.
[path_format.archived_subdir]
type = "nothing"
```

</details>

<details>
<summary><strong>Home Manager Configuration</strong></summary>

For users of [Nix](https://nixos.org/) and [Home Manager](https://github.com/nix-community/home-manager), Relfa provides a module for declarative configuration.

1.  **Add the flake to your inputs:**

    ```nix
    # flake.nix
    {
      inputs = {
        relfa.url = "github:nilp0inter/relfa";
        # ... other inputs
      };
    }
    ```

2.  **Import the module in your `home.nix`:**

    ```nix
    { inputs, ... }: {
      imports = [ inputs.relfa.homeManagerModules.relfa ];
    }
    ```

3.  **Configure Relfa:**

    The `programs.relfa.settings` block is required for the program to run. Note that the `path_format` section and its sub-sections are also mandatory.

    ```nix
    # home.nix
    programs.relfa = {
      enable = true;

      settings = {
        inbox = "${config.home.homeDirectory}/Downloads";
        graveyard = "${config.home.homeDirectory}/Archive";
        age_threshold_days = 5;
        auto_archive_threshold_days = 14;
        notification = "desktop";

        # The `path_format` block is required.
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
            target = "created";
          };
        };
      };

      # Optional: Enable a systemd timer for automated execution.
      timer = {
        enable = true;
        # Run `relfa scan` daily.
        command = "scan";
        frequency = "daily";
        # Add a random delay to avoid running at the exact same time as other services.
        randomizedDelay = "1h";
      };
    };
    ```

#### Timer Options

The `timer` submodule allows you to automate Relfa's execution.

-   `command`: Which command to run. Can be `"scan"`, `"archive"`, or `"scan-then-archive"`.
-   `frequency`: How often to run the command. Accepts `systemd.time` calendar event formats (e.g., `"daily"`, `"hourly"`, `"*:0/30"` for every 30 minutes).
-   `randomizedDelay`: A random delay to add before execution (e.g., `"1h"`, `"30m"`).

</details>

## Development

<details>
<summary><strong>Setup and Workflow</strong></summary>

The official and recommended development setup for Relfa uses [Nix](https://nixos.org/) and [Direnv](https://direnv.net/). While other setups are possible, they are not officially supported and are left to the user's discretion.

### Prerequisites

Before you begin, ensure you have both Nix and Direnv installed on your system.

### Setup

Setting up the development environment is a one-step process. Simply navigate to the project's root directory in your terminal and run:

```sh
direnv allow
```

This command will trigger the Nix flake to build the complete development environment. It automatically:

-   Downloads and installs all necessary dependencies (Rust toolchain, etc.).
-   Configures and installs the required Git hooks.
-   Activates a `devshell` with pre-configured aliases for common tasks (build, test, format, etc.).

### Workflow

Once the environment is active, you will have access to a `devshell` menu with commands for building, testing, and formatting the code.

The installed Git hooks will run automatically on every commit. These hooks check for correct formatting and ensure the project compiles, helping to guarantee that your changes will pass the CI pipeline.

</details>

<!-- Community stuff -->

## Community

### Contributing

Contributions are welcome, but please follow these guidelines to ensure a smooth process.

-   **Reporting Issues**: If you find a bug, please [open an issue](https://github.com/nilp0inter/relfa/issues). Include your OS, Relfa version, and clear steps to reproduce the problem.
-   **Feature Requests**: If you have an idea for a new feature, please [open an issue](https://github.com/nilp0inter/relfa/issues) to start a discussion. **Please do not submit a pull request for a new feature without prior discussion and approval in an issue.**
-   **Code Contributions**:
    -   Pull requests are welcome for **bug fixes only**.
    -   Your pull request **must** include tests that demonstrate the bug and verify your fix.
    -   To contribute:
        1.  Fork the repository.
        2.  Create a branch for your fix (`git checkout -b fix/some-bug`).
        3.  Implement your changes and add corresponding tests.
        4.  Submit a pull request that links to the relevant issue.

<!-- Legal stuff -->

## Legal

This project is licensed under the MIT License. See the `LICENSE` file for details.

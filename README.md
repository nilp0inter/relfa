# Relfa 🪦

**Relfa** is your gentle digital gravedigger. It helps you keep your computer’s clutter under control by monitoring your `Inbox` folder, nudging you to review old files, and lovingly archiving them in a dust-covered, cobwebby digital **Graveyard** — instead of letting them rot in forgotten digital corners.

> “In Relfa’s Graveyard, nothing is truly lost: just waiting in gentle slumber… for you, or the next digital archaeologist.”

---

## Features

- 📦 **Inbox Watch:** Monitors your `~/Inbox` and reminds you about neglected files and folders.
- ⏰ **Gentle Nudges:** Notifies you (CLI and/or desktop) when files become stale.
- 🪦 **Archival Graveyard:** Moves your forgotten digital detritus into a `Graveyard`, organized by device and date.
- 🔗 **Time-Triplets:** Each file’s journey is preserved via real file moves and spooky symlinks by **creation**, **last modification**, and **archiving** time.
- 🕹 **CLI Tools:** Scan, review, and archive with a single command, or interactively choose what to bury… or resurrect.
- 🔒 **Safety First:** Never deletes anything without your say-so. All file moves are safe and reversible.
- 🔮 **Lighthearted, Never Grim:** Outputs and folders with a smile (and maybe a spider).

---

## How It Works

1. **Inbox:**  
   Save files in `~/Inbox/` for working-on or classifying later.

2. **Relfa Scan:**  
   Relfa checks which files or folders have gathered dust (i.e., no changes in N days).

3. **Notifications:**  
   Get reminders when there's digital clutter, with details on what's ripe for archiving.

4. **Archiving:**  
   Old files are **moved** into the `Graveyard` folder, organized so you always know where (and when!) you can find them.

5. **Symlinks:**  
   Each file is reachable in the `Graveyard` by its creation, modification, *and* archiving date — for the true digital time traveler.

---

## Directory Structure

```
/home/youruser/
├── Inbox/
│   ├── todo.txt
│   └── ancient-folder/
└── Graveyard/
    └── HOSTNAME/
        ├── created/
        │   └── YYYY/MM/DD/<file-or-folder>
        ├── modified/
        │   └── YYYY/MM/DD/<file-or-folder> -> symlink to created
        └── archived/
            └── YYYY/MM/DD/<file-or-folder> -> symlink to created
```

**Only** top-level files or folders in your `Inbox/` are archived (their whole contents, not bits and pieces).

---

## Quickstart

1. **Install**

   Clone and build (Rust required):

   ```sh
   git clone https://github.com/youruser/relfa.git
   cd relfa
   cargo install --path .
   ```

2. **Run a scan**

   ```sh
   relfa scan
   ```

3. **Review & archive interactively**

   ```sh
   relfa review
   ```

4. **Archive everything ripe for burial**

   ```sh
   relfa archive --all
   ```

---

## CLI Commands

| Command                  | What it does                                   |
|--------------------------|------------------------------------------------|
| `relfa scan`             | Lists stale (old/untouched) items in Inbox     |
| `relfa review`           | Interactively archive, skip, or delete files   |
| `relfa archive [item]`   | Manually archive a specific file/folder        |
| `relfa archive --all`    | Archive all eligible files/folders             |
| `relfa config`           | Show or edit config                            |

Example output:
```
☠️  2 items in ~/Inbox are gathering dust:
    - "forgotten.txt" (last touched: 2024-05-10)
Archive them now? [y/N]
```

---

## Configuration

Edit `~/.config/relfa/config.toml` to customize:

```toml
inbox = "/home/youruser/Inbox"
graveyard = "/home/youruser/Graveyard"
age_threshold_days = 14
hostname = "laptop-mbp"
notification = "desktop" # or "cli"
```

*Default paths:*
- Inbox: `~/Inbox`
- Graveyard: `~/Graveyard`
- Notifications: CLI output

---

## Design Philosophy

- **Gentle, never grim:** Your files rest safe in the graveyard; nothing is lost, and nothing is deleted without your choice.
- **Fun and thematic:** A touch of the spooky, but always friendly—think dusty attics and cobwebs, not data loss!
- **Minimal friction:** One command to scan or archive; always obvious what will happen next.
- **Reversible:** Files are just moved or symlinked; you can restore anything from the graveyard by copying it out.

---

## FAQ

**Q: Will Relfa delete my files?**  
A: Never without explicit confirmation during review.

**Q: What if I have identically named files?**  
A: Files in the graveyard are uniquified using timestamps or suffixes where needed.

**Q: Are symlinks safe?**  
A: Symlinks point only to files within your own graveyard, and only if your filesystem supports them.

---

## Roadmap / Stretch Goals

- Per-item epitaphs (notes on why you buried a file!)
- ASCII tombstone banners 🪦
- Statistics: your digital entropy charted over time!
- Quick resurrection: `relfa resurrect <file>`
- File search & fuzzy find in the graveyard

Open to PRs and ideas—help haunt Relfa’s future!

---

## License

MIT License.

---

## Acknowledgements

- Inspired by Getting Things Done, and the endless struggle against digital entropy.
- Conceptual spiders, digital mothballs, and wise gravediggers everywhere.

---

## Contributing

Pull requests, bug reports, and grave ethics all welcome!

---

## “For dust thou art, and unto dust shalt thou return.”  
*But maybe you'll want that markdown file again someday!*

---

**Happy haunting, and tidy archiving!**

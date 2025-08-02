# **Relfa Specification**

---

## 1. Purpose and Motivation

**Relfa** is a personal digital assistant that helps users manage their semi-temporary files. It monitors a designated "Inbox" and gently reminds users to classify, archive, or delete items that linger too long. Relfa automatically moves such items to a delightfully dusty, cobwebby "Graveyard"—a structured archive—for long-term storage, so users maintain digital hygiene while never losing anything important.

Relfa’s goal is to make forgotten files visible, encourage mindful classification, and provide a characterful experience reminiscent of a cluttered attic or spooky graveyard.

---

## 2. Key Concepts

- **Inbox**:  
  Local-only directory (default: `~/Inbox/`). Contains working files/folders awaiting organization.
- **Graveyard**:  
  The main archive, structured and quirky (default: `~/Graveyard/`). Items are organized chronologically and by host and event time.
- **Events**:  
  - **Created**: When a file/folder originally appeared (birth time if available).
  - **Modified**: Last time contents changed.
  - **Archived**: When Relfa moved it to the Graveyard.
- **Scope**:  
  Only top-level files and directories in Inbox are eligible for archival. Relfa treats directories atomically (no partial archiving).

---

## 3. Directory Layout

### 3.1 Inbox

```
~/Inbox/
  ├─ file1.pdf
  ├─ myfolder/
  └─ readme.txt
```

### 3.2 Graveyard

```
~/Graveyard/
  └── HOSTNAME/
      ├── created/
      │    └── YYYY/
      │         └── MM/
      │              └── DD/
      │                   └── <original_name>
      ├── modified/
      │    └── YYYY/MM/DD/<original_name> -> ../../../../created/YYYY/MM/DD/<original_name>
      └── archived/
           └── YYYY/MM/DD/<original_name> -> ../../../../created/YYYY/MM/DD/<original_name>
```
- All links (symlinks) in `modified/` and `archived/` point to the `created/` directory.

---

## 4. Behavior

### 4.1 Monitoring

- On user log-in or on demand, Relfa scans the Inbox.
- For each top-level item (file or directory):
  - If **untouched** (no mtime change, and for directories, no contained file mtime change) for more than N days (default: 14), notify user.
  - "Untouched" = the *most recent modification time* among the item or all its descendants.

### 4.2 Notifications

- Issue warning (CLI output and/or desktop notification):  
  `"Relfa: Some files in Inbox have gathered digital dust. Consider classifying or removing them!"`
- Optionally, provide a summary:  
  ```
  2 items in ~/Inbox have been untouched for over 14 days:
    - "notes.md" (last changed: 2024-06-10)
    - "old_project/" (last change inside: 2024-06-01)
  ```

### 4.3 Archival

- User chooses to archive via CLI command or interface (interactive confirmation, or write a script to automate on further delay).
- When archiving an eligible item:
  1. **Determine Timestamps**
     - **Created**: Use file/folder birth time (`btime`) if available, or fallback to earliest mtime.
     - **Modified**: Most recent modification time (file, or latest in dir tree).
     - **Archived**: Current time (when the move occurs).
  2. **Construct Archive Paths**  
     ```
     <Graveyard>/<HOSTNAME>/created/YYYY/MM/DD/<item>
     <Graveyard>/<HOSTNAME>/modified/YYYY/MM/DD/<item>  [symlink]
     <Graveyard>/<HOSTNAME>/archived/YYYY/MM/DD/<item>  [symlink]
     ```
  3. **Ensure Unique Names**
     - If a name collision occurs in any directory, append a suffix (`_1`, or UNIX timestamp).
  4. **Move Item**
     - Move the actual file/folder to the `created/` path.
     - Create symlinks in the `modified/` and `archived/` trees pointing to the file in `created/`.

### 4.4 Directory Timestamps

- Update parent directory mtimes (in the archive trees) to always reflect the latest child modification times, for accurate sorting and quick age queries.

---

## 5. Constraints & Notes

- **No partial archival**: Only top-level items, never files inside subdirectories.
- **Files are never deleted** unless user removes them from Inbox or Graveyard.
- **Symlink conflicts** are gracefully handled (by suffixing as above).
- **Everything is local**: Never uploads, never syncs by itself.
- **Handle Host Correctly**: Use only safe characters for HOSTNAME in paths (sanitize, or use a config-provided label if system hostname is unsuitable).
- **Configurable**: User should be able to specify:
  - Inbox location
  - Graveyard location
  - Age threshold in days
  - Notification preferences

---

## 6. CLI Interface

**Basic commands:**

- `relfa scan`  
  Scans Inbox and prints/delivers notifications for aged files.
- `relfa review`  
  Interactive review: list aged files, prompt to (archive/skip/delete).
- `relfa archive [--all | <item>]`  
  Archive all touched items or a specific file/folder.
- `relfa config`  
  Print current config and path info.

**Examples**
```
$ relfa scan
2 items in ~/Inbox are gathering dust...

$ relfa archive notes.md
Archived notes.md to the Graveyard 🪦

$ relfa review
[1/2] Archive old_project/? (last moved: 6/1/24) [Y/n/d] _
```

---

## 7. Implementation Notes

- Written in Rust.
- Cross-platform, but only symlinks on platforms that support them.
- Should fail gracefully if permissions or symlinks are unavailable.
- Structure and messages should be whimsical, a bit spooky, but always respectful of data.

---

## 8. Sample Motivation (for README/UX)

> Like a digital grave digger, Relfa prowls your Inbox looking for files you’ve forgotten. With gentle reminders and a bit of graveyard humor, it helps you decide what to classify, what to keep, what to bury safely. In Relfa’s Graveyard, nothing is ever really lost—just carefully entombed for future archaeologists (or nostalgic you).

---

## 9. Stretch/Optional Features

- Per-item notes or "epitaphs" on archived files (user can add a short message at archive time).
- ASCII-art tombstones/banners.
- Graveyard search: `relfa find <term>`.
- Undo action for the most recent archive.

---

## 10. Success Criteria

- Must not lose, overwrite, or corrupt any user data.
- Must never archive erroneously (should always be a clear user action per item).
- Should perform well with hundreds of files/folders.
- Should be clear, comfortable, and lightly spooky in its output and organization.

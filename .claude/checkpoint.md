# Checkpoint: claude-task-manager Project State

**Date:** 2025-12-26
**Last Commit:** Pending (rebrand to claude-task-manager)

---

## Completed Work This Session

### v0.12.0 - Rebrand to claude-task-manager

**Project Renamed:**
- Package name: `tascli` → `claude-task-manager`
- CLI command: `tascli` → `ctm`
- Data directory: `~/.local/share/tascli/` → `~/.local/share/ctm/`
- Config directory: `~/.config/tascli/` → `~/.config/ctm/`
- Database file: `tascli.db` → `ctm.db`

**Files Updated:**
- `Cargo.toml` - New name, description, repository, keywords, categories
- `README.md` - Complete rewrite with Claude-first positioning
- `CHANGELOG.md` - Added v0.12.0 rebrand entry
- `src/config/mod.rs` - Updated paths
- `.claude/CLAUDE.md` - Updated all references
- `.claude/agents/ctm.md` - Renamed from tascli.md
- `.claude/commands/*.md` - All updated to use `ctm`
- `.claude/resume_prompt.md` - Updated
- `.claude/checkpoint.md` - Updated

---

## Current Schema Version

**Version 4** with columns:
- id, action, category, content, create_time, target_time, modify_time, status
- cron_schedule, human_schedule, recurring_task_id, good_until
- reminder_days (v3)
- project (v4) - links task to project in config

---

## Usage

```bash
# Install
cargo install claude-task-manager

# Add task with project
ctm task "Fix login bug" friday -p myapp

# In Claude Code, open session in project directory
/work 3
```

### Config Structure:
```json
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "myapp": {
      "path": "/mnt/c/python/myapp",
      "conda_env": "myapp-env",
      "claude_flags": "--dangerously-skip-permissions"
    }
  }
}
```

---

## Files Modified This Session

**Rust Source:**
- `src/config/mod.rs` - Updated DB_NAME, DEFAULT_DATA_DIR, CONFIG_PATH constants

**Documentation:**
- `Cargo.toml` - name, version, description, repository, keywords, categories, bin name
- `README.md` - Complete rewrite
- `CHANGELOG.md` - Added v0.12.0

**Claude Code Integration:**
- `.claude/CLAUDE.md` - Updated all references
- `.claude/agents/ctm.md` - New (renamed from tascli.md)
- `.claude/agents/tascli.md` - Deleted
- `.claude/commands/today.md` - Updated
- `.claude/commands/tasks.md` - Updated
- `.claude/commands/task.md` - Updated
- `.claude/commands/done.md` - Updated
- `.claude/commands/overdue.md` - Updated
- `.claude/commands/reminders.md` - Updated
- `.claude/commands/work.md` - Updated
- `.claude/resume_prompt.md` - Updated
- `.claude/checkpoint.md` - Updated

---

## Test Status

All 65 tests should pass (need to verify after changes).

---

## Migration Note

Users migrating from tascli should:
```bash
# Move database
mv ~/.local/share/tascli/tascli.db ~/.local/share/ctm/ctm.db

# Move config
mv ~/.config/tascli/config.json ~/.config/ctm/config.json
```

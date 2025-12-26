# Checkpoint: tascli Project State

**Date:** 2025-12-26
**Last Commit:** `3520b1e` - feat: add /work command for project sessions

---

## Completed Work This Session

### 1. Claude-First Optimization
- Added `/tasks`, `/task`, `/done`, `/overdue`, `/today` commands
- Enhanced tascli agent with natural language understanding
- Updated CLAUDE.md documentation

### 2. Reminder Window Feature (v0.10.3)
- Added `-r` flag for tasks (schema v3)
- Tasks with reminders appear in `/today` within their reminder window
- Default 7 days when `-r` specified without value

### 3. Project Association & /work Command (v0.11.0)
- **Schema v4** - Added `project` column to items table
- **Config extension** - Added `projects` section with path, conda_env, claude_flags, prompt_template
- **CLI** - Added `-p` flag to TaskCommand and UpdateCommand
- **Path utility** - `src/utils/path.rs` for Linux→Windows path conversion
- **`/work` command** - Claude command that spawns sessions in project directories

### 4. Spawning Claude Sessions
**Working command template:**
```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p Ubuntu -d C:\windows\path wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && claude\""
```

**Key discoveries:**
- `/init` bypasses WSL interop execute permission issues
- `-p Ubuntu` uses the correct terminal profile
- `export PATH=...` fixes "native installation not in PATH" warning
- Can pass flags and prompts to claude

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
# Add task with project
tascli task "Fix login bug" friday -p myapp

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

## Files Modified

**Rust Source (project feature):**
- `src/config/mod.rs` - ProjectConfig struct, helper functions
- `src/db/conn.rs` - Schema v4, project column migration
- `src/db/item.rs` - Added project field
- `src/db/crud.rs` - Updated INSERT/UPDATE for project
- `src/args/parser.rs` - Added -p flag
- `src/actions/addition.rs` - Project validation and assignment
- `src/actions/modify.rs` - Project update handling
- `src/utils/mod.rs` - New utils module
- `src/utils/path.rs` - Path conversion utilities
- `src/main.rs` - Include utils module

**Claude Code Integration:**
- `.claude/CLAUDE.md` - Full documentation with project config
- `.claude/agents/tascli.md` - Project patterns and /work capability
- `.claude/commands/work.md` - New /work command
- `.claude/commands/task.md` - Added -p flag examples

**User Documentation:**
- `README.md` - Project feature, configuration, Claude Code section
- `CHANGELOG.md` - v0.10.3 and v0.11.0 entries

---

## Test Status

All 65 tests passing (includes 3 new path utility tests).

---

## WSL Configuration Note

Enabled Windows interop in `/etc/wsl.conf`:
```ini
[interop]
enabled = true
appendWindowsPath = false
```

The `fmask=111` in automount options strips execute bits, which is why we use `/init` as a workaround.

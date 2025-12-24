# Checkpoint: tascli Project State

**Date:** 2025-12-24
**Last Commit:** `2cece8d` - feat: add reminder window for tasks (-r flag)

---

## Completed Work This Session

### 1. Claude-First Optimization
- Added `/tasks`, `/task`, `/done`, `/overdue`, `/today` commands
- Enhanced tascli agent with natural language understanding
- Updated CLAUDE.md documentation

### 2. Reminder Window Feature
- Added `-r` flag for tasks (schema v3)
- Tasks with reminders appear in `/today` within their reminder window
- Default 7 days when `-r` specified without value
- All tests passing (62/62)

### 3. Spawning Claude Sessions (Proof of Concept)
**Successfully proven we can spawn new Claude Code sessions from within a session.**

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

**Version 3** with columns:
- id, action, category, content, create_time, target_time, modify_time, status
- cron_schedule, human_schedule, recurring_task_id, good_until
- reminder_days (new in v3)

---

## Next Task: Implement `/work` Command

**Goal:** Add project-aware task spawning so `/work 3` opens Claude in the task's project directory.

**Plan file:** `.claude/plans/work-command-implementation.md`

### Summary of Implementation:

1. **Config extension** - Add `projects` section to `~/.config/tascli/config.json`
2. **Schema v4** - Add `project` column to items table
3. **CLI** - Add `-p` flag to TaskCommand and UpdateCommand
4. **Path conversion** - Linux path → Windows path utility
5. **`/work` command** - Claude command that spawns sessions

### Config Structure:
```json
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "myapp": {
      "path": "/mnt/c/python/myapp",
      "conda_env": "myapp-env",
      "claude_flags": "--dangerously-skip-permissions",
      "prompt_template": null
    }
  }
}
```

### Usage:
```bash
tascli task "Fix bug" friday -p myapp
/work 1  # Opens Claude in /mnt/c/python/myapp
```

---

## Files Modified This Session

- `.claude/CLAUDE.md` - Updated with quick commands
- `.claude/agents/tascli.md` - Enhanced with natural language + reminder docs
- `.claude/commands/today.md` - New command
- `.claude/commands/tasks.md` - New command
- `.claude/commands/task.md` - New command
- `.claude/commands/done.md` - New command
- `.claude/commands/overdue.md` - New command
- `src/db/conn.rs` - Schema v3, reminder_days column
- `src/db/item.rs` - Added reminder_days field
- `src/db/crud.rs` - Updated INSERT/UPDATE for reminder_days
- `src/args/parser.rs` - Added -r flag
- `src/actions/addition.rs` - Handle reminder on task creation
- `src/actions/modify.rs` - Handle reminder on task update
- `src/actions/list/tasks.rs` - Reminder window filtering

---

## WSL Configuration Note

Enabled Windows interop in `/etc/wsl.conf`:
```ini
[interop]
enabled = true
appendWindowsPath = false
```

The `fmask=111` in automount options strips execute bits, which is why we use `/init` as a workaround.

---

## Test Status

All 62 tests passing as of last commit.

# Resume Prompt: claude-task-manager

## Context

You are working on **claude-task-manager** (formerly tascli), a Claude-first task management CLI written in Rust. The project is at `/mnt/c/python/tascli` (to be renamed to `/mnt/c/python/claude-task-manager`).

## Recent Work Completed

### v0.12.0 - Rebrand to claude-task-manager
- Renamed project from tascli to claude-task-manager
- CLI command changed from `tascli` to `ctm`
- Updated all paths: `~/.local/share/ctm/`, `~/.config/ctm/`
- Complete README rewrite emphasizing Claude-first design
- Updated all .claude files (CLAUDE.md, agent, commands)

### v0.11.0 - Project Association & /work Command
- **Schema v4** - Added `project` column to items table
- **Config extension** - Added `projects` section to config.json
- **CLI** - Added `-p` flag to `task` and `update` commands
- **Path utility** - Linux→Windows path conversion for WSL
- **`/work` command** - Opens Claude in task's project directory

### v0.10.3 - Reminder Window Feature
- Added `-r` flag for tasks (schema v3)
- Tasks with reminders appear in `/today` within their reminder window
- Added Claude Code quick commands (`/today`, `/tasks`, `/task`, `/done`, `/overdue`)

## Current State

- **Schema Version:** 4
- **Tests:** All 65 passing
- **Last Commit:** Pending (rebrand changes)

## Key Files

| Area | Files |
|------|-------|
| Config | `src/config/mod.rs` |
| Schema | `src/db/conn.rs` |
| Model | `src/db/item.rs` |
| CLI | `src/args/parser.rs` |
| Actions | `src/actions/addition.rs`, `src/actions/modify.rs` |
| Utils | `src/utils/path.rs` |
| Commands | `.claude/commands/*.md` |
| Agent | `.claude/agents/ctm.md` |
| Docs | `README.md`, `.claude/CLAUDE.md`, `CHANGELOG.md` |

## Usage

```bash
# Create config with projects
cat > ~/.config/ctm/config.json << 'EOF'
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "ctm": { "path": "/mnt/c/python/claude-task-manager" }
  }
}
EOF

# Add task with project
ctm task "Test feature" tomorrow -p ctm

# Open Claude in project directory
/work 1
```

## Spawn Command Template (Proven Working)

```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p {profile} -d {win_path} wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && claude\""
```

## Notes

- WSL interop uses `/init` workaround for fmask execute bit stripping
- Terminal profile "Ubuntu" opens the correct window style
- `export PATH=...` fixes native installation PATH issue

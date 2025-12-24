# Resume Prompt: Implement `/work` Command for tascli

## Context

You are continuing work on **tascli**, a Claude-first task management CLI written in Rust. The project is at `/mnt/c/python/tascli`.

## What Was Accomplished

1. **Reminder feature** implemented and committed (`-r` flag, schema v3)
2. **Proven we can spawn new Claude sessions** from within a session using:

```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p Ubuntu -d C:\windows\path wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && claude\""
```

## Your Task

Implement the `/work` command feature that allows users to:

1. Associate tasks with projects: `tascli task "Fix bug" friday -p myapp`
2. Spawn Claude in project directory: `/work 1`

## Key Files to Read First

1. **Implementation Plan:** `.claude/plans/work-command-implementation.md`
2. **Current State:** `.claude/checkpoint.md`
3. **Config module:** `src/config/mod.rs` (needs project parsing)
4. **Schema:** `src/db/conn.rs` (needs v4 migration)
5. **Item model:** `src/db/item.rs` (needs project field)

## Implementation Summary

### 1. Extend Config (`src/config/mod.rs`)

Add structs and parsing for:
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

### 2. Schema v4 (`src/db/conn.rs`)

```rust
const SCHEMA_VERSION: i32 = 4;

// Migration:
if current_version < 4 && current_version > 0 {
    conn.execute("ALTER TABLE items ADD COLUMN project TEXT", [])?;
}
```

### 3. Add `-p` Flag (`src/args/parser.rs`)

```rust
#[arg(short = 'p', long)]
pub project: Option<String>,
```

### 4. Path Conversion Utility

Create `src/utils/path.rs`:
- `linux_to_windows_path("/mnt/c/foo")` → `"C:\foo"`
- `build_spawn_command(...)` → full wt.exe command

### 5. Create `/work` Command (`.claude/commands/work.md`)

A Claude command that:
1. Gets task by index
2. Looks up project in config
3. Converts path
4. Spawns new Claude session

## Spawn Command Template

```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p {profile} -d {win_path} wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && {conda_activate} claude {flags} \\\"{prompt}\\\"\""
```

## Testing

After implementation:
```bash
# Create test config
cat > ~/.config/tascli/config.json << 'EOF'
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "tascli": {
      "path": "/mnt/c/python/tascli"
    }
  }
}
EOF

# Add task with project
tascli task "Test work command" tomorrow -p tascli

# Test spawn
/work 1
```

## Commits

1. `feat: add project field to tasks` - schema, model, CLI, CRUD
2. `feat: add /work command for project sessions` - path util, command, docs

## Notes

- WSL interop uses `/init` as workaround for fmask execute bit stripping
- Terminal profile "Ubuntu" opens the purple window
- `export PATH=...` fixes native installation PATH issue
- All 62 existing tests should continue to pass

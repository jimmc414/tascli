# Plan: Implement `/work` Command for Project-Aware Task Sessions

## Overview

Add the ability to spawn new Claude Code sessions in project-specific directories when working on tasks. Tasks can be associated with projects defined in config, and `/work <index>` launches a properly configured Claude session.

## Proven Working Command

```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p Ubuntu -d C:\windows\path wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && claude\""
```

### Components:
- `/init` - WSL interop wrapper (bypasses execute permission issues)
- `wt.exe -p Ubuntu` - Windows Terminal with Ubuntu profile
- `-d C:\path` - Windows-format working directory
- `wsl.exe -e bash -c "..."` - Execute bash command
- `export PATH=...` - Ensure Claude is in PATH
- `claude` - Can add flags and prompt

---

## Implementation Steps

### Step 1: Extend Config Schema (`src/config/mod.rs`)

**Current config structure:**
```json
{
  "data_dir": "/custom/path"
}
```

**New config structure:**
```json
{
  "data_dir": "/custom/path",
  "terminal_profile": "Ubuntu",
  "claude_path": "/home/jim/.local/bin/claude",
  "projects": {
    "tascli": {
      "path": "/mnt/c/python/tascli",
      "conda_env": null,
      "claude_flags": null,
      "prompt_template": null
    },
    "security_questionnaire": {
      "path": "/mnt/c/python/security_questionnaire",
      "conda_env": "sec-env",
      "claude_flags": "--dangerously-skip-permissions",
      "prompt_template": "Continue working on: {content}"
    },
    "myapp": {
      "path": "/mnt/c/python/myapp",
      "conda_env": "myapp-env",
      "claude_flags": "--dangerously-skip-permissions",
      "prompt_template": null
    }
  }
}
```

**Rust structs to add:**
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectConfig {
    pub path: String,
    pub conda_env: Option<String>,
    pub claude_flags: Option<String>,
    pub prompt_template: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub data_dir: Option<String>,
    pub terminal_profile: Option<String>,
    pub claude_path: Option<String>,
    pub projects: Option<HashMap<String, ProjectConfig>>,
}
```

**Functions to add:**
- `get_project(name: &str) -> Option<ProjectConfig>`
- `list_projects() -> Vec<String>`

---

### Step 2: Database Schema v4 (`src/db/conn.rs`)

Add `project` column to items table:

```rust
const SCHEMA_VERSION: i32 = 4;  // was 3

// In init_table(), add migration:
if current_version < 4 && current_version > 0 {
    conn.execute("ALTER TABLE items ADD COLUMN project TEXT", [])?;
}
```

Also update CREATE TABLE to include `project TEXT` for new databases.

---

### Step 3: Update Item Model (`src/db/item.rs`)

Add field:
```rust
pub struct Item {
    // ... existing fields ...
    pub project: Option<String>,  // References key in config.projects
}
```

Update:
- `Item::new()` - add `project: None`
- `Item::with_target_time()` - add `project: None`
- `from_row()` - add `project: row.get("project").ok()`

---

### Step 4: Update CRUD Operations (`src/db/crud.rs`)

**insert_item():**
```rust
"INSERT INTO items (..., project) VALUES (..., ?11)"
// Add item.project to params
```

**update_item():**
```rust
"UPDATE items SET ..., project = ?12 WHERE id = ?13"
// Add item.project to params
```

---

### Step 5: Add `-p` Flag to CLI Parser (`src/args/parser.rs`)

```rust
#[derive(Debug, Args)]
pub struct TaskCommand {
    // ... existing fields ...
    /// project name (must be defined in config)
    #[arg(short = 'p', long)]
    pub project: Option<String>,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    // ... existing fields ...
    /// update project association
    #[arg(short = 'p', long)]
    pub project: Option<String>,
}
```

---

### Step 6: Handle Project in Task Addition (`src/actions/addition.rs`)

```rust
pub fn handle_taskcmd(conn: &Connection, cmd: &TaskCommand) -> Result<(), String> {
    // ... existing code ...

    // Validate project exists in config if specified
    if let Some(ref project_name) = cmd.project {
        if get_project(project_name).is_none() {
            return Err(format!("Project '{}' not found in config", project_name));
        }
    }

    let mut new_task = Item::with_target_time(...);
    new_task.project = cmd.project.clone();
    // ...
}
```

---

### Step 7: Handle Project in Task Update (`src/actions/modify.rs`)

```rust
// In handle_updatecmd():
if let Some(ref project) = cmd.project {
    if get_project(project).is_none() {
        return Err(format!("Project '{}' not found in config", project));
    }
    item.project = Some(project.clone());
}
```

---

### Step 8: Update Display to Show Project (`src/actions/display/`)

Modify task display to show project column:

```
| Index | Project    | Category | Content        | Deadline |
|-------|------------|----------|----------------|----------|
| 1     | myapp      | work     | Fix login bug  | Friday   |
| 2     | (none)     | personal | Buy groceries  | Today    |
```

---

### Step 9: Create Path Conversion Utility

**New file: `src/utils/path.rs`**

```rust
/// Convert Linux path to Windows path for wt.exe
/// /mnt/c/python/myapp -> C:\python\myapp
pub fn linux_to_windows_path(linux_path: &str) -> Result<String, String> {
    if linux_path.starts_with("/mnt/") {
        let parts: Vec<&str> = linux_path.splitn(4, '/').collect();
        if parts.len() >= 4 {
            let drive = parts[2].to_uppercase();
            let rest = parts[3].replace('/', "\\");
            return Ok(format!("{}:\\{}", drive, rest));
        }
    }
    Err(format!("Cannot convert path: {}", linux_path))
}

/// Build the spawn command for launching Claude in a project
pub fn build_spawn_command(
    terminal_profile: &str,
    windows_path: &str,
    conda_env: Option<&str>,
    claude_path: &str,
    claude_flags: Option<&str>,
    prompt: Option<&str>,
) -> String {
    let mut bash_cmd = String::from("export PATH=\\$HOME/.local/bin:\\$PATH");

    if let Some(env) = conda_env {
        bash_cmd.push_str(&format!(" && conda activate {}", env));
    }

    bash_cmd.push_str(&format!(" && {}", claude_path));

    if let Some(flags) = claude_flags {
        bash_cmd.push_str(&format!(" {}", flags));
    }

    if let Some(p) = prompt {
        // Escape quotes in prompt
        let escaped = p.replace('"', "\\\"");
        bash_cmd.push_str(&format!(" \\\"{}\\\"", escaped));
    }

    format!(
        "/init /mnt/c/Windows/System32/cmd.exe /c \"wt.exe -p {} -d {} wsl.exe -e bash -c \\\"{}\\\"\"",
        terminal_profile,
        windows_path,
        bash_cmd
    )
}
```

---

### Step 10: Create `/work` Command (`.claude/commands/work.md`)

```markdown
---
description: Work on a task - opens Claude in the task's project directory
---

Work on the specified task by opening a new Claude session in its project directory.

## Usage
The user will provide a task index (e.g., `/work 3`).

## Steps

1. Parse the task index from the arguments
2. Run `tascli list task -s open` to get current tasks and validate the index
3. Get the task details including the project field
4. If the task has no project, inform the user and offer to work on it in current session
5. If the task has a project:
   a. Look up project config from `~/.config/tascli/config.json`
   b. Convert the Linux path to Windows path
   c. Build the spawn command with:
      - Terminal profile (default: Ubuntu)
      - Project path
      - Conda environment (if specified)
      - Claude flags (if specified)
      - Starting prompt: "Work on task: {task_content}"
   d. Execute the spawn command
   e. Confirm to user: "Opened Claude session for '{task_content}' in {project_path}"

## Spawn Command Template

```bash
/init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p {profile} -d {win_path} wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && {conda_activate} claude {flags} \\\"{prompt}\\\"\""
```

## Example

User: `/work 3`

If task 3 is "Fix login bug" with project "myapp":
1. Look up myapp in config -> path: /mnt/c/python/myapp, conda_env: myapp-env
2. Convert path -> C:\python\myapp
3. Build command with prompt "Work on task: Fix login bug"
4. Execute spawn command
5. Output: "Opened Claude session for 'Fix login bug' in /mnt/c/python/myapp"
```

---

### Step 11: Update Tests

Add tests for:
- Config parsing with projects
- Path conversion (linux_to_windows_path)
- Task creation with project field
- Task update with project field
- Project validation (reject unknown projects)

---

### Step 12: Update Documentation

**`.claude/CLAUDE.md`:**
- Add `/work` command to quick commands table
- Document project configuration
- Add examples

**`.claude/agents/tascli.md`:**
- Add project-related natural language patterns
- Document `-p` flag for tasks

---

## Files to Modify/Create

| File | Action | Lines |
|------|--------|-------|
| `src/config/mod.rs` | Modify | +40 |
| `src/db/conn.rs` | Modify | +8 |
| `src/db/item.rs` | Modify | +6 |
| `src/db/crud.rs` | Modify | +4 |
| `src/args/parser.rs` | Modify | +8 |
| `src/actions/addition.rs` | Modify | +10 |
| `src/actions/modify.rs` | Modify | +8 |
| `src/actions/display/row.rs` | Modify | +15 |
| `src/utils/mod.rs` | Create | +5 |
| `src/utils/path.rs` | Create | +50 |
| `.claude/commands/work.md` | Create | +50 |
| `.claude/CLAUDE.md` | Modify | +15 |
| `.claude/agents/tascli.md` | Modify | +10 |
| **Total** | | **~230 lines** |

---

## Config File Location

`~/.config/tascli/config.json`

User needs to create/edit this file to define projects before using `-p` flag.

---

## Commit Strategy

Split into 2-3 commits:

1. **feat: add project field to tasks**
   - Schema v4, item model, CRUD, parser, addition, modify
   - Tests for project field

2. **feat: add /work command for project sessions**
   - Path conversion utility
   - /work command
   - Documentation updates

---

## Testing Plan

1. Create test config with projects
2. Add task with project: `tascli task "Test" tomorrow -p tascli`
3. Verify project shows in list
4. Run `/work 1` and verify:
   - New window opens
   - Correct directory
   - Claude starts with prompt
   - No PATH warning

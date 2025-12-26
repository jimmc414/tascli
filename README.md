# claude-task-manager

**Claude-first task management for AI-assisted development workflows.**

[![Crates.io](https://img.shields.io/crates/v/claude-task-manager.svg)](https://crates.io/crates/claude-task-manager)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A fast, local CLI that integrates seamlessly with [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Manage tasks through natural language, spawn AI sessions in project directories, and keep your development workflow in sync with your task list.

## Why claude-task-manager?

- **Claude-first design** - Built for developers using Claude Code as their primary development tool
- **Natural language** - Just talk: "Add a task to fix the login bug by Friday"
- **Project-aware sessions** - `/work 3` opens Claude in the task's project directory
- **Fast & local** - <2MB binary, <10ms response, SQLite storage
- **No cloud dependencies** - Your tasks stay on your machine

## Installation

```bash
# From crates.io
cargo install claude-task-manager

# The CLI command is 'ctm'
ctm task "My first task" today
```

## Quick Start with Claude Code

Once installed, use these commands directly in Claude Code:

| Command | What it does |
|---------|--------------|
| `/today` | Show today's tasks and anything overdue |
| `/tasks` | Overview of open tasks |
| `/task fix bug friday -p myapp` | Add task linked to a project |
| `/done 1` | Mark task complete |
| `/work 3` | Open Claude in task's project directory |

Or just talk naturally:
- "What do I need to do today?"
- "Add a task to review the PR by tomorrow"
- "Mark the first task done"
- "Work on the login bug" (opens Claude in project directory)

## Core Features

### Task Management

```bash
# Add tasks with flexible time formats
ctm task "Ship feature" friday
ctm task "Code review" tomorrow -c work
ctm task "Quarterly planning" "next month" -r        # 7-day reminder
ctm task "Fix auth bug" friday -p myapp              # Link to project

# Recurring tasks
ctm task "Standup notes" "weekday 9am"
ctm task "Weekly review" "weekly friday"

# List and manage
ctm list task                    # Open tasks
ctm list task --overdue          # Include overdue
ctm done 1                       # Complete task
ctm done 1 -c "Fixed in PR #42"  # Complete with note
ctm update 2 -t "next week"      # Reschedule
```

### Project Integration

Link tasks to projects for seamless context switching:

```json
// ~/.config/ctm/config.json
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "myapp": {
      "path": "/mnt/c/projects/myapp",
      "claude_flags": "--dangerously-skip-permissions"
    },
    "api": {
      "path": "/mnt/c/projects/api",
      "conda_env": "api-env"
    }
  }
}
```

```bash
# Add task with project
ctm task "Implement OAuth" friday -p myapp

# In Claude Code, open a session in the project directory
/work 1
```

The `/work` command spawns a new Claude Code session in the project's directory with the task context pre-loaded.

### Records

Track completed work and events:

```bash
ctm record "Deployed v2.1 to production"
ctm record -c meetings "Sprint planning - decided on Q1 priorities"
ctm list record -d 7    # Last 7 days
```

## Time Formats

| Format | Examples |
|--------|----------|
| Relative | `today`, `tomorrow`, `friday`, `next week` |
| Absolute | `2025-01-15`, `jan 15`, `1/15` |
| With time | `tomorrow 3pm`, `friday 9:00` |
| Recurring | `daily 9am`, `weekday 9am`, `weekly monday`, `monthly 1st` |
| Special | `eom` (end of month), `eoy` (end of year) |

## Configuration

```json
// ~/.config/ctm/config.json
{
  "data_dir": "/custom/path",           // Default: ~/.local/share/ctm/
  "terminal_profile": "Ubuntu",         // Windows Terminal profile
  "projects": {
    "project-name": {
      "path": "/path/to/project",       // Required
      "conda_env": "env-name",          // Optional: activate conda env
      "claude_flags": "--flag",         // Optional: Claude CLI flags
      "prompt_template": "Work on: {content}"  // Optional: custom prompt
    }
  }
}
```

## Claude Code Integration

This tool is designed to work with Claude Code's agent system. The integration includes:

- **tascli agent** (`.claude/agents/tascli.md`) - Natural language task management
- **Quick commands** (`.claude/commands/`) - `/today`, `/tasks`, `/task`, `/done`, `/work`, etc.
- **Project documentation** (`.claude/CLAUDE.md`) - Full integration guide

When you mention tasks, todos, deadlines, or reminders in Claude Code, the tascli agent is automatically invoked.

## Command Reference

```
ctm <COMMAND>

Commands:
  task    Add task with deadline
  record  Add record/log entry
  done    Mark task complete
  update  Modify task or record
  delete  Remove item
  list    List tasks or records
  help    Show help
```

### Flags

| Flag | Commands | Description |
|------|----------|-------------|
| `-c, --category` | task, record, update | Categorize items |
| `-r, --reminder` | task | Days before due to show in `/today` (default: 7) |
| `-p, --project` | task, update | Link to project for `/work` command |
| `-t, --time` | update | Reschedule to new time |
| `-s, --status` | list, update | Filter/set status |
| `-d, --days` | list | Time range filter |
| `--overdue` | list | Include overdue items |
| `--search` | list | Search content |

## Data Storage

- **Database**: `~/.local/share/ctm/ctm.db` (SQLite)
- **Config**: `~/.config/ctm/config.json` (optional)

## Migration from tascli

If you're migrating from the original tascli:

```bash
# Move your database
mv ~/.local/share/tascli/tascli.db ~/.local/share/ctm/ctm.db

# Update config location
mv ~/.config/tascli/config.json ~/.config/ctm/config.json
```

The database schema is compatible.

## License

MIT

## Credits

Forked from [tascli](https://github.com/Aperocky/tascli) by Aperocky. Extended with Claude Code integration, project-aware sessions, and AI-first workflow features.

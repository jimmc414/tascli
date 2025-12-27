# claude-task-manager

**Claude-first task management for AI-assisted development workflows.**

[![Crates.io](https://img.shields.io/crates/v/claude-task-manager.svg)](https://crates.io/crates/claude-task-manager)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A fast, local CLI that integrates seamlessly with [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Manage tasks through natural language, spawn AI sessions in project directories, track team workloads, and keep your development workflow in sync.

## Why claude-task-manager?

- **Claude-first design** - Built for developers using Claude Code as their primary development tool
- **Natural language** - Just talk: "Add a high priority task to fix the login bug by Friday"
- **Multi-tenant** - Track tasks for yourself and your team with users and namespaces
- **Project-aware sessions** - `/work 3` opens Claude in the task's project directory with full context
- **GitHub integration** - Create tasks from issues, close issues when completing tasks
- **Team reporting** - Workload analysis, completion stats, and team dashboards
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
| `/task fix bug friday -P high` | Add high priority task |
| `/done 1` | Mark task complete |
| `/work 3` | Open Claude in task's project with context |
| `/standup` | Generate daily standup report |

Or just talk naturally:
- "What do I need to do today?"
- "Add a high priority task to review the PR by tomorrow"
- "Show me Sarah's workload"
- "Mark the first task done"
- "Create a task from issue owner/repo#42"

## Core Features

### Task Management

```bash
# Add tasks with flexible time formats
ctm task "Ship feature" friday
ctm task "Code review" tomorrow -c work
ctm task "Quarterly planning" "next month" -r        # 7-day reminder
ctm task "Fix auth bug" friday -p myapp              # Link to project

# Priority and estimates
ctm task "Critical fix" today -P high -e 2h          # High priority, 2 hour estimate
ctm task "Minor update" friday -P low -e 30m         # Low priority, 30 min estimate

# Assign to team members
ctm task "Review docs" tomorrow --for sarah          # Assign to sarah

# Create from GitHub issues
ctm task --from-issue owner/repo#42                  # Import issue as task

# Recurring tasks
ctm task "Standup notes" "weekday 9am"
ctm task "Weekly review" "weekly friday"

# List and manage
ctm list task                    # Open tasks
ctm list task --overdue          # Include overdue
ctm list task -u sarah           # Sarah's tasks
ctm list task --all-users        # Everyone's tasks
ctm done 1                       # Complete task
ctm done 1 -c "Fixed in PR #42"  # Complete with note
ctm done 1 --close-issue         # Complete and close linked GitHub issue
ctm update 2 -t "next week"      # Reschedule
```

### Task Details and Notes

```bash
# View detailed task information
ctm show 3                       # Full details, notes, links

# Add notes to track progress
ctm note 3 "Investigated root cause"
ctm note 3 "Waiting on API team response"

# Attach links (commits, issues, PRs)
ctm link 3 --commit abc123
ctm link 3 --issue owner/repo#42
ctm link 3 --pr owner/repo#43
ctm link 3 --url "https://docs.example.com"

# Claim unassigned tasks
ctm claim 5                      # Take ownership of task 5
```

### Multi-Tenant: Users and Namespaces

Track tasks for yourself and your team:

```bash
# User management
ctm user create sarah -d "Sarah Chen"
ctm user list
ctm user delete sarah

# Namespace management (organize by project/team)
ctm ns create backend -d "Backend team tasks"
ctm ns list
ctm ns switch backend            # Set as default
ctm ns add-user backend sarah --role admin
ctm ns members backend

# Work as a specific user or in a specific namespace
ctm --as sarah list task         # See sarah's view
ctm --ns backend task "Deploy API" friday
```

### Team Reporting

```bash
# Team task distribution
ctm team                         # Who has what tasks
ctm team --json                  # JSON output for integrations
ctm team --md                    # Markdown for documentation

# Workload analysis
ctm workload                     # Hours per person
ctm workload --user sarah        # Single user detail

# Task statistics
ctm stats                        # Last 30 days
ctm stats --days 7               # Last week
ctm stats --json                 # JSON output
```

### GitHub Integration

```bash
# Create task from GitHub issue
ctm task --from-issue owner/repo#42

# Complete task and close linked issue
ctm done 3 --close-issue

# Link existing task to GitHub
ctm link 3 --issue owner/repo#42
ctm link 3 --pr owner/repo#43
```

Requires the [GitHub CLI](https://cli.github.com/) (`gh`) to be installed and authenticated.

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

The `/work` command spawns a new Claude Code session in the project's directory with:
- Task details (priority, estimate, due date)
- All notes and progress updates
- Linked commits, issues, and PRs
- Recent git activity (last 5 commits, open PRs/issues)

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

## Command Reference

```
ctm [OPTIONS] <COMMAND>

Commands:
  task      Add task with deadline
  record    Add record/log entry
  done      Mark task complete
  update    Modify task or record
  delete    Remove item
  list      List tasks or records
  show      Detailed task view
  note      Add note to task
  claim     Claim unassigned task
  link      Attach link to task
  user      Manage users
  ns        Manage namespaces
  team      Team task distribution
  workload  Workload by user
  stats     Task statistics
  help      Show help

Global Options:
  --as <USER>     Act as specific user
  --ns <NAMESPACE> Use specific namespace
```

### Task Flags

| Flag | Description |
|------|-------------|
| `-c, --category` | Categorize task |
| `-r, --reminder` | Days before due to show in `/today` (default: 7) |
| `-p, --project` | Link to project for `/work` command |
| `-P, --priority` | high, normal (default), low (or h/n/l) |
| `-e, --estimate` | Time estimate: 30m, 2h, 1h30m |
| `--for` | Assign to user |
| `--from-issue` | Create from GitHub issue |

### Done Flags

| Flag | Description |
|------|-------------|
| `-c, --comment` | Add completion note |
| `-s, --status` | Status: done, cancelled, duplicate |
| `--close-issue` | Close linked GitHub issue |

### List Flags

| Flag | Description |
|------|-------------|
| `-s, --status` | Filter by status |
| `-d, --days` | Time range |
| `-u, --user` | Filter by assignee |
| `--all-users` | Show all users' tasks |
| `--overdue` | Include overdue |
| `--search` | Search content |

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

### Environment Variables

| Variable | Description |
|----------|-------------|
| `CTM_USER` | Default user (fallback: system $USER) |
| `CTM_NAMESPACE` | Default namespace (fallback: "default") |

## Claude Code Integration

This tool is designed to work with Claude Code's agent system. The integration includes:

- **ctm agent** (`.claude/agents/ctm.md`) - Natural language task management
- **Quick commands** (`.claude/commands/`) - `/today`, `/tasks`, `/task`, `/done`, `/work`, `/standup`, etc.
- **Project documentation** (`.claude/CLAUDE.md`) - Full integration guide

When you mention tasks, todos, deadlines, or reminders in Claude Code, the ctm agent is automatically invoked.

### Slash Commands

| Command | Description |
|---------|-------------|
| `/today` | Today's tasks + overdue |
| `/tasks` | Open tasks overview |
| `/task <desc> <time>` | Quick add task |
| `/done <index>` | Complete task |
| `/overdue` | Overdue items |
| `/work <index>` | Open Claude in project with task context |
| `/standup` | Generate daily standup |
| `/reminders` | Full task summary |

## Data Storage

- **Database**: `~/.local/share/ctm/ctm.db` (SQLite)
- **Config**: `~/.config/ctm/config.json` (optional)

### Database Schema

The database uses schema v5 with support for:
- Users and namespaces (multi-tenant)
- Task ownership and assignment
- Priority and time estimates
- Notes and links
- Audit logging

## Migration from tascli

If you're migrating from the original tascli:

```bash
# Move your database
mv ~/.local/share/tascli/tascli.db ~/.local/share/ctm/ctm.db

# Update config location
mv ~/.config/tascli/config.json ~/.config/ctm/config.json
```

The database schema is compatible and will auto-migrate.

## Status Values

| Status | Code | Description |
|--------|------|-------------|
| ongoing | 0 | In progress |
| done | 1 | Completed |
| cancelled | 2 | Cancelled |
| duplicate | 3 | Duplicate |
| suspended | 4 | On hold |
| pending | 6 | Not started |
| open | 254 | ongoing + pending + suspended |
| closed | 253 | done + cancelled + duplicate |
| all | 255 | All statuses |

## License

MIT

## Credits

Forked from [tascli](https://github.com/Aperocky/tascli) by Aperocky. Extended with Claude Code integration, multi-tenant support, team tracking, GitHub integration, and AI-first workflow features.

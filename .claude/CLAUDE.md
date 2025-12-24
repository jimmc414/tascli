# tascli - Claude Code Integration

This project includes Claude Code integration for task and reminder management.

## Installation

### Prerequisites
- Rust toolchain (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Build tools (`sudo apt-get install build-essential` on Ubuntu/Debian)

### Install tascli
```bash
# From the project directory
cargo install --path .

# Or from crates.io
cargo install tascli
```

### Verify installation
```bash
tascli --help
```

## Claude Code Components

### Agent: tascli
Location: `.claude/agents/tascli.md`

The tascli agent handles all task management operations. It is automatically invoked when you mention tasks, reminders, or todo items.

**Capabilities:**
- Add tasks and records
- Create recurring tasks (daily, weekly, monthly)
- List and filter tasks by category, status, or date
- Mark tasks complete
- Update or delete items

**Example prompts:**
- "Add a task to review the quarterly report by Friday"
- "Show me my overdue tasks"
- "Mark task 3 as done"
- "Add a recurring task for standup every weekday at 9am"

### Command: /reminders
Location: `.claude/commands/reminders.md`

Invoke with `/reminders` to get a daily task overview showing:
- Overdue tasks needing attention
- Tasks due today
- Tasks due this week
- Pending recurring tasks

## Data Storage

Tasks are stored in SQLite at `~/.local/share/tascli/tascli.db`

Configuration (optional): `~/.config/tascli/config.json`

## Quick Reference

### Adding Items
```bash
tascli task "description" [timestr] [-c category]
tascli record "description" [-c category]
```

### Time Formats
- Relative: `today`, `tomorrow`, `next week`, `in 3 days`
- Absolute: `2024-01-15`, `jan 15`, `monday`
- With time: `tomorrow 3pm`, `monday 9:00`
- Recurring: `daily 9am`, `weekday 9am`, `weekly monday`, `monthly 1st`

### Listing
```bash
tascli list task              # Open tasks
tascli list task -s all       # All tasks
tascli list task --overdue    # Include overdue
tascli list task -d 7         # Due in 7 days
tascli list task -c work      # Filter by category
tascli list record -d 7       # Records from past week
```

### Managing
```bash
tascli done <index>                    # Complete task
tascli done <index> -c "note"          # Complete with comment
tascli update <index> -t "tomorrow"    # Reschedule
tascli update <index> -s cancelled     # Change status
tascli delete <index>                  # Delete item
```

### Status Values
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

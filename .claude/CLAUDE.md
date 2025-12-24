# tascli - Claude Code Integration

Claude-first task management. Use natural language or quick commands.

## Installation

```bash
# From crates.io
cargo install tascli

# Or from source
cargo install --path .

# Or via Homebrew
brew tap Aperocky/tascli && brew install tascli
```

## Quick Commands

| Command | Purpose | Example |
|---------|---------|---------|
| `/today` | Today's tasks + overdue | Quick daily view |
| `/tasks` | Show open tasks | Overview of today/week |
| `/task` | Quick add | `/task review PR tomorrow` |
| `/done` | Mark complete | `/done 1` |
| `/overdue` | Show overdue | What needs attention |
| `/reminders` | Daily overview | Full task summary |

## Natural Language

Just talk to Claude. The tascli agent understands:

- "What tasks do I have today?"
- "Add a task to review the PR by Friday"
- "Mark the first task done"
- "Reschedule task 2 to next week"
- "What did I complete today?"
- "Show me everything overdue"

## Agent: tascli

Location: `.claude/agents/tascli.md`

Automatically invoked when you mention tasks, reminders, todo, deadlines, or schedules.

**Capabilities:**
- Natural language task management
- Add tasks and records
- Create recurring tasks (daily, weekly, monthly)
- List and filter by category, status, or date
- Mark tasks complete with optional notes
- Reschedule or cancel tasks

## Data Storage

Tasks are stored in SQLite at `~/.local/share/tascli/tascli.db`

Configuration (optional): `~/.config/tascli/config.json`

## Quick Reference

### Adding Items
```bash
tascli task "description" [timestr] [-c category]
tascli task "description" [timestr] -r        # With 7-day reminder
tascli task "description" [timestr] -r 14     # With 14-day reminder
tascli record "description" [-c category]
```

**Reminder flag (-r):** Tasks appear in `/today` when within their reminder window, even if not yet due.

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

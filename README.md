# tascli

[![Crates.io](https://img.shields.io/crates/v/tascli.svg)](https://crates.io/crates/tascli)
[![tests](https://github.com/Aperocky/tascli/workflows/tests/badge.svg)](https://github.com/Aperocky/tascli/actions?query=workflow%3Atests)
[![benchmark](https://github.com/Aperocky/tascli/workflows/benchmark/badge.svg)](https://github.com/Aperocky/tascli/actions?query=workflow%3Abenchmark)
![Downloads](https://img.shields.io/crates/d/tascli.svg)

A *small (<2MB), simple, fast (<10ms), local* CLI tool for tracking tasks and records from unix terminal.

Installation:

```bash
cargo install tascli
# or use brew
brew tap Aperocky/tascli
brew install tascli
```

![tascli demo](demo/tascli.gif)

## Basic Usage

Tasks and records are stored in `~/.local/share/tascli/tascli.db` (configurable) with `sqlite`.

### Tasks

Create tasks with deadlines:
```bash
# Basic tasks
tascli task "Create readme" today
tascli task "Publish package" tomorrow
tascli task "Do taxes" 4/15

# With category
tascli task -c work "Read emails" week

# With reminder (shows in today view 7 days before due)
tascli task "Quarterly review" "next month" -r

# With project association (for Claude Code integration)
tascli task "Fix login bug" friday -p myapp
```

Create recurring tasks:

```bash
tascli task "write diary" daily
tascli task "mortgage payment" "monthly 17th"
```

List tasks:
```bash
# List active tasks
$ tascli list task
```
output:
```
Task List:
----------------------------------------------------------------------------------------------
| Index  | Category            | Content                               | Deadline            |
----------------------------------------------------------------------------------------------
| 1      | life (recurring)    | write diary                           | Today               |
----------------------------------------------------------------------------------------------
| 2      | tascli              | Add pagination capability for tascli  | Sunday              |
|        |                     | list actions                          |                     |
----------------------------------------------------------------------------------------------
| 3      | tascli              | Add readme section on timestring      | Sunday              |
|        |                     | format                                |                     |
----------------------------------------------------------------------------------------------
| 4      | life                | Do state taxes                        | Sunday              |
----------------------------------------------------------------------------------------------
| 5      | tascli              | Sort list output by time instead of   | Sunday              |
|        |                     | internal id                           |                     |
----------------------------------------------------------------------------------------------
| 6      | tascli              | Fix length issue for unicode chars    | Sunday              |
----------------------------------------------------------------------------------------------
| 7      | life                | Two month pictures - follow the lead  | 4/23                |
|        |                     | from the previous one month pictures  |                     |
----------------------------------------------------------------------------------------------
```

Complete tasks:
```bash
# Mark index 1 as done
tascli done 1
```

Completing a task or a recurring tasks will generate a corresponding record.

Search tasks:
```bash
tascli list task --search "rust"
```

List all tasks in `tascli` category (including completed)
```bash
tascli list task -s all -c tascli
```

Example output:
```
Task List:
----------------------------------------------------------------------------------------------
| Index  | Category            | Content                               | Deadline            |
----------------------------------------------------------------------------------------------
| 1      | baby (Recurring)    | Mix egg yolk milk for Rowan           | Daily (fulfilled)   |
----------------------------------------------------------------------------------------------
| 2      | tascli              | Fix addition and modification commands| Today (completed)   |
|        |                     | output to have N/A for index          |                     |
----------------------------------------------------------------------------------------------
| 3      | tascli              | Insert guardrail against accidental   | Today (completed)   |
|        |                     | valid syntax like 'task list' that is |                     |
|        |                     | mistakenly made                       |                     |
----------------------------------------------------------------------------------------------
| 4      | tascli              | Create a gif for readme               | Today (completed)   |
----------------------------------------------------------------------------------------------
| 5      | tascli              | Add pagination capability for tascli  | Sunday              |
|        |                     | list actions                          |                     |
----------------------------------------------------------------------------------------------
| 6      | tascli              | Add readme section on timestring      | Sunday              |
|        |                     | format                                |                     |
----------------------------------------------------------------------------------------------
```

### Records

Create records (for tracking events):
```bash
# With current time
tascli record -c feeding "100ML"

# With specific time
tascli record -c feeding -t 11:20AM "100ML"
```

List records:
```bash
# -d 1 stand for only get last 1 day of record
tascli list record -d 1
```

Search records:
```bash
tascli list record --search "secret"
```

Example output:
```
Records List:
----------------------------------------------------------------------------------------------
| Index  | Category            | Content                               | Created At          |
----------------------------------------------------------------------------------------------
| 1      | feeding             | 110ML                                 | Today 1:00AM        |
----------------------------------------------------------------------------------------------
| 2      | feeding             | breastfeeding                         | Today 4:10AM        |
----------------------------------------------------------------------------------------------
| 3      | feeding             | 100ML                                 | Today 7:30AM        |
----------------------------------------------------------------------------------------------
| 3      | life (Recurring)    | write diary                           | Today 10:30AM       |
----------------------------------------------------------------------------------------------
| 4      | feeding             | 110ML                                 | Today 11:20AM       |
----------------------------------------------------------------------------------------------
```

### Time Format

This application accepts flexible time strings in various formats:

- **Simple dates**: `today`, `tomorrow`, `yesterday`, `friday`, `eom` (end of month), `eoy` (end of year)
- **Date formats**: `YYYY-MM-DD`, `MM/DD/YYYY`, `MM/DD` (current year)
- **Time formats**: `HH:MM`, `3:00PM`, `3PM`
- **Combined**: `2025-03-24 15:30`, `tomorrow 3PM`

When only a date is provided, the time defaults to end of day (23:59:59). When only a time is provided, the date defaults to today.

Recurring Formats (schedules) are applicable to tasks:

- **Recurring Formats**: `daily`, `daily 9PM`, `weekly`, `weekly Friday 9AM`, `weekly mon-fri`, `monthly 1st`
- **Recurring Formats (II)**: `every day`, `every 9PM`, `every monday`, `every 9th of the month`, `every 2/14`

### Configuration

Create a config file at `~/.config/tascli/config.json` to customize settings:

```json
{
    "data_dir": "/where/you/want/it",
    "terminal_profile": "Ubuntu",
    "projects": {
        "myapp": {
            "path": "/mnt/c/python/myapp",
            "conda_env": "myapp-env",
            "claude_flags": "--dangerously-skip-permissions"
        },
        "tascli": {
            "path": "/mnt/c/python/tascli"
        }
    }
}
```

**Configuration options:**
- `data_dir`: Custom location for the SQLite database (default: `~/.local/share/tascli/`)
- `terminal_profile`: Windows Terminal profile name for `/work` command (default: "Ubuntu")
- `projects`: Project definitions for the `-p` flag and `/work` command
  - `path`: Linux path to project directory (required)
  - `conda_env`: Conda environment to activate (optional)
  - `claude_flags`: Additional Claude CLI flags (optional)
  - `prompt_template`: Custom prompt template using `{content}` (optional)

Note: If you already have existing tasks, move/copy the db file before changing `data_dir`.

### Claude Code Integration

tascli integrates with Claude Code for AI-assisted development workflows:

```bash
# Add task with project association
tascli task "Fix login bug" friday -p myapp

# In Claude Code, use /work to open a new session in the project directory
/work 3  # Opens Claude in myapp's directory with task context
```

See `.claude/CLAUDE.md` for full Claude Code integration documentation including:
- Quick commands (`/today`, `/tasks`, `/task`, `/done`, `/work`, etc.)
- Natural language task management via the tascli agent
- Project-aware development sessions

### Help

`tascli` uses `clap` for argument parsing, use `--help` to get help on all levels of this cli:

```
aperocky@~$ tascli -h
Usage: tascli <COMMAND>

Commands:
  task    add task with end time
  record  add record
  done    Finish tasks
  update  Update tasks or records wording/deadlines
  delete  Delete Records or Tasks
  list    list tasks or records
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
aperocky@~$ tascli task -h
add task

Usage: tascli task [OPTIONS] <CONTENT> [TIMESTR]

Arguments:
  <CONTENT>  description of the task
  [TIMESTR]  time the task is due for completion, default to EOD

Options:
  -c, --category <CATEGORY>    category of the task
  -r, --reminder [<REMINDER>]  reminder days before due date (default: 7)
  -p, --project <PROJECT>      project name (must be defined in config)
  -h, --help                   Print help
```

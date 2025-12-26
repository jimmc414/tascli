---
description: Quick add a task (e.g., /task review PR tomorrow)
---

Add a new task using the provided arguments.

Parse the input as: `description [timestr] [-c category] [-r [days]] [-p project]`

Examples:
- `/task review PR tomorrow` → `tascli task "review PR" tomorrow`
- `/task "submit report" friday -c work` → `tascli task "submit report" friday -c work`
- `/task standup daily 9am` → `tascli task "standup" "daily 9am"`
- `/task quarterly review next month -r` → `tascli task "quarterly review" "next month" -r`
- `/task fix login bug friday -p myapp` → `tascli task "fix login bug" friday -p myapp`

**Flags:**
- `-c category` - Assign a category
- `-r [days]` - Set reminder window (default: 7 days)
- `-p project` - Link to a project (must be defined in config)

If no timestr is provided, default to "today".

Run the tascli command and confirm the task was added.

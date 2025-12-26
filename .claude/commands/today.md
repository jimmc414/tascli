---
description: Show today's tasks, reminders, and any overdue items
---

Show my tasks for today and anything overdue.

Run these commands:
1. `ctm list task --overdue -s open` - Show overdue items first
2. `ctm list task -d 1 -s open` - Tasks due today (includes tasks with reminders in their window)

Note: Tasks with `-r` (reminder) flag will appear when within their reminder window, even if not yet due.

Present a concise summary. Highlight overdue items as needing immediate attention.

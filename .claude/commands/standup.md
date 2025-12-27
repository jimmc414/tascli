---
description: Generate a daily standup report from tasks and records
---

Generate a standup report summarizing yesterday's completions, today's plan, and any blockers.

## Usage

```
/standup
/standup --md       # Output as markdown (for pasting into Slack/Teams)
/standup --json     # Output as JSON
```

Arguments: `$ARGUMENTS`

## Steps

1. **Gather Yesterday's Completions:**
   ```bash
   ctm list record -d 1
   ```
   This shows records from the past 24 hours (completed tasks generate records).

2. **Gather Today's Tasks:**
   ```bash
   ctm list task today -s open
   ctm list task tomorrow -s open   # Also include tomorrow's tasks for context
   ```

3. **Gather Blockers/Overdue:**
   ```bash
   ctm list task --overdue -s open
   ```
   Tasks that are past due and not closed are blockers.

4. **Gather Suspended Tasks (Optional blockers):**
   ```bash
   ctm list task -s suspended
   ```
   Suspended tasks often represent blocked work.

5. **Format the Output:**

## Default Output Format

```
# Daily Standup - [Today's Date]

## Yesterday
- Completed task 1
- Completed task 2
- [If no completions: "No tasks completed yesterday"]

## Today
- [ ] Task due today 1
- [ ] Task due today 2
- [ ] Task due tomorrow (upcoming)
- [If no tasks: "No tasks scheduled for today"]

## Blockers
- [OVERDUE] Task 3 days overdue
- [SUSPENDED] Waiting on API access
- [If no blockers: "No blockers"]
```

## Markdown Output (--md)

When `--md` is passed, format for easy pasting into chat:

```markdown
### Daily Standup - December 27, 2025

**Yesterday:**
- Completed task 1
- Completed task 2

**Today:**
- [ ] Task due today
- [ ] Another task

**Blockers:**
- :warning: Task 3 days overdue
- :pause_button: Waiting on API access
```

## JSON Output (--json)

When `--json` is passed:

```json
{
  "date": "2025-12-27",
  "yesterday": [
    {"content": "Completed task 1", "category": "work"},
    {"content": "Completed task 2", "category": "default"}
  ],
  "today": [
    {"index": 1, "content": "Task due today", "priority": "normal", "estimate": "2h"},
    {"index": 2, "content": "Another task", "priority": "high", "estimate": null}
  ],
  "blockers": [
    {"index": 3, "content": "Overdue task", "days_overdue": 3, "type": "overdue"},
    {"index": 4, "content": "Waiting on API", "type": "suspended"}
  ]
}
```

## Example

User: `/standup`

Output:
```
# Daily Standup - Friday, December 27, 2025

## Yesterday
- Fixed login timeout bug (work)
- Updated documentation (docs)
- Reviewed PR #45 (default)

## Today
- [ ] Implement caching layer [HIGH, 4h]
- [ ] Write unit tests [2h]
- [ ] Team sync meeting

## Blockers
- [OVERDUE: 2 days] Deploy to staging - waiting on DevOps
- [SUSPENDED] API integration - external team dependency
```

## Implementation Notes

1. **Record Interpretation:**
   - Records created by `ctm done` contain the completed task's content
   - The category often indicates the work area

2. **Priority Display:**
   - HIGH priority tasks shown with [HIGH] tag
   - Include estimate if available: [2h] or [30m]

3. **Blocker Classification:**
   - OVERDUE: Tasks past their due date
   - SUSPENDED: Tasks explicitly put on hold

4. **Time Handling:**
   - "Yesterday" = past 24 hours of records
   - "Today" = tasks due today (including already due but not overdue)
   - Consider timezone (use local time)

5. **Empty Sections:**
   - Always show all three sections
   - If a section is empty, show a helpful message

## Related Commands

- `/today` - Quick view of today's tasks
- `/tasks` - Full task list
- `/overdue` - Just the overdue items

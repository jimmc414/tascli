---
description: Work on a task - opens Claude in the task's project directory
---

Work on the specified task by opening a new Claude session in its project directory with rich context.

## Usage

The user provides a task index: `/work 3`

## Steps

1. Parse the task index from the arguments: `$ARGUMENTS`

2. Get the task list to find the task:
   ```bash
   ctm list task -s open
   ```

3. Identify the task at the given index and check if it has a project field.

4. If the task has NO project:
   - Inform the user: "Task [index] '[content]' has no project associated."
   - Offer to work on it in the current session instead.

5. If the task HAS a project:

   **a. Gather Task Context:**

   Run `ctm show <index>` to get detailed task information including:
   - Task content and description
   - Priority and estimate
   - Owner and assignee
   - Notes (timestamped progress updates)
   - Links (commits, issues, PRs, URLs)

   **b. Gather Project Context (if in project directory):**

   - Last 5 commits: `git log --oneline -5`
   - Open PRs: `gh pr list --state open --limit 5` (if gh available)
   - Open issues: `gh issue list --state open --limit 5` (if gh available)

   Note: These git/gh commands should only run if the project path exists and is a git repo.

   **c. Load Project Configuration:**

   Load config from `~/.config/ctm/config.json` and extract:
   - path: Linux path to project
   - conda_env: Optional conda environment
   - claude_flags: Optional Claude CLI flags
   - prompt_template: Optional custom template (use `{content}` for task content, `{context}` for full context)

   **d. Build Rich Prompt:**

   Construct a prompt with all gathered context:
   ```
   Work on task: [task content]

   ## Task Details
   Priority: [priority]
   Estimate: [estimate]
   Due: [target date]

   ## Notes
   [timestamp] [note content]
   ...

   ## Links
   [type] [reference] - [title]
   ...

   ## Recent Activity (if available)
   Last 5 commits:
   [git log output]

   Open PRs:
   [pr list]

   Open Issues:
   [issue list]
   ```

   **e. Convert Path and Spawn:**

   - Convert Linux path to Windows: `/mnt/c/python/myapp` -> `C:\python\myapp`
   - Build spawn command:
     ```
     /init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p {profile} -d {win_path} wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && {conda_activate} claude {flags} \\\"{prompt}\\\"\""
     ```
   - Execute spawn command
   - Confirm: "Opened Claude session for '[task_content]' in {project_path}"

## Config File Format

The config file at `~/.config/ctm/config.json`:

```json
{
  "terminal_profile": "Ubuntu",
  "projects": {
    "ctm": {
      "path": "/mnt/c/python/claude-task-manager"
    },
    "myapp": {
      "path": "/mnt/c/python/myapp",
      "conda_env": "myapp-env",
      "claude_flags": "--dangerously-skip-permissions",
      "prompt_template": "Continue working on: {content}\n\nContext:\n{context}"
    }
  }
}
```

## Path Conversion

Convert Linux WSL paths to Windows paths:
- `/mnt/c/python/myapp` -> `C:\python\myapp`
- `/mnt/d/projects/foo` -> `D:\projects\foo`

## Spawn Command Details

The spawn command uses:
- `/init` - WSL interop wrapper (bypasses execute permission issues)
- `wt.exe -p Ubuntu` - Windows Terminal with specified profile
- `-d C:\path` - Windows-format working directory
- `wsl.exe -e bash -c "..."` - Execute bash command in WSL
- `export PATH=...` - Ensure Claude is in PATH (native installation)
- `conda activate` - Optional conda environment activation
- `claude {flags}` - Claude with optional flags
- `"{prompt}"` - Rich context prompt

## Example

User: `/work 3`

If task 3 is "Fix login timeout bug" with project "myapp":

1. Run `ctm show 3` to get:
   - Priority: HIGH
   - Notes: "User reported 5s load times", "Tried redis, too slow"
   - Links: issue myapp#42

2. Check project directory for git/gh context:
   - Last commits: "a1b2c3d Add retry logic", "f4e5d6c Update config"
   - Open PRs: #45 "Add caching layer"

3. Build prompt:
   ```
   Work on task: Fix login timeout bug

   ## Task Details
   Priority: HIGH
   Project: myapp

   ## Notes
   [Dec 26, 10:30] User reported 5s load times
   [Dec 26, 14:15] Tried redis, too slow

   ## Links
   [issue] myapp#42

   ## Recent Activity
   Last 5 commits:
   a1b2c3d Add retry logic
   f4e5d6c Update config

   Open PRs:
   #45 Add caching layer
   ```

4. Spawn Claude in `/mnt/c/python/myapp` with this context

5. Output: "Opened Claude session for 'Fix login timeout bug' in /mnt/c/python/myapp"

## Error Handling

- If task index is invalid, show error with valid range
- If task has no project, offer to work in current session
- If project not found in config, show error with config location
- If path conversion fails, show error with path format requirements
- If git/gh commands fail, proceed without that context (graceful degradation)

## Implementation Notes

The context gathering is designed for graceful degradation:
- If `ctm show` fails, use basic task info from list
- If git commands fail (not a repo), skip recent commits
- If gh is not available, skip PR/issue context
- Always proceed with whatever context is available

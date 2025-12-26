---
description: Work on a task - opens Claude in the task's project directory
---

Work on the specified task by opening a new Claude session in its project directory.

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
   a. Load the project configuration from `~/.config/ctm/config.json`
   b. Extract project settings: path, conda_env, claude_flags, prompt_template
   c. Convert the Linux path to Windows path:
      - `/mnt/c/python/myapp` → `C:\python\myapp`
   d. Get the terminal profile (default: "Ubuntu")
   e. Build the spawn command using this template:
      ```
      /init /mnt/c/Windows/System32/cmd.exe /c "wt.exe -p {profile} -d {win_path} wsl.exe -e bash -c \"export PATH=\$HOME/.local/bin:\$PATH && {conda_activate} claude {flags} \\\"{prompt}\\\"\""
      ```
   f. Execute the spawn command
   g. Confirm: "Opened Claude session for '[task_content]' in {project_path}"

## Config File Format

The config file at `~/.config/ctm/config.json` should look like:

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
      "prompt_template": "Continue working on: {content}"
    }
  }
}
```

## Path Conversion

Convert Linux WSL paths to Windows paths:
- `/mnt/c/python/myapp` → `C:\python\myapp`
- `/mnt/d/projects/foo` → `D:\projects\foo`

## Spawn Command Details

The spawn command uses:
- `/init` - WSL interop wrapper (bypasses execute permission issues)
- `wt.exe -p Ubuntu` - Windows Terminal with specified profile
- `-d C:\path` - Windows-format working directory
- `wsl.exe -e bash -c "..."` - Execute bash command in WSL
- `export PATH=...` - Ensure Claude is in PATH (native installation)
- `conda activate` - Optional conda environment activation
- `claude {flags}` - Claude with optional flags
- `"{prompt}"` - Optional starting prompt

## Example

User: `/work 3`

If task 3 is "Fix login bug" with project "myapp":
1. Look up myapp in config → path: /mnt/c/python/myapp, conda_env: myapp-env
2. Convert path → C:\python\myapp
3. Build prompt: "Work on task: Fix login bug"
4. Execute spawn command
5. Output: "Opened Claude session for 'Fix login bug' in /mnt/c/python/myapp"

## Error Handling

- If task index is invalid, show error with valid range
- If task has no project, offer to work in current session
- If project not found in config, show error with config location
- If path conversion fails, show error with path format requirements

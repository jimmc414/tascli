# Resume Prompt: CTM Multi-Tenant Implementation

Use this prompt to resume work after context compaction.

---

## Context

The multi-tenant implementation for claude-task-manager (ctm) is **COMPLETE**. All 8 phases have been implemented.

**Project location:** `/mnt/c/python/claude-task-manager`

## Current Status

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Schema v5 Migration | COMPLETE |
| 2 | Identity Context System | COMPLETE |
| 3 | User/Namespace Commands | COMPLETE |
| 4 | Task Enhancements | COMPLETE |
| 5 | Notes/Show/Claim/Link | COMPLETE |
| 6 | Reporting Commands | COMPLETE |
| 7 | GitHub Integration | COMPLETE |
| 8 | /work + /standup | COMPLETE |

**All 143 tests pass.**

---

## What Was Implemented

### Phase 8: /work + /standup
- Enhanced `.claude/commands/work.md` with rich context:
  - Task details via `ctm show <index>`
  - Project context (git log, gh pr/issue list)
  - Graceful degradation when git/gh unavailable
- Created `.claude/commands/standup.md`:
  - Yesterday (completions), Today (planned), Blockers (overdue/suspended)
  - Support for --md and --json output formats

### Phase 7: GitHub Integration
- `src/github/api.rs` - gh CLI wrapper
- `--from-issue` flag on TaskCommand
- `--close-issue` flag on DoneCommand

### Phase 6: Reporting Commands
- `ctm team`, `ctm workload`, `ctm stats`
- Output formats: text, `--json`, `--md`

### Phases 1-5: Multi-Tenant Foundation
- Schema v5 with users, namespaces, task_links, task_notes, audit_log
- Context/identity system with `--as` and `--ns` flags
- User/namespace CRUD commands
- Task priority, estimates, assignees
- Notes, show, claim, link commands

---

## Key Files

- **Full plan:** `/home/jim/.claude/plans/modular-fluttering-aurora.md`
- **Checkpoint:** `.claude/checkpoint.md`
- **GitHub module:** `src/github/api.rs`
- **CLI commands:** `src/args/parser.rs`
- **Command handlers:** `src/actions/handler.rs`
- **Slash commands:** `.claude/commands/work.md`, `.claude/commands/standup.md`

---

## Design Decisions (Reference)

- Single DB, multi-tenant (not separate DBs per namespace)
- Single manager model (one person runs ctm, tracks team)
- Roles are per-namespace (owner/admin/member/viewer)
- Task ownership: owner_id (accountable) + assignee_id (working)
- Unassigned tasks allowed, claimable via `ctm claim`
- Identity: --as flag → CTM_USER env → system $USER
- Auto-setup on first run (create user + default namespace)
- GitHub: use `gh` CLI wrapper, not HTTP API
- Future-proof for concurrent access (proper IDs, audit trails)
- Reports support --json and --md output flags

---

## Available Commands

### Task Management
```bash
ctm task "description" [timestr] [-c category] [-P priority] [-e estimate] [--for user]
ctm task --from-issue owner/repo#42
ctm done <index> [-c comment] [--close-issue]
ctm update <index> [-t time] [-c category] [-s status] [-p project]
ctm delete <index>
ctm list task [timestr] [-s status] [-u user] [--all-users] [--overdue]
ctm show <index>
ctm note <index> "note text"
ctm claim <index>
ctm link <index> --issue owner/repo#42
```

### User/Namespace Management
```bash
ctm user create <name>
ctm user list
ctm user delete <name>
ctm ns create <name>
ctm ns list
ctm ns switch <name>
ctm ns add-user <ns> <user> [--role admin]
ctm ns remove-user <ns> <user>
ctm ns members [namespace]
```

### Reporting
```bash
ctm team [--json] [--md]
ctm workload [--user user] [--json] [--md]
ctm stats [--days 30] [--json] [--md]
```

### Slash Commands
```bash
/work <index>        # Open Claude with rich task+project context
/standup             # Generate standup report
/standup --md        # Markdown format for chat
/standup --json      # JSON format for integrations
```

### Global Flags
```bash
ctm --as sarah list task    # Act as user
ctm --ns work list task     # Use namespace
```

---

## Future Enhancements (Not Planned)

If continuing development, consider:
- Web UI or API server
- Real-time collaboration
- Calendar integration
- Time tracking
- Sprint/milestone management

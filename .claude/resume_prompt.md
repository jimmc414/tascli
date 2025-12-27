# Resume Prompt: CTM Multi-Tenant Implementation

Use this prompt to resume work after context compaction.

---

## Context

I'm implementing a major feature set for claude-task-manager (ctm), a Rust CLI task manager. **Phases 1-7 are complete, ready for Phase 8.**

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
| 8 | /work + /standup | **NEXT** |

**All 143 tests pass.**

## What Was Completed

### Phase 7: GitHub Integration
- New files: `src/github/mod.rs`, `src/github/api.rs`
- gh CLI wrapper: `parse_issue_ref()`, `is_gh_available()`, `get_issue()`, `close_issue()`
- `--from-issue` flag on TaskCommand
- `--close-issue` flag on DoneCommand
- `handle_from_issue()` in addition.rs
- `close_linked_issue()` in modify.rs

### Phase 6: Reporting Commands
- New file: `src/actions/reporting.rs`
- Commands: `ctm team`, `ctm workload`, `ctm stats`
- Output formats: text, `--json`, `--md`

### Phase 1-5: Multi-Tenant Foundation
- Schema v5 with users, namespaces, task_links, task_notes, audit_log
- Context/identity system with `--as` and `--ns` flags
- User/namespace CRUD commands
- Task priority, estimates, assignees
- Notes, show, claim, link commands

## Key Files

- **Full plan:** `/home/jim/.claude/plans/modular-fluttering-aurora.md`
- **Checkpoint:** `.claude/checkpoint.md`
- **GitHub module:** `src/github/api.rs`
- **CLI commands:** `src/args/parser.rs`
- **Command handlers:** `src/actions/handler.rs`

## To Resume

```
1. Read checkpoint: .claude/checkpoint.md
2. Start Phase 8: /work + /standup
   - Modify .claude/commands/work.md - Enhanced /work context
   - Create .claude/commands/standup.md - Standup generation
```

## Design Decisions (Don't Re-Ask)

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

## Phase 7 Commands (COMPLETE)

```bash
# Create task from GitHub issue
ctm task --from-issue owner/repo#42

# Complete task and close linked issue
ctm done 3 --close-issue
```

## Phase 8 Commands to Implement

```bash
# Enhanced /work context with task notes, links, last commits
/work <index>

# Standup generation from yesterday's completions and today's tasks
/standup
```

## Global Flags (Already Implemented)

```bash
ctm --as sarah list task    # Act as user
ctm --ns work list task     # Use namespace
```

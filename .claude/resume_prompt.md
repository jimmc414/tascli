# Resume Prompt: CTM Multi-Tenant Implementation

Use this prompt to resume work after context compaction.

---

## Context

I'm implementing a major feature set for claude-task-manager (ctm), a Rust CLI task manager. **Phases 1-4 are complete, ready for Phase 5.**

**Project location:** `/mnt/c/python/claude-task-manager`

## Current Status

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Schema v5 Migration | COMPLETE |
| 2 | Identity Context System | COMPLETE |
| 3 | User/Namespace Commands | COMPLETE |
| 4 | Task Enhancements | COMPLETE |
| 5 | Notes/Show/Claim/Link | **NEXT** |
| 6 | Reporting Commands | Not started |
| 7 | GitHub Integration | Not started |
| 8 | /work + /standup | Not started |

**All 87 tests pass.**

## What Was Completed

### Phase 1: Schema v5
- New tables: `users`, `namespaces`, `user_namespaces`, `task_links`, `task_notes`, `audit_log`
- New `items` columns: `owner_id`, `assignee_id`, `namespace_id`, `priority`, `estimate_minutes`, `github_issue`
- Auto-setup creates default user + namespace on first run
- Files: `src/db/conn.rs`, `src/db/item.rs`, `src/db/crud.rs`

### Phase 2: Identity Context
- `Context` struct resolves current user/namespace
- Global `--as` and `--ns` CLI flags added
- Files: `src/context/mod.rs`, `src/context/identity.rs`, `src/main.rs`, `src/actions/handler.rs`, `src/args/parser.rs`

### Phase 3: User/Namespace Commands
- User struct + CRUD: `src/db/user.rs`
- Namespace struct + CRUD: `src/db/namespace.rs`
- User command handlers: `src/actions/user.rs`
- Namespace command handlers: `src/actions/namespace.rs`
- Commands: `ctm user create/list/delete`, `ctm ns create/list/delete/switch/add-user/remove-user/members`

### Phase 4: Task Enhancements
- Priority parser: `src/args/priority.rs` (high/normal/low → 0/1/2)
- Estimate parser: `src/args/estimate.rs` (2h/30m/1h30m → minutes)
- TaskCommand flags: `-P`, `-e`, `--for` (priority, estimate, assignee)
- ListTaskCommand flags: `-u/--user`, `--all-users` (user filtering)
- ItemQuery filters: assignee_id, owner_id, namespace_id
- Tasks now store: owner_id, assignee_id, namespace_id, priority, estimate_minutes

## Key Files

- **Full plan:** `/home/jim/.claude/plans/modular-fluttering-aurora.md`
- **Checkpoint:** `.claude/checkpoint.md`
- **Schema migrations:** `src/db/conn.rs`
- **CLI commands:** `src/args/parser.rs`
- **Command handlers:** `src/actions/handler.rs`
- **Context:** `src/context/identity.rs`

## To Resume

```
1. Read checkpoint: .claude/checkpoint.md
2. Start Phase 5: Notes/Show/Claim/Link
   - Create src/db/note.rs - TaskNote struct + CRUD
   - Create src/db/link.rs - TaskLink struct + CRUD
   - Create src/actions/note.rs - Note command handler (ctm note <index> "text")
   - Create src/actions/show.rs - Detailed view (ctm show <index>)
   - Create src/actions/claim.rs - Take ownership (ctm claim <index>)
   - Add Link command for attaching commits/issues/PRs to tasks
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

## Phase 5 Commands to Implement

```bash
# Notes
ctm note <index> "note text"     # Append timestamped note

# Detailed view
ctm show <index>                 # Full task details with notes/links/history

# Claiming
ctm claim <index>                # Take ownership of unassigned task

# Links
ctm link <index> --issue owner/repo#42
ctm link <index> --pr owner/repo#43
ctm link <index> --commit abc123
```

## Global Flags (Already Implemented)

```bash
ctm --as sarah list task    # Act as user
ctm --ns work list task     # Use namespace
```

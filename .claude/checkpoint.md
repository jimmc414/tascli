# Checkpoint: Multi-Tenant CTM Implementation

**Date:** 2025-12-26
**Status:** Phases 1-4 COMPLETE, ready for Phase 5

---

## Completed This Session

### Phase 1: Schema v5 Migration ✓
- Updated `SCHEMA_VERSION` from 4 to 5 in `src/db/conn.rs`
- Created 6 new tables: `users`, `namespaces`, `user_namespaces`, `task_links`, `task_notes`, `audit_log`
- Added 6 new columns to `items`: `owner_id`, `assignee_id`, `namespace_id`, `priority`, `estimate_minutes`, `github_issue`
- Added indexes: `idx_owner_id`, `idx_assignee_id`, `idx_namespace_id`, `idx_priority`, `idx_task_links_item_id`, `idx_task_notes_item_id`, `idx_audit_log_item_id`
- Implemented `setup_default_user_and_namespace()` for auto-setup on first run
- Updated `Item` struct in `src/db/item.rs` with new fields
- Updated `insert_item()` and `update_item()` in `src/db/crud.rs`

### Phase 2: Identity Context System ✓
- Created `src/context/mod.rs` and `src/context/identity.rs`
- Implemented `Context` struct with identity resolution:
  - User: `--as` flag → `CTM_USER` env → `USER` env → "default"
  - Namespace: `--ns` flag → `CTM_NAMESPACE` env → "default"
- Added global `--as` and `--ns` flags to `src/args/parser.rs`
- Modified `src/main.rs` to resolve context after DB connect
- Modified `src/actions/handler.rs` to accept `&Context` parameter

### Phase 3: User/Namespace Commands ✓
- Created `src/db/user.rs` with User struct and CRUD operations
- Created `src/db/namespace.rs` with Namespace, NamespaceMembership structs and CRUD
- Created `src/actions/user.rs` with command handlers
- Created `src/actions/namespace.rs` with command handlers
- Added User and Ns subcommands to `src/args/parser.rs`
- Commands implemented:
  - `ctm user create <name> [-d "Display Name"]`
  - `ctm user list`
  - `ctm user delete <name>`
  - `ctm ns create <name> [-d "description"]`
  - `ctm ns list`
  - `ctm ns delete <name>`
  - `ctm ns switch <name>`
  - `ctm ns add-user <ns> <user> [--role admin]`
  - `ctm ns remove-user <ns> <user>`
  - `ctm ns members [namespace]`

### Phase 4: Task Enhancements ✓
- Created `src/args/priority.rs` - Parse high/normal/low to 0/1/2
- Created `src/args/estimate.rs` - Parse 2h/30m/1h30m to minutes
- Updated `src/args/mod.rs` to export new modules
- Added `-P`, `-e`, `--for` flags to TaskCommand in parser.rs
- Added `-u/--user` and `--all-users` flags to ListTaskCommand
- Updated ItemQuery with assignee_id, owner_id, namespace_id filters
- Updated crud.rs query_items to filter by assignee/owner/namespace
- Updated addition.rs to handle priority, estimate, assignee (resolved to user ID)
- Updated list/tasks.rs to resolve and pass user filter to query functions
- Tasks now store owner_id, assignee_id, namespace_id, priority, estimate_minutes

**All 87 tests pass.**

---

## Key Decisions Made

| Decision | Choice |
|----------|--------|
| Database model | Single DB, multi-tenant |
| User model | Single manager tracking team |
| Identity resolution | --as flag → CTM_USER env → system $USER |
| First run | Auto-setup (create user + default namespace) |
| Task ownership | owner_id (accountable) + assignee_id (working on it) |
| Roles | Per-namespace (owner/admin/member/viewer) |
| GitHub integration | Use `gh` CLI wrapper (not HTTP API) |
| Future-proofing | Design for concurrent access later |

---

## Implementation Phases

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Schema v5 Migration | COMPLETE |
| 2 | Identity Context System | COMPLETE |
| 3 | User/Namespace Commands | COMPLETE |
| 4 | Task Enhancements | COMPLETE |
| 5 | Notes/Show/Claim/Link | Not started |
| 6 | Reporting Commands | Not started |
| 7 | GitHub Integration | Not started |
| 8 | /work + /standup | Not started |

---

## Files Created

```
src/context/mod.rs          # Context module export
src/context/identity.rs     # Context struct + resolution logic
src/db/user.rs              # User struct + CRUD operations
src/db/namespace.rs         # Namespace struct + CRUD operations
src/actions/user.rs         # User command handlers
src/actions/namespace.rs    # Namespace command handlers
src/args/priority.rs        # Priority parsing (high/normal/low)
src/args/estimate.rs        # Estimate parsing (2h/30m/1h30m)
```

## Files Modified

```
src/db/conn.rs              # Schema v5 migration + auto-setup
src/db/item.rs              # Item struct with new fields + ItemQuery filters
src/db/crud.rs              # insert_item/update_item + assignee filtering
src/db/mod.rs               # Export user and namespace modules
src/args/parser.rs          # --as, --ns flags + User/Ns + -P/-e/--for/--user
src/args/mod.rs             # Export priority and estimate modules
src/main.rs                 # Context resolution after DB connect
src/actions/handler.rs      # Route User/Ns commands + pass ctx to task
src/actions/mod.rs          # Export user and namespace modules
src/actions/addition.rs     # Handle priority, estimate, assignee on task creation
src/actions/list/tasks.rs   # User filter support for listing tasks
```

---

## Current Schema (v5)

### Items Table
```sql
id, action, category, content, create_time, target_time, modify_time, status,
cron_schedule, human_schedule, recurring_task_id, good_until,
reminder_days, project,
owner_id, assignee_id, namespace_id, priority, estimate_minutes, github_issue
```

### New Tables
```sql
users (id, name, display_name, created_at, created_by)
namespaces (id, name, description, created_at, created_by)
user_namespaces (user_id, namespace_id, role, created_at)
task_links (id, item_id, link_type, reference, title, created_at, created_by)
task_notes (id, item_id, content, created_at, created_by)
audit_log (id, item_id, table_name, action, field_name, old_value, new_value, created_at, created_by)
```

---

## Next Action

Start Phase 5: Notes/Show/Claim/Link
- Create `src/db/note.rs` - TaskNote struct and CRUD
- Create `src/db/link.rs` - TaskLink struct and CRUD
- Create `src/actions/note.rs` - Note command handler (`ctm note <index> "text"`)
- Create `src/actions/show.rs` - Detailed view (`ctm show <index>`)
- Create `src/actions/claim.rs` - Take ownership (`ctm claim <index>`)
- Add Link command for attaching commits/issues/PRs to tasks

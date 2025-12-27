# Checkpoint: Multi-Tenant CTM Implementation

**Date:** 2025-12-26
**Status:** Phases 1-2 COMPLETE, ready for Phase 3

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

**All 68 tests pass.**

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
| 3 | User/Namespace Commands | Not started |
| 4 | Task Enhancements | Not started |
| 5 | Notes/Show/Claim/Link | Not started |
| 6 | Reporting Commands | Not started |
| 7 | GitHub Integration | Not started |
| 8 | /work + /standup | Not started |

---

## Files Created

```
src/context/mod.rs        # Context module export
src/context/identity.rs   # Context struct + resolution logic
```

## Files Modified

```
src/db/conn.rs           # Schema v5 migration + auto-setup
src/db/item.rs           # Item struct with new fields
src/db/crud.rs           # insert_item/update_item with new fields
src/args/parser.rs       # --as and --ns global flags
src/main.rs              # Context resolution after DB connect
src/actions/handler.rs   # Accept &Context parameter
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

Start Phase 3: User/Namespace Commands
- Create `src/db/user.rs` and `src/db/namespace.rs`
- Create `src/actions/user.rs` and `src/actions/namespace.rs`
- Add `User` and `Ns` subcommands to parser
- Route in handler

# Resume Prompt: CTM Multi-Tenant Implementation

Use this prompt to resume work after context compaction.

---

## Context

I'm implementing a major feature set for claude-task-manager (ctm), a Rust CLI task manager. We completed planning and are ready to implement.

**Project location:** `/mnt/c/python/claude-task-manager`

## Quick Summary

Transform ctm from single-user to multi-tenant with:
- Users and namespaces for team tracking
- Task priority (-P), estimates (-e), notes, assignees (--for)
- GitHub integration (issues, PRs, commits)
- Team reporting (dashboard, workload, stats)
- Enhanced /work context for Claude sessions

## Key Files

- **Full plan:** `/home/jim/.claude/plans/modular-fluttering-aurora.md`
- **Checkpoint:** `.claude/checkpoint.md`
- **Schema migrations:** `src/db/conn.rs`
- **CLI commands:** `src/args/parser.rs`
- **Command handlers:** `src/actions/handler.rs`
- **Item model:** `src/db/item.rs`

## Implementation Order

1. **Schema v5 Migration** - New tables (users, namespaces, task_links, task_notes, audit_log) + items columns (owner_id, assignee_id, namespace_id, priority, estimate_minutes, github_issue)
2. **Identity Context System** - --as, --ns global flags, context resolution
3. **User/Namespace Commands** - `ctm user create/list/delete`, `ctm ns create/switch/add-user`
4. **Task Enhancements** - Priority (-P), estimates (-e), assignee (--for)
5. **Notes/Show/Claim/Link** - Append notes, detailed view, claim tasks, link issues/PRs/commits
6. **Reporting** - `ctm team`, `ctm workload`, `ctm stats` with --json/--md output
7. **GitHub Integration** - --from-issue, --close-issue (uses `gh` CLI)
8. **Enhanced /work + /standup** - Rich context for Claude sessions

## To Resume

```
1. Read the full plan: /home/jim/.claude/plans/modular-fluttering-aurora.md
2. Check status: .claude/checkpoint.md
3. Start/continue with the next incomplete phase
```

## Design Decisions (Don't Re-Ask)

- Single DB, multi-tenant (not separate DBs per namespace)
- Single manager model (one person runs ctm, tracks team)
- Roles are per-namespace (owner/admin/member/viewer)
- Task ownership: owner_id (accountable) + assignee_id (working)
- Unassigned tasks allowed, claimable via `ctm claim`
- Identity: --as flag → CTM_USER env → config → system $USER
- Auto-setup on first run (create user + default namespace)
- GitHub: use `gh` CLI wrapper, not HTTP API
- Future-proof for concurrent access (proper IDs, audit trails)
- Reports support --json and --md output flags
- /work passes: task notes, links, last 5 commits, open PRs/issues

## New CLI Commands

```bash
# User management
ctm user create <name>
ctm user list
ctm user delete <name>

# Namespace management
ctm ns create <name>
ctm ns list
ctm ns delete <name>
ctm ns switch <name>
ctm ns add-user <ns> <user> [--role admin]
ctm ns remove-user <ns> <user>

# Task enhancements
ctm task "desc" friday -P high -e 2h --for sarah
ctm note <index> "note text"
ctm show <index>
ctm claim <index>
ctm link <index> --issue owner/repo#42

# Reporting
ctm team [--json] [--md]
ctm workload [--user sarah]
ctm stats

# GitHub
ctm task --from-issue owner/repo#42
ctm done <index> --close-issue
```

## Global Flags

```bash
ctm --as sarah list task    # Act as user
ctm --ns work list task     # Use namespace
```

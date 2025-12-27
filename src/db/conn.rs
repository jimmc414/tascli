use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::get_data_path;

// Going forward, all schema changes require toggling
// this DB_VERSION to a higher number.
const SCHEMA_VERSION: i32 = 5;

pub fn init_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    let current_version: i32 = conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

    if current_version == SCHEMA_VERSION {
        return Ok(());
    }

    // Single polymorphic table
    // Supports task, record, recurring_task, recurring_task_record
    // distinguished via field "action"
    // common fields: id; action; category; content; create_time; modify_time; status;
    // target_time is specific for type task
    // cron_schedule; human_schedule is specific for type recurring_task
    // recurring_task_id; good_until is for type recurring task record
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            action TEXT NOT NULL,
            category TEXT NOT NULL,
            content TEXT NOT NULL,
            create_time INTEGER NOT NULL,
            target_time INTEGER,
            modify_time INTEGER,
            status INTEGER DEFAULT 0,
            cron_schedule TEXT,
            human_schedule TEXT,
            recurring_task_id INTEGER,
            good_until INTEGER,
            reminder_days INTEGER,
            project TEXT,
            owner_id INTEGER REFERENCES users(id),
            assignee_id INTEGER REFERENCES users(id),
            namespace_id INTEGER REFERENCES namespaces(id),
            priority INTEGER DEFAULT 1,
            estimate_minutes INTEGER,
            github_issue TEXT
        )",
        [],
    )?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_action ON items(action)", [])?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_create_time ON items(create_time)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_target_time ON items(target_time)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_category ON items(category)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_category_create_time ON items(category, create_time)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_category_target_time ON items(category, target_time)",
        [],
    )?;

    // Create cache table for list commands
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
            key INTEGER PRIMARY KEY,
            value INTEGER NOT NULL
        )",
        [],
    )?;

    // Users table (team members you track)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            display_name TEXT,
            created_at INTEGER NOT NULL,
            created_by INTEGER REFERENCES users(id)
        )",
        [],
    )?;

    // Namespaces table (work, personal, team-x)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS namespaces (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL,
            created_by INTEGER REFERENCES users(id)
        )",
        [],
    )?;

    // User-namespace membership with roles
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_namespaces (
            user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            namespace_id INTEGER NOT NULL REFERENCES namespaces(id) ON DELETE CASCADE,
            role TEXT NOT NULL CHECK(role IN ('owner', 'admin', 'member', 'viewer')),
            created_at INTEGER NOT NULL,
            PRIMARY KEY (user_id, namespace_id)
        )",
        [],
    )?;

    // Task links (commits, issues, PRs)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            link_type TEXT NOT NULL CHECK(link_type IN ('commit', 'issue', 'pr', 'url')),
            reference TEXT NOT NULL,
            title TEXT,
            created_at INTEGER NOT NULL,
            created_by INTEGER REFERENCES users(id)
        )",
        [],
    )?;

    // Task notes (append-only)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            created_by INTEGER REFERENCES users(id)
        )",
        [],
    )?;

    // Audit log
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER REFERENCES items(id) ON DELETE SET NULL,
            table_name TEXT NOT NULL,
            action TEXT NOT NULL CHECK(action IN ('create', 'update', 'delete', 'complete')),
            field_name TEXT,
            old_value TEXT,
            new_value TEXT,
            created_at INTEGER NOT NULL,
            created_by INTEGER REFERENCES users(id)
        )",
        [],
    )?;

    // Migrate from version 1 to 2 - add columns for recurring task support
    if current_version < 2 && current_version > 0 {
        conn.execute("ALTER TABLE items ADD COLUMN cron_schedule TEXT", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN human_schedule TEXT", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN recurring_task_id INTEGER", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN good_until INTEGER", [])?;
    }

    // Migrate from version 2 to 3 - add reminder_days column for early task reminders
    if current_version < 3 && current_version > 0 {
        conn.execute("ALTER TABLE items ADD COLUMN reminder_days INTEGER", [])?;
    }

    // Migrate from version 3 to 4 - add project column for project association
    if current_version < 4 && current_version > 0 {
        conn.execute("ALTER TABLE items ADD COLUMN project TEXT", [])?;
    }

    // Migrate from version 4 to 5 - add multi-tenant columns
    if current_version < 5 && current_version > 0 {
        conn.execute("ALTER TABLE items ADD COLUMN owner_id INTEGER REFERENCES users(id)", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN assignee_id INTEGER REFERENCES users(id)", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN namespace_id INTEGER REFERENCES namespaces(id)", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN priority INTEGER DEFAULT 1", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN estimate_minutes INTEGER", [])?;
        conn.execute("ALTER TABLE items ADD COLUMN github_issue TEXT", [])?;
    }

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_recurring_task_id_good_until ON items(recurring_task_id, good_until)",
        [],
    )?;

    // Indexes for multi-tenant columns
    conn.execute("CREATE INDEX IF NOT EXISTS idx_owner_id ON items(owner_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_assignee_id ON items(assignee_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_namespace_id ON items(namespace_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_priority ON items(priority)", [])?;

    // Indexes for task_links and task_notes
    conn.execute("CREATE INDEX IF NOT EXISTS idx_task_links_item_id ON task_links(item_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_task_notes_item_id ON task_notes(item_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_audit_log_item_id ON audit_log(item_id)", [])?;

    // Auto-setup default user and namespace on first run or upgrade to v5
    setup_default_user_and_namespace(conn, current_version)?;

    conn.execute(&format!("PRAGMA user_version = {SCHEMA_VERSION}"), [])?;

    Ok(())
}

/// Creates default user (from system $USER) and namespace on first run or v5 upgrade.
/// Migrates existing items to the default user/namespace.
fn setup_default_user_and_namespace(conn: &Connection, from_version: i32) -> Result<(), rusqlite::Error> {
    // Check if users table is empty (first run or fresh v5 install)
    let user_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM users",
        [],
        |row| row.get(0)
    )?;

    if user_count > 0 {
        // Already set up
        return Ok(());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Get username from environment, fall back to "default"
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME")) // Windows fallback
        .unwrap_or_else(|_| "default".to_string());

    // Create default user
    conn.execute(
        "INSERT INTO users (name, display_name, created_at, created_by) VALUES (?1, ?2, ?3, NULL)",
        rusqlite::params![&username, &username, now],
    )?;

    let user_id: i64 = conn.query_row(
        "SELECT id FROM users WHERE name = ?1",
        [&username],
        |row| row.get(0)
    )?;

    // Create default namespace
    conn.execute(
        "INSERT INTO namespaces (name, description, created_at, created_by) VALUES ('default', 'Default namespace', ?1, ?2)",
        rusqlite::params![now, user_id],
    )?;

    let namespace_id: i64 = conn.query_row(
        "SELECT id FROM namespaces WHERE name = 'default'",
        [],
        |row| row.get(0)
    )?;

    // Assign user as owner of default namespace
    conn.execute(
        "INSERT INTO user_namespaces (user_id, namespace_id, role, created_at) VALUES (?1, ?2, 'owner', ?3)",
        rusqlite::params![user_id, namespace_id, now],
    )?;

    // If upgrading from v4 (has existing data), migrate items to default user/namespace
    if from_version >= 1 {
        conn.execute(
            "UPDATE items SET owner_id = ?1, namespace_id = ?2 WHERE owner_id IS NULL",
            rusqlite::params![user_id, namespace_id],
        )?;
    }

    Ok(())
}

pub fn connect() -> Result<Connection, String> {
    let db_path = get_data_path()?;
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    init_table(&conn).map_err(|e| e.to_string())?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use rusqlite::Row;

    use super::*;
    use crate::tests::get_test_conn;

    #[test]
    fn test_init_table() {
        let (conn, _temp_file) = get_test_conn();

        let result = init_table(&conn);
        assert!(
            result.is_ok(),
            "Failed to initialize table: {:?}",
            result.err()
        );

        let item_table_exists = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='items'",
            [],
            |row: &Row| row.get::<_, String>(0),
        );
        assert!(item_table_exists.is_ok(), "Table 'items' does not exist");
        let cache_table_exists = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='cache'",
            [],
            |row: &Row| row.get::<_, String>(0),
        );
        assert!(cache_table_exists.is_ok(), "Table 'cache' does not exist");
        let pragma_version = conn.query_row("PRAGMA user_version", [], |row| row.get::<_, i32>(0));
        assert_eq!(SCHEMA_VERSION, pragma_version.unwrap());

        let second_result = init_table(&conn);
        assert!(
            second_result.is_ok(),
            "Second initialization failed: {:?}",
            second_result.err()
        );
    }

    #[test]
    fn test_init_table_version_logic() {
        let (conn, _temp_file) = get_test_conn();

        // Manaually drop table and verify with schema version the same
        // init_table DOES NOT recreate it.
        conn.execute("DROP TABLE cache", []).unwrap();
        let second_result = init_table(&conn);
        assert!(second_result.is_ok());

        let cache_exists = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='cache'",
            [],
            |row: &Row| row.get::<_, String>(0),
        );
        assert!(
            cache_exists.is_err(),
            "Cache table should NOT exist - proves early return works"
        );

        // Manually set schema version to a lower number,
        // init_table should then run.
        conn.execute("PRAGMA user_version = 0", []).unwrap();
        let third_result = init_table(&conn);
        assert!(third_result.is_ok());

        let cache_exists_after_update = conn.query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='cache'",
            [],
            |row: &Row| row.get::<_, String>(0),
        );
        assert!(
            cache_exists_after_update.is_ok(),
            "Cache table should exist after version reset"
        );

        let final_version = conn
            .query_row("PRAGMA user_version", [], |row| row.get::<_, i32>(0))
            .unwrap();
        assert_eq!(SCHEMA_VERSION, final_version);
    }
}

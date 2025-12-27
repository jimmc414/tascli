use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

/// Link types for task associations
pub const LINK_TYPE_COMMIT: &str = "commit";
pub const LINK_TYPE_ISSUE: &str = "issue";
pub const LINK_TYPE_PR: &str = "pr";
pub const LINK_TYPE_URL: &str = "url";

#[derive(Debug, Clone)]
pub struct TaskLink {
    pub id: i64,
    pub item_id: i64,
    pub link_type: String,
    pub reference: String,
    pub title: Option<String>,
    pub created_at: i64,
    pub created_by: Option<i64>,
}

impl TaskLink {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(TaskLink {
            id: row.get("id")?,
            item_id: row.get("item_id")?,
            link_type: row.get("link_type")?,
            reference: row.get("reference")?,
            title: row.get("title")?,
            created_at: row.get("created_at")?,
            created_by: row.get("created_by")?,
        })
    }

    /// Formats the link for display
    pub fn display(&self) -> String {
        match self.title.as_ref() {
            Some(title) => format!("[{}] {} - {}", self.link_type, self.reference, title),
            None => format!("[{}] {}", self.link_type, self.reference),
        }
    }
}

/// Validates a link type.
pub fn validate_link_type(link_type: &str) -> Result<(), String> {
    match link_type {
        LINK_TYPE_COMMIT | LINK_TYPE_ISSUE | LINK_TYPE_PR | LINK_TYPE_URL => Ok(()),
        _ => Err(format!(
            "Invalid link type '{}'. Valid types: commit, issue, pr, url",
            link_type
        )),
    }
}

/// Adds a link to a task.
pub fn add_link(
    conn: &Connection,
    item_id: i64,
    link_type: &str,
    reference: &str,
    title: Option<&str>,
    created_by: Option<i64>,
) -> Result<i64, String> {
    validate_link_type(link_type)?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    conn.execute(
        "INSERT INTO task_links (item_id, link_type, reference, title, created_at, created_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![item_id, link_type, reference, title, now, created_by],
    )
    .map_err(|e| e.to_string())?;

    let link_id = conn.last_insert_rowid();
    Ok(link_id)
}

/// Gets all links for a task, ordered by creation time (oldest first).
pub fn get_links_for_item(conn: &Connection, item_id: i64) -> Result<Vec<TaskLink>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, link_type, reference, title, created_at, created_by
             FROM task_links
             WHERE item_id = ?1
             ORDER BY created_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let links = stmt
        .query_map([item_id], TaskLink::from_row)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(links)
}

/// Gets links for a task filtered by type.
pub fn get_links_by_type(
    conn: &Connection,
    item_id: i64,
    link_type: &str,
) -> Result<Vec<TaskLink>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, link_type, reference, title, created_at, created_by
             FROM task_links
             WHERE item_id = ?1 AND link_type = ?2
             ORDER BY created_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let links = stmt
        .query_map([item_id.to_string(), link_type.to_string()], TaskLink::from_row)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(links)
}

/// Deletes a specific link by ID.
pub fn delete_link(conn: &Connection, link_id: i64) -> Result<(), String> {
    let deleted = conn
        .execute("DELETE FROM task_links WHERE id = ?1", [link_id])
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err(format!("Link {} not found", link_id));
    }

    Ok(())
}

/// Gets the count of links for a task.
pub fn count_links_for_item(conn: &Connection, item_id: i64) -> Result<i64, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM task_links WHERE item_id = ?1",
        [item_id],
        |row| row.get(0),
    )
    .map_err(|e| e.to_string())
}

/// Checks if a link with the same reference already exists for a task.
pub fn link_exists(conn: &Connection, item_id: i64, reference: &str) -> Result<bool, String> {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM task_links WHERE item_id = ?1 AND reference = ?2",
            rusqlite::params![item_id, reference],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(count > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{get_test_conn, insert_task};

    #[test]
    fn test_add_and_get_links() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Add commit link
        let link1_id = add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();
        assert!(link1_id > 0);

        // Add issue link with title
        let link2_id = add_link(
            &conn,
            task_id,
            "issue",
            "owner/repo#42",
            Some("Fix login bug"),
            None,
        )
        .unwrap();
        assert!(link2_id > link1_id);

        // Get all links
        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].link_type, "commit");
        assert_eq!(links[0].reference, "abc123");
        assert_eq!(links[0].title, None);
        assert_eq!(links[1].link_type, "issue");
        assert_eq!(links[1].reference, "owner/repo#42");
        assert_eq!(links[1].title, Some("Fix login bug".to_string()));
    }

    #[test]
    fn test_add_link_invalid_type() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let result = add_link(&conn, task_id, "invalid", "ref", None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid link type"));
    }

    #[test]
    fn test_get_links_by_type() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();
        add_link(&conn, task_id, "commit", "def456", None, None).unwrap();
        add_link(&conn, task_id, "issue", "owner/repo#1", None, None).unwrap();
        add_link(&conn, task_id, "pr", "owner/repo#2", None, None).unwrap();

        let commits = get_links_by_type(&conn, task_id, "commit").unwrap();
        assert_eq!(commits.len(), 2);

        let issues = get_links_by_type(&conn, task_id, "issue").unwrap();
        assert_eq!(issues.len(), 1);

        let prs = get_links_by_type(&conn, task_id, "pr").unwrap();
        assert_eq!(prs.len(), 1);
    }

    #[test]
    fn test_delete_link() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let link_id = add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();
        let count = count_links_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 1);

        delete_link(&conn, link_id).unwrap();
        let count = count_links_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_link_exists() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        assert!(!link_exists(&conn, task_id, "abc123").unwrap());

        add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();

        assert!(link_exists(&conn, task_id, "abc123").unwrap());
        assert!(!link_exists(&conn, task_id, "def456").unwrap());
    }

    #[test]
    fn test_link_display() {
        let link_without_title = TaskLink {
            id: 1,
            item_id: 1,
            link_type: "commit".to_string(),
            reference: "abc123".to_string(),
            title: None,
            created_at: 0,
            created_by: None,
        };
        assert_eq!(link_without_title.display(), "[commit] abc123");

        let link_with_title = TaskLink {
            id: 2,
            item_id: 1,
            link_type: "issue".to_string(),
            reference: "owner/repo#42".to_string(),
            title: Some("Fix bug".to_string()),
            created_at: 0,
            created_by: None,
        };
        assert_eq!(link_with_title.display(), "[issue] owner/repo#42 - Fix bug");
    }

    #[test]
    fn test_links_isolated_by_task() {
        let (conn, _temp_file) = get_test_conn();
        let task1_id = insert_task(&conn, "work", "Task 1", "today");
        let task2_id = insert_task(&conn, "work", "Task 2", "today");

        add_link(&conn, task1_id, "commit", "abc123", None, None).unwrap();
        add_link(&conn, task2_id, "commit", "def456", None, None).unwrap();
        add_link(&conn, task2_id, "issue", "owner/repo#1", None, None).unwrap();

        let links1 = get_links_for_item(&conn, task1_id).unwrap();
        assert_eq!(links1.len(), 1);
        assert_eq!(links1[0].reference, "abc123");

        let links2 = get_links_for_item(&conn, task2_id).unwrap();
        assert_eq!(links2.len(), 2);
    }

    #[test]
    fn test_all_link_types() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Test all valid link types
        add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();
        add_link(&conn, task_id, "issue", "owner/repo#1", None, None).unwrap();
        add_link(&conn, task_id, "pr", "owner/repo#2", None, None).unwrap();
        add_link(&conn, task_id, "url", "https://example.com", None, None).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 4);
    }
}

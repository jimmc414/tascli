use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TaskNote {
    pub id: i64,
    pub item_id: i64,
    pub content: String,
    pub created_at: i64,
    pub created_by: Option<i64>,
}

impl TaskNote {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(TaskNote {
            id: row.get("id")?,
            item_id: row.get("item_id")?,
            content: row.get("content")?,
            created_at: row.get("created_at")?,
            created_by: row.get("created_by")?,
        })
    }
}

/// Adds a note to a task.
pub fn add_note(
    conn: &Connection,
    item_id: i64,
    content: &str,
    created_by: Option<i64>,
) -> Result<i64, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    conn.execute(
        "INSERT INTO task_notes (item_id, content, created_at, created_by) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![item_id, content, now, created_by],
    )
    .map_err(|e| e.to_string())?;

    let note_id = conn.last_insert_rowid();
    Ok(note_id)
}

/// Gets all notes for a task, ordered by creation time (oldest first).
pub fn get_notes_for_item(conn: &Connection, item_id: i64) -> Result<Vec<TaskNote>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, item_id, content, created_at, created_by
             FROM task_notes
             WHERE item_id = ?1
             ORDER BY created_at ASC",
        )
        .map_err(|e| e.to_string())?;

    let notes = stmt
        .query_map([item_id], TaskNote::from_row)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(notes)
}

/// Deletes a specific note by ID.
pub fn delete_note(conn: &Connection, note_id: i64) -> Result<(), String> {
    let deleted = conn
        .execute("DELETE FROM task_notes WHERE id = ?1", [note_id])
        .map_err(|e| e.to_string())?;

    if deleted == 0 {
        return Err(format!("Note {} not found", note_id));
    }

    Ok(())
}

/// Gets the count of notes for a task.
pub fn count_notes_for_item(conn: &Connection, item_id: i64) -> Result<i64, String> {
    conn.query_row(
        "SELECT COUNT(*) FROM task_notes WHERE item_id = ?1",
        [item_id],
        |row| row.get(0),
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{get_test_conn, insert_task};

    #[test]
    fn test_add_and_get_notes() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Add first note
        let note1_id = add_note(&conn, task_id, "First note", None).unwrap();
        assert!(note1_id > 0);

        // Add second note
        let note2_id = add_note(&conn, task_id, "Second note", None).unwrap();
        assert!(note2_id > note1_id);

        // Get notes
        let notes = get_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].content, "First note");
        assert_eq!(notes[1].content, "Second note");
    }

    #[test]
    fn test_notes_with_created_by() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Add note with created_by
        let note_id = add_note(&conn, task_id, "User note", Some(1)).unwrap();
        assert!(note_id > 0);

        let notes = get_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].created_by, Some(1));
    }

    #[test]
    fn test_delete_note() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let note_id = add_note(&conn, task_id, "To be deleted", None).unwrap();
        let count = count_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 1);

        delete_note(&conn, note_id).unwrap();
        let count = count_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_delete_nonexistent_note() {
        let (conn, _temp_file) = get_test_conn();
        let result = delete_note(&conn, 99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_count_notes() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let count = count_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 0);

        add_note(&conn, task_id, "Note 1", None).unwrap();
        add_note(&conn, task_id, "Note 2", None).unwrap();
        add_note(&conn, task_id, "Note 3", None).unwrap();

        let count = count_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_notes_isolated_by_task() {
        let (conn, _temp_file) = get_test_conn();
        let task1_id = insert_task(&conn, "work", "Task 1", "today");
        let task2_id = insert_task(&conn, "work", "Task 2", "today");

        add_note(&conn, task1_id, "Note for task 1", None).unwrap();
        add_note(&conn, task2_id, "Note for task 2", None).unwrap();
        add_note(&conn, task2_id, "Another note for task 2", None).unwrap();

        let notes1 = get_notes_for_item(&conn, task1_id).unwrap();
        assert_eq!(notes1.len(), 1);
        assert_eq!(notes1[0].content, "Note for task 1");

        let notes2 = get_notes_for_item(&conn, task2_id).unwrap();
        assert_eq!(notes2.len(), 2);
    }
}

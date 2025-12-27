use rusqlite::Connection;

use crate::{
    actions::display,
    args::parser::NoteCommand,
    context::Context,
    db::{
        cache,
        crud::get_item,
        item::{RECORD, RECURRING_TASK_RECORD},
        note::add_note,
    },
};

pub fn handle_notecmd(conn: &Connection, ctx: &Context, cmd: &NoteCommand) -> Result<(), String> {
    validate_cache(conn)?;
    let row_id = get_rowid_from_cache(conn, cmd.index)?;

    let item = get_item(conn, row_id).map_err(|e| format!("Failed to get item: {:?}", e))?;
    if item.action == RECORD || item.action == RECURRING_TASK_RECORD {
        return Err("Cannot add notes to records".to_string());
    }

    let note_id = add_note(conn, row_id, &cmd.content, Some(ctx.current_user_id))?;

    display::print_bold(&format!("Added note #{} to task:", note_id));
    display::print_items(&[item], false, false);
    println!("  Note: {}", cmd.content);

    Ok(())
}

fn validate_cache(conn: &Connection) -> Result<(), String> {
    match cache::validate_cache(conn) {
        Ok(true) => Ok(()),
        Ok(false) => Err("Cache is not valid, consider running list command first".to_string()),
        Err(_) => Err("Cannot connect to cache".to_string()),
    }
}

fn get_rowid_from_cache(conn: &Connection, index: usize) -> Result<i64, String> {
    let index = index as i64;
    match cache::read(conn, index).map_err(|e| format!("Failed to read cache table: {:?}", e))? {
        Some(id) => Ok(id),
        None => Err(format!("index {} does not exist", index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Context,
        db::{
            cache,
            crud::query_items,
            item::{ItemQuery, TASK},
            note::get_notes_for_item,
        },
        tests::{get_test_conn, insert_task},
    };

    #[test]
    fn test_handle_notecmd() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let note_cmd = NoteCommand {
            index: 1,
            content: "This is a test note".to_string(),
        };

        handle_notecmd(&conn, &ctx, &note_cmd).unwrap();

        let notes = get_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].content, "This is a test note");
        assert_eq!(notes[0].created_by, Some(ctx.current_user_id));
    }

    #[test]
    fn test_handle_notecmd_multiple() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        // Add first note
        let note_cmd1 = NoteCommand {
            index: 1,
            content: "First note".to_string(),
        };
        handle_notecmd(&conn, &ctx, &note_cmd1).unwrap();

        // Add second note
        let note_cmd2 = NoteCommand {
            index: 1,
            content: "Second note".to_string(),
        };
        handle_notecmd(&conn, &ctx, &note_cmd2).unwrap();

        let notes = get_notes_for_item(&conn, task_id).unwrap();
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].content, "First note");
        assert_eq!(notes[1].content, "Second note");
    }

    #[test]
    fn test_handle_notecmd_on_record_fails() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();

        // Insert a record
        use crate::tests::insert_record;
        insert_record(&conn, "work", "Test record", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(RECORD)).unwrap();
        cache::store(&conn, &items).unwrap();

        let note_cmd = NoteCommand {
            index: 1,
            content: "This should fail".to_string(),
        };

        let result = handle_notecmd(&conn, &ctx, &note_cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot add notes to records"));
    }

    #[test]
    fn test_handle_notecmd_invalid_index() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let note_cmd = NoteCommand {
            index: 99,
            content: "This should fail".to_string(),
        };

        let result = handle_notecmd(&conn, &ctx, &note_cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }
}

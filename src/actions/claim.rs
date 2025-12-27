use rusqlite::Connection;

use crate::{
    actions::display,
    context::Context,
    db::{
        cache,
        crud::{get_item, update_item},
        item::{RECORD, RECURRING_TASK_RECORD},
    },
};

/// Handles the claim command - takes ownership of an unassigned task
pub fn handle_claimcmd(conn: &Connection, ctx: &Context, index: usize) -> Result<(), String> {
    validate_cache(conn)?;
    let row_id = get_rowid_from_cache(conn, index)?;

    let mut item = get_item(conn, row_id).map_err(|e| format!("Failed to get item: {:?}", e))?;

    if item.action == RECORD || item.action == RECURRING_TASK_RECORD {
        return Err("Cannot claim a record".to_string());
    }

    // Check if already assigned
    if let Some(assignee_id) = item.assignee_id {
        if assignee_id == ctx.current_user_id {
            return Err("You are already assigned to this task".to_string());
        }
        return Err(format!(
            "Task is already assigned. Use update command to reassign."
        ));
    }

    // Claim the task
    item.assignee_id = Some(ctx.current_user_id);
    update_item(conn, &item).map_err(|e| format!("Failed to update item: {:?}", e))?;

    display::print_bold(&format!("Claimed task (assigned to {}):", ctx.current_user_name));
    display::print_items(&[item], false, false);

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
            crud::{get_item, query_items},
            item::{ItemQuery, TASK},
            user::create_user,
        },
        tests::{get_test_conn, insert_task},
    };

    #[test]
    fn test_handle_claimcmd() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Unassigned task", "today");

        // Verify task is unassigned
        let item = get_item(&conn, task_id).unwrap();
        assert!(item.assignee_id.is_none());

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        // Claim the task
        handle_claimcmd(&conn, &ctx, 1).unwrap();

        // Verify task is now assigned
        let item = get_item(&conn, task_id).unwrap();
        assert_eq!(item.assignee_id, Some(ctx.current_user_id));
    }

    #[test]
    fn test_handle_claimcmd_already_assigned_to_self() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Manually assign to self
        let mut item = get_item(&conn, task_id).unwrap();
        item.assignee_id = Some(ctx.current_user_id);
        update_item(&conn, &item).unwrap();

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_claimcmd(&conn, &ctx, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already assigned to this task"));
    }

    #[test]
    fn test_handle_claimcmd_assigned_to_other() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        // Create another user and assign to them
        let other_user_id = create_user(&conn, "other", None, None).unwrap();
        let mut item = get_item(&conn, task_id).unwrap();
        item.assignee_id = Some(other_user_id);
        update_item(&conn, &item).unwrap();

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_claimcmd(&conn, &ctx, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already assigned"));
    }

    #[test]
    fn test_handle_claimcmd_on_record_fails() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();

        use crate::tests::insert_record;
        insert_record(&conn, "work", "Test record", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(RECORD)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_claimcmd(&conn, &ctx, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot claim a record"));
    }

    #[test]
    fn test_handle_claimcmd_invalid_index() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_claimcmd(&conn, &ctx, 99);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }
}

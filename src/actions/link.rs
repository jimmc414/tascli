use rusqlite::Connection;

use crate::{
    actions::display,
    args::parser::LinkCommand,
    context::Context,
    db::{
        cache,
        crud::get_item,
        item::{RECORD, RECURRING_TASK_RECORD},
        link::{add_link, link_exists},
    },
};

/// Handles the link command - attaches a commit, issue, PR, or URL to a task
pub fn handle_linkcmd(conn: &Connection, ctx: &Context, cmd: &LinkCommand) -> Result<(), String> {
    validate_cache(conn)?;
    let row_id = get_rowid_from_cache(conn, cmd.index)?;

    let item = get_item(conn, row_id).map_err(|e| format!("Failed to get item: {:?}", e))?;

    if item.action == RECORD || item.action == RECURRING_TASK_RECORD {
        return Err("Cannot add links to records".to_string());
    }

    // Determine link type and reference from command flags
    let (link_type, reference) = if let Some(ref commit) = cmd.commit {
        ("commit", commit.as_str())
    } else if let Some(ref issue) = cmd.issue {
        ("issue", issue.as_str())
    } else if let Some(ref pr) = cmd.pr {
        ("pr", pr.as_str())
    } else if let Some(ref url) = cmd.url {
        ("url", url.as_str())
    } else {
        return Err("Must specify one of: --commit, --issue, --pr, or --url".to_string());
    };

    // Check if link already exists
    if link_exists(conn, row_id, reference)? {
        return Err(format!("Link '{}' already exists for this task", reference));
    }

    let link_id = add_link(
        conn,
        row_id,
        link_type,
        reference,
        cmd.title.as_deref(),
        Some(ctx.current_user_id),
    )?;

    display::print_bold(&format!("Added {} link #{} to task:", link_type, link_id));
    display::print_items(&[item], false, false);
    match &cmd.title {
        Some(title) => println!("  Link: [{}] {} - {}", link_type, reference, title),
        None => println!("  Link: [{}] {}", link_type, reference),
    }

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
            link::get_links_for_item,
        },
        tests::{get_test_conn, insert_task},
    };

    fn make_link_cmd(index: usize) -> LinkCommand {
        LinkCommand {
            index,
            commit: None,
            issue: None,
            pr: None,
            url: None,
            title: None,
        }
    }

    #[test]
    fn test_handle_linkcmd_commit() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.commit = Some("abc123".to_string());

        handle_linkcmd(&conn, &ctx, &cmd).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link_type, "commit");
        assert_eq!(links[0].reference, "abc123");
        assert_eq!(links[0].created_by, Some(ctx.current_user_id));
    }

    #[test]
    fn test_handle_linkcmd_issue_with_title() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.issue = Some("owner/repo#42".to_string());
        cmd.title = Some("Fix login bug".to_string());

        handle_linkcmd(&conn, &ctx, &cmd).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link_type, "issue");
        assert_eq!(links[0].reference, "owner/repo#42");
        assert_eq!(links[0].title, Some("Fix login bug".to_string()));
    }

    #[test]
    fn test_handle_linkcmd_pr() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.pr = Some("owner/repo#43".to_string());

        handle_linkcmd(&conn, &ctx, &cmd).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link_type, "pr");
        assert_eq!(links[0].reference, "owner/repo#43");
    }

    #[test]
    fn test_handle_linkcmd_url() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.url = Some("https://example.com/docs".to_string());
        cmd.title = Some("Documentation".to_string());

        handle_linkcmd(&conn, &ctx, &cmd).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link_type, "url");
        assert_eq!(links[0].reference, "https://example.com/docs");
        assert_eq!(links[0].title, Some("Documentation".to_string()));
    }

    #[test]
    fn test_handle_linkcmd_no_type_specified() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let cmd = make_link_cmd(1);

        let result = handle_linkcmd(&conn, &ctx, &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Must specify one of"));
    }

    #[test]
    fn test_handle_linkcmd_duplicate_link() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.commit = Some("abc123".to_string());

        // First link succeeds
        handle_linkcmd(&conn, &ctx, &cmd).unwrap();

        // Second link with same reference fails
        let result = handle_linkcmd(&conn, &ctx, &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_handle_linkcmd_on_record_fails() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();

        use crate::tests::insert_record;
        insert_record(&conn, "work", "Test record", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(RECORD)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(1);
        cmd.commit = Some("abc123".to_string());

        let result = handle_linkcmd(&conn, &ctx, &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Cannot add links to records"));
    }

    #[test]
    fn test_handle_linkcmd_invalid_index() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let mut cmd = make_link_cmd(99);
        cmd.commit = Some("abc123".to_string());

        let result = handle_linkcmd(&conn, &ctx, &cmd);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_handle_linkcmd_multiple_links() {
        let (conn, _temp_file) = get_test_conn();
        let ctx = Context::default_from_db(&conn).unwrap();
        let task_id = insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        // Add commit
        let mut cmd1 = make_link_cmd(1);
        cmd1.commit = Some("abc123".to_string());
        handle_linkcmd(&conn, &ctx, &cmd1).unwrap();

        // Add issue
        let mut cmd2 = make_link_cmd(1);
        cmd2.issue = Some("owner/repo#42".to_string());
        handle_linkcmd(&conn, &ctx, &cmd2).unwrap();

        // Add PR
        let mut cmd3 = make_link_cmd(1);
        cmd3.pr = Some("owner/repo#43".to_string());
        handle_linkcmd(&conn, &ctx, &cmd3).unwrap();

        let links = get_links_for_item(&conn, task_id).unwrap();
        assert_eq!(links.len(), 3);
    }
}

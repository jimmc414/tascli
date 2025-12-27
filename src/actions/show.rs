use chrono::{Local, TimeZone};
use rusqlite::Connection;

use crate::{
    args::estimate::format_estimate,
    db::{
        cache,
        crud::get_item,
        item::{Item, RECORD, RECURRING_TASK, RECURRING_TASK_RECORD, TASK},
        link::get_links_for_item,
        note::get_notes_for_item,
        user::get_user_by_id,
    },
};

/// Handles the show command - displays detailed view of a task
pub fn handle_showcmd(conn: &Connection, index: usize) -> Result<(), String> {
    validate_cache(conn)?;
    let row_id = get_rowid_from_cache(conn, index)?;

    let item = get_item(conn, row_id).map_err(|e| format!("Failed to get item: {:?}", e))?;

    print_detailed_view(conn, &item, index)?;

    Ok(())
}

fn print_detailed_view(conn: &Connection, item: &Item, index: usize) -> Result<(), String> {
    let action_label = match item.action.as_str() {
        TASK => "Task",
        RECURRING_TASK => "Recurring Task",
        RECORD => "Record",
        RECURRING_TASK_RECORD => "Recurring Task Record",
        other => other,
    };

    // Header
    println!();
    println!("\x1b[1m{} #{}: {}\x1b[0m", action_label, index, item.content.lines().next().unwrap_or(&item.content));
    println!("{}", "━".repeat(50));

    // Basic fields
    println!("  \x1b[90mPriority:\x1b[0m   {}", format_priority_colored(item.priority));
    println!("  \x1b[90mStatus:\x1b[0m     {}", format_status(item.status));
    println!("  \x1b[90mCategory:\x1b[0m   {}", item.category);

    // Owner
    if let Some(owner_id) = item.owner_id {
        if let Ok(Some(user)) = get_user_by_id(conn, owner_id) {
            let display = user.display_name.as_ref().unwrap_or(&user.name);
            println!("  \x1b[90mOwner:\x1b[0m      {}", display);
        }
    }

    // Assignee
    if let Some(assignee_id) = item.assignee_id {
        if let Ok(Some(user)) = get_user_by_id(conn, assignee_id) {
            let display = user.display_name.as_ref().unwrap_or(&user.name);
            println!("  \x1b[90mAssignee:\x1b[0m   {}", display);
        }
    } else {
        println!("  \x1b[90mAssignee:\x1b[0m   \x1b[33munassigned\x1b[0m");
    }

    // Project
    if let Some(ref project) = item.project {
        println!("  \x1b[90mProject:\x1b[0m    {}", project);
    }

    // Due date / Schedule
    if item.action == RECURRING_TASK {
        if let Some(ref schedule) = item.human_schedule {
            println!("  \x1b[90mSchedule:\x1b[0m   {}", schedule);
        }
    } else if let Some(target_time) = item.target_time {
        println!("  \x1b[90mDue:\x1b[0m        {}", format_timestamp_relative(target_time));
    }

    // Estimate
    let estimate_str = format_estimate(item.estimate_minutes);
    if estimate_str != "-" {
        println!("  \x1b[90mEstimate:\x1b[0m   {}", estimate_str);
    }

    // Reminder
    if let Some(reminder) = item.reminder_days {
        println!("  \x1b[90mReminder:\x1b[0m   {} days before", reminder);
    }

    // Created / Modified
    println!("  \x1b[90mCreated:\x1b[0m    {}", format_timestamp(item.create_time));
    if let Some(modify_time) = item.modify_time {
        if modify_time != item.create_time {
            println!("  \x1b[90mModified:\x1b[0m   {}", format_timestamp(modify_time));
        }
    }

    // Full content if multiline
    let lines: Vec<&str> = item.content.lines().collect();
    if lines.len() > 1 {
        println!();
        println!("\x1b[90mContent:\x1b[0m");
        for line in lines {
            println!("  {}", line);
        }
    }

    // Notes
    let notes = get_notes_for_item(conn, item.id.unwrap())?;
    if !notes.is_empty() {
        println!();
        println!("\x1b[90mNotes:\x1b[0m");
        for note in &notes {
            let timestamp = format_timestamp_short(note.created_at);
            let author = if let Some(created_by) = note.created_by {
                if let Ok(Some(user)) = get_user_by_id(conn, created_by) {
                    format!(" ({})", user.name)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            println!("  \x1b[90m[{}{}]\x1b[0m {}", timestamp, author, note.content);
        }
    }

    // Links
    let links = get_links_for_item(conn, item.id.unwrap())?;
    if !links.is_empty() {
        println!();
        println!("\x1b[90mLinks:\x1b[0m");
        for link in &links {
            let type_colored = match link.link_type.as_str() {
                "commit" => "\x1b[33mcommit\x1b[0m",
                "issue" => "\x1b[32missue\x1b[0m",
                "pr" => "\x1b[35mpr\x1b[0m",
                "url" => "\x1b[36murl\x1b[0m",
                _ => &link.link_type,
            };
            match &link.title {
                Some(title) => println!("  [{}] {} - {}", type_colored, link.reference, title),
                None => println!("  [{}] {}", type_colored, link.reference),
            }
        }
    }

    println!();
    Ok(())
}

fn format_priority_colored(priority: Option<u8>) -> String {
    match priority {
        Some(0) => "\x1b[91mHIGH\x1b[0m".to_string(),
        Some(1) => "normal".to_string(),
        Some(2) => "\x1b[90mlow\x1b[0m".to_string(),
        _ => "-".to_string(),
    }
}

fn format_status(status: u8) -> &'static str {
    match status {
        0 => "ongoing",
        1 => "done",
        2 => "cancelled",
        3 => "duplicate",
        4 => "suspended",
        5 => "removed",
        6 => "pending",
        _ => "unknown",
    }
}

fn format_timestamp(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        _ => "unknown".to_string(),
    }
}

fn format_timestamp_short(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%b %d, %H:%M").to_string(),
        _ => "unknown".to_string(),
    }
}

fn format_timestamp_relative(timestamp: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let diff = timestamp - now;
    let days = diff / 86400;

    let date_str = match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d").to_string(),
        _ => "unknown".to_string(),
    };

    if diff < 0 {
        let past_days = (-diff) / 86400;
        if past_days == 0 {
            format!("{} (\x1b[91moverdue today\x1b[0m)", date_str)
        } else if past_days == 1 {
            format!("{} (\x1b[91m1 day overdue\x1b[0m)", date_str)
        } else {
            format!("{} (\x1b[91m{} days overdue\x1b[0m)", date_str, past_days)
        }
    } else if days == 0 {
        format!("{} (\x1b[93mtoday\x1b[0m)", date_str)
    } else if days == 1 {
        format!("{} (tomorrow)", date_str)
    } else if days <= 7 {
        format!("{} ({} days)", date_str, days)
    } else {
        date_str
    }
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
        db::{
            cache,
            crud::query_items,
            item::ItemQuery,
            link::add_link,
            note::add_note,
        },
        tests::{get_test_conn, insert_task},
    };

    #[test]
    fn test_handle_showcmd() {
        let (conn, _temp_file) = get_test_conn();
        insert_task(&conn, "work", "Test task for show", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        // Should succeed
        let result = handle_showcmd(&conn, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_showcmd_with_notes_and_links() {
        let (conn, _temp_file) = get_test_conn();
        let task_id = insert_task(&conn, "work", "Complex task", "tomorrow");

        // Add notes
        add_note(&conn, task_id, "First note", None).unwrap();
        add_note(&conn, task_id, "Second note", None).unwrap();

        // Add links
        add_link(&conn, task_id, "commit", "abc123", None, None).unwrap();
        add_link(&conn, task_id, "issue", "owner/repo#42", Some("Fix bug"), None).unwrap();

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_showcmd(&conn, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_showcmd_invalid_index() {
        let (conn, _temp_file) = get_test_conn();
        insert_task(&conn, "work", "Test task", "today");

        let items = query_items(&conn, &ItemQuery::new().with_action(TASK)).unwrap();
        cache::store(&conn, &items).unwrap();

        let result = handle_showcmd(&conn, 99);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_format_status() {
        assert_eq!(format_status(0), "ongoing");
        assert_eq!(format_status(1), "done");
        assert_eq!(format_status(2), "cancelled");
        assert_eq!(format_status(3), "duplicate");
        assert_eq!(format_status(4), "suspended");
        assert_eq!(format_status(5), "removed");
        assert_eq!(format_status(6), "pending");
        assert_eq!(format_status(99), "unknown");
    }

    #[test]
    fn test_format_priority_colored() {
        assert!(format_priority_colored(Some(0)).contains("HIGH"));
        assert_eq!(format_priority_colored(Some(1)), "normal");
        assert!(format_priority_colored(Some(2)).contains("low"));
        assert_eq!(format_priority_colored(None), "-");
    }
}

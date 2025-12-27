use clap::{
    Args,
    Parser,
    Subcommand,
};
use crate::args::{
    estimate::parse_estimate,
    priority::parse_priority,
    timestr::{parse_flexible_timestr, parse_recurring_timestr},
};

/// Claude-first task management CLI with multi-tenant support.
///
/// Data is stored at ~/.local/share/ctm/ctm.db,
/// or where defined in config at ~/.config/ctm/config.json
#[derive(Debug, Parser)]
#[command(author, version)]
pub struct CliArgs {
    /// Act as a specific user (overrides CTM_USER env and system $USER)
    #[arg(long = "as", global = true)]
    pub as_user: Option<String>,

    /// Use a specific namespace (overrides CTM_NAMESPACE env, defaults to "default")
    #[arg(long = "ns", global = true)]
    pub namespace: Option<String>,

    #[command(subcommand)]
    pub arguments: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// add task
    Task(TaskCommand),
    /// add record
    Record(RecordCommand),
    /// complete task and generates a corresponding record entry
    Done(DoneCommand),
    /// update task and record entries
    Update(UpdateCommand),
    /// delete task or record
    Delete(DeleteCommand),
    /// list tasks or records
    #[command(subcommand)]
    List(ListCommand),
    /// add a note to a task
    Note(NoteCommand),
    /// show detailed view of a task
    Show(ShowCommand),
    /// claim an unassigned task
    Claim(ClaimCommand),
    /// attach a link (commit, issue, PR, URL) to a task
    Link(LinkCommand),
    /// manage users
    #[command(subcommand)]
    User(UserCommand),
    /// manage namespaces
    #[command(subcommand)]
    Ns(NamespaceCommand),
    /// show team task distribution
    Team(TeamCommand),
    /// show workload by user
    Workload(WorkloadCommand),
    /// show task statistics
    Stats(StatsCommand),
}

#[derive(Debug, Args)]
pub struct TaskCommand {
    /// description of the task
    #[arg(value_parser = |s: &str| syntax_helper("task", s))]
    pub content: String,
    /// time the task is due for completion, default to EOD,
    /// If it is a schedule, then a recurring task would be created.
    #[arg(value_parser = validate_timestr)]
    pub timestr: Option<String>,
    /// category of the task
    #[arg(short, long)]
    pub category: Option<String>,
    /// reminder days before due date to show task in today view,
    /// defaults to 7 days when specified without value
    #[arg(short = 'r', long, default_missing_value = "7", num_args = 0..=1)]
    pub reminder: Option<i64>,
    /// project name (must be defined in ~/.config/ctm/config.json)
    #[arg(short = 'p', long)]
    pub project: Option<String>,
    /// priority: high, normal (default), low (or h/n/l)
    #[arg(short = 'P', long, value_parser = parse_priority)]
    pub priority: Option<u8>,
    /// time estimate: 30m, 2h, 1h30m, 1.5h
    #[arg(short = 'e', long, value_parser = parse_estimate)]
    pub estimate: Option<i64>,
    /// assign task to a user (username)
    #[arg(long = "for")]
    pub assignee: Option<String>,
    /// create task from GitHub issue (e.g., owner/repo#42)
    #[arg(long)]
    pub from_issue: Option<String>,
}

#[derive(Debug, Args)]
pub struct RecordCommand {
    /// content of the record
    #[arg(value_parser = |s: &str| syntax_helper("record", s))]
    pub content: String,
    /// category of the record
    #[arg(short, long)]
    pub category: Option<String>,
    /// time the record is made,
    /// default to current time
    #[arg(short = 't', long = "time", value_parser = validate_timestr)]
    pub timestr: Option<String>,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
    /// optional status, default to done.
    #[arg(short, long, value_parser = parse_status, default_value_t = 1)]
    pub status: u8,
    /// add comment to task content and completion record
    #[arg(short, long)]
    pub comment: Option<String>,
    /// close linked GitHub issue when completing task
    #[arg(long)]
    pub close_issue: bool,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
    /// update the target time of task,
    /// or event time of record,
    /// or schedule of a recurring task
    #[arg(short, long, value_parser = validate_timestr)]
    pub target_time: Option<String>,
    /// update category of the task/record
    #[arg(short, long)]
    pub category: Option<String>,
    /// replace the content of the task/record
    #[arg(short='w', long)]
    pub content: Option<String>,
    /// add to entry content in a newline
    #[arg(short, long)]
    pub add_content: Option<String>,
    /// update status of the tasks,
    /// accept ongoing|done|cancelled|duplicate|suspended|pending
    #[arg(short, long, value_parser = parse_status)]
    pub status: Option<u8>,
    /// set reminder days before due date to show in today view
    #[arg(short = 'r', long)]
    pub reminder: Option<i64>,
    /// update project association (must be defined in config)
    #[arg(short = 'p', long)]
    pub project: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum ListCommand {
    /// list tasks
    Task(ListTaskCommand),
    /// list records
    Record(ListRecordCommand),
    /// show specific listed item content directly for ease to copy
    Show(ShowContentCommand),
}

#[derive(Debug, Args)]
pub struct ListTaskCommand {
    /// task due time. e.g. today,
    /// when present it restrict the task listed to be those,
    /// that are marked for completion prior to this time
    pub timestr: Option<String>,
    /// category of the task
    #[arg(short, long)]
    pub category: Option<String>,
    /// days in the future for tasks to list - mutually exclusive with timestr
    #[arg(short, long, conflicts_with = "timestr")]
    pub days: Option<usize>,
    /// status to list, default to "open",
    /// you can filter individually to ongoing|done|cancelled|duplicate|suspended|pending,
    /// or aggregate status like open|closed|all
    #[arg(short, long, value_parser = parse_status, default_value_t = 254)]
    pub status: u8,
    /// hhow overdue tasks - tasks that are scheduled to be completed in the past,
    /// but were not closed, these tasks are not returned by default
    #[arg(short, long, default_value_t = false)]
    pub overdue: bool,
    /// limit the amount of tasks returned
    #[arg(short, long, default_value_t = 100, value_parser = validate_limit)]
    pub limit: usize,
    /// next page if the previous list command reached limit
    #[arg(short, long, default_value_t = false)]
    pub next_page: bool,
    /// search for tasks containing this text in their content
    #[arg(long)]
    pub search: Option<String>,
    /// filter tasks by assignee username
    #[arg(short, long)]
    pub user: Option<String>,
    /// show tasks for all users (ignores current user filter)
    #[arg(long, default_value_t = false)]
    pub all_users: bool,
}

#[derive(Debug, Args)]
pub struct ListRecordCommand {
    /// category of the record
    #[arg(short, long)]
    pub category: Option<String>,
    /// days of records to retrieve,
    /// e.g. 1 shows record made in the last 24 hours,
    /// value of 7 would show record made in the past week
    #[arg(short, long, conflicts_with_all = ["starting_date", "ending_date"])]
    pub days: Option<usize>,
    /// limit the amount of records returned
    #[arg(short, long, default_value_t = 100, value_parser = validate_limit)]
    pub limit: usize,
    /// list the record starting from this time,
    /// if this is date only, then it is non-inclusive
    #[arg(short, long, value_parser = validate_timestr, conflicts_with = "days")]
    pub starting_time: Option<String>,
    /// list the record ending at this time,
    /// if this is date only, then it is inclusive
    #[arg(short, long, value_parser = validate_timestr, conflicts_with = "days")]
    pub ending_time: Option<String>,
    /// next page if the previous list command reached limit
    #[arg(short, long, default_value_t = false)]
    pub next_page: bool,
    /// search for records containing this text in their content
    #[arg(long)]
    pub search: Option<String>,
}

#[derive(Debug, Args)]
pub struct ShowContentCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
}

#[derive(Debug, Subcommand)]
pub enum UserCommand {
    /// create a new user
    Create(UserCreateCommand),
    /// list all users
    List,
    /// delete a user
    Delete(UserDeleteCommand),
}

#[derive(Debug, Args)]
pub struct UserCreateCommand {
    /// username (unique identifier)
    pub name: String,
    /// display name for the user
    #[arg(short = 'd', long = "display-name")]
    pub display_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct UserDeleteCommand {
    /// username to delete
    pub name: String,
}

#[derive(Debug, Subcommand)]
pub enum NamespaceCommand {
    /// create a new namespace
    Create(NamespaceCreateCommand),
    /// list all namespaces
    List,
    /// delete a namespace
    Delete(NamespaceDeleteCommand),
    /// switch default namespace
    Switch(NamespaceSwitchCommand),
    /// add a user to a namespace
    AddUser(NamespaceAddUserCommand),
    /// remove a user from a namespace
    RemoveUser(NamespaceRemoveUserCommand),
    /// list members of a namespace
    Members(NamespaceMembersCommand),
}

#[derive(Debug, Args)]
pub struct NamespaceCreateCommand {
    /// namespace name (unique identifier)
    pub name: String,
    /// description of the namespace
    #[arg(short = 'd', long)]
    pub description: Option<String>,
}

#[derive(Debug, Args)]
pub struct NamespaceDeleteCommand {
    /// namespace name to delete
    pub name: String,
}

#[derive(Debug, Args)]
pub struct NamespaceSwitchCommand {
    /// namespace name to switch to
    pub name: String,
}

#[derive(Debug, Args)]
pub struct NamespaceAddUserCommand {
    /// namespace name
    pub namespace: String,
    /// username to add
    pub user: String,
    /// role for the user (owner, admin, member, viewer)
    #[arg(short, long, default_value = "member")]
    pub role: String,
}

#[derive(Debug, Args)]
pub struct NamespaceRemoveUserCommand {
    /// namespace name
    pub namespace: String,
    /// username to remove
    pub user: String,
}

#[derive(Debug, Args)]
pub struct NamespaceMembersCommand {
    /// namespace name (defaults to current namespace)
    pub namespace: Option<String>,
}

#[derive(Debug, Args)]
pub struct NoteCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
    /// note content to add
    pub content: String,
}

#[derive(Debug, Args)]
pub struct ShowCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
}

#[derive(Debug, Args)]
pub struct ClaimCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
}

#[derive(Debug, Args)]
pub struct LinkCommand {
    /// index from previous list command
    #[arg(value_parser = validate_index)]
    pub index: usize,
    /// attach a commit hash
    #[arg(long)]
    pub commit: Option<String>,
    /// attach a GitHub issue (e.g., owner/repo#42)
    #[arg(long)]
    pub issue: Option<String>,
    /// attach a pull request (e.g., owner/repo#43)
    #[arg(long)]
    pub pr: Option<String>,
    /// attach a URL
    #[arg(long)]
    pub url: Option<String>,
    /// optional title for the link
    #[arg(short, long)]
    pub title: Option<String>,
}

#[derive(Debug, Args)]
pub struct TeamCommand {
    /// output as JSON
    #[arg(long)]
    pub json: bool,
    /// output as Markdown
    #[arg(long)]
    pub md: bool,
}

#[derive(Debug, Args)]
pub struct WorkloadCommand {
    /// filter to specific user
    #[arg(short, long)]
    pub user: Option<String>,
    /// output as JSON
    #[arg(long)]
    pub json: bool,
    /// output as Markdown
    #[arg(long)]
    pub md: bool,
}

#[derive(Debug, Args)]
pub struct StatsCommand {
    /// time period in days (default 30)
    #[arg(short, long, default_value_t = 30)]
    pub days: i64,
    /// output as JSON
    #[arg(long)]
    pub json: bool,
    /// output as Markdown
    #[arg(long)]
    pub md: bool,
}

fn syntax_helper(cmd: &str, s: &str) -> Result<String, String> {
    if s == "list" {
        return Err(format!("Do you mean 'list {}' instead of '{} list'", cmd, cmd));
    }
    if s == "help" {
        return Err("Do you mean --help instead of help".to_string());
    }
    Ok(s.to_string())
}

fn validate_limit(s: &str) -> Result<usize, String> {
    let limit: usize = s.parse().map_err(|_| "Must be a number".to_string())?;
    if limit < 1 {
        return Err("Limit cannot be less than 1".to_string());
    }
    if limit > 65536 {
        return Err("Limit cannot exceed 65536".to_string());
    }
    Ok(limit)
}

fn validate_index(s: &str) -> Result<usize, String> {
    let index: usize = s.parse().map_err(|_| "Index must be a number".to_string())?;
    if index == 0 {
        return Err("Index must be greater than 0".to_string());
    }
    if index > 65536 {
        return Err("Index cannot exceed 65536".to_string());
    }
    Ok(index)
}

fn validate_timestr(s: &str) -> Result<String, String> {
    match parse_flexible_timestr(s) {
        Ok(_) => Ok(s.to_string()),
        Err(_) => {
            match parse_recurring_timestr(s) {
                Ok(_) => Ok(s.to_string()),
                Err(e) => Err(e)
            }
        }
    }
}

fn parse_status(s: &str) -> Result<u8, String> {
    match s.to_lowercase().as_str() {
        "ongoing" => Ok(0),
        "done" | "complete" | "completed" => Ok(1),
        "cancelled" | "canceled" | "cancel" => Ok(2),
        "duplicate" => Ok(3),
        "deferred" | "suspended" | "shelved" => Ok(4),
        "removed" | "remove" | "unneeded" | "unnecessary" => Ok(5),
        "pending" => Ok(6),
        "closed" => Ok(253), // combination of done | cancelled | duplicate | removed
        "open" => Ok(254), // combination of ongoing | pending | suspended
        "all" => Ok(255), // all status
        _ => {
            s.parse::<u8>().map_err(|_| 
                format!("Invalid closing code: '{}'. Expected 'completed', 'cancelled', 'duplicate' or a number from 0-255", s)
            )
        }
    }
}

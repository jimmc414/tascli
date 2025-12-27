use rusqlite::Connection;

use crate::{
    actions::{
        addition,
        list,
        modify,
        namespace,
        user,
    },
    args::parser::{
        Action,
        CliArgs,
        ListCommand,
    },
    context::Context,
};

pub fn handle_commands(conn: &Connection, ctx: &Context, args: CliArgs) -> Result<(), String> {
    match args.arguments {
        Action::Task(cmd) => addition::handle_taskcmd(conn, ctx, &cmd),
        Action::Record(cmd) => addition::handle_recordcmd(conn, &cmd),
        Action::Done(cmd) => modify::handle_donecmd(conn, &cmd),
        Action::Delete(cmd) => modify::handle_deletecmd(conn, &cmd),
        Action::Update(cmd) => modify::handle_updatecmd(conn, &cmd),
        Action::List(list_cmd) => match list_cmd {
            ListCommand::Task(cmd) => list::handle_listtasks(conn, cmd),
            ListCommand::Record(cmd) => list::handle_listrecords(conn, cmd),
            ListCommand::Show(cmd) => list::handle_showcontent(conn, cmd),
        },
        Action::User(cmd) => user::handle_user_cmd(conn, ctx, cmd),
        Action::Ns(cmd) => namespace::handle_namespace_cmd(conn, ctx, cmd),
    }
}

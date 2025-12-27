mod actions;
mod args;
mod config;
mod context;
mod db;
mod github;
mod utils;

use std::process::exit;

use actions::display::print_red;
use args::parser::CliArgs;
use clap::Parser;
use context::Context;

fn main() {
    let cli_args = CliArgs::parse();
    let conn = match db::conn::connect() {
        Ok(conn) => conn,
        Err(err) => {
            print_red(&format!("Error connecting to db file: {}", err));
            exit(1)
        }
    };

    // Resolve identity context
    let ctx = match Context::resolve(
        &conn,
        cli_args.as_user.as_deref(),
        cli_args.namespace.as_deref(),
    ) {
        Ok(ctx) => ctx,
        Err(err) => {
            print_red(&format!("Error resolving identity: {}", err));
            exit(1)
        }
    };

    let result = actions::handler::handle_commands(&conn, &ctx, cli_args);
    if result.is_err() {
        print_red(&format!("Error: {}", result.unwrap_err()));
        exit(1)
    }
}

#[cfg(test)]
pub mod tests;

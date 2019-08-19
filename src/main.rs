mod api;
mod lib;

use api::start_web_server;
use lib::constants::APP_DISPLAY_NAME;
use lib::db::handle_db_command;
use lib::entities::*;
use lib::service_manager::handle_service_command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        handle_command(Command::from_arguments(&args));
    }
}

fn display_about() {
    println!("\n{} is your digital helper\n", APP_DISPLAY_NAME);
}

fn handle_command(cmd: Command) {
    /*
        Note(MIKAEL) - Once this gets too messy install the clap lib,
        right now we don't need the overhead.
    */
    match cmd.name.as_str() {
        "start" => start_web_server(), /* Should only be used for debugging */
        "service" => handle_service_command(cmd.parameter.as_str()),
        "db" => run_with_app_context(cmd.parameter.as_str(), &handle_db_command),
        "about" => display_about(),
        "sandbox" => sandbox_fn(),
        _ => println!("Unknown command"),
    };
}

fn sandbox_fn() {}

fn run_with_app_context(command: &str, app_run_command: &Fn(&str, &AppConfig)) {
    let database_path = lib::db::func::get_database_path();
    let app_config = AppConfig { database_path };

    app_run_command(command, &app_config);
}

struct Command {
    name: String,
    parameter: String,
}

impl Command {
    fn from_arguments(args: &Vec<String>) -> Command {
        Command {
            name: args[1].to_lowercase(),
            parameter: if args.len() >= 3 {
                args[2].clone()
            } else {
                String::from("")
            },
        }
    }
}

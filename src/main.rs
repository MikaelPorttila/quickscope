use std::env;
mod api;
mod lib;
mod service_manager;

use api::start_web_server;
use lib::constants::APP_DISPLAY_NAME;
use service_manager::handle_service_command;

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
    match cmd.name.as_str() {
        "start" => start_web_server(), /* Should only be used for debugging */
        "service" => handle_service_command(cmd.parameter.as_str()),
        "about" => display_about(),
        "sandbox" => sandbox_fn(),
        _ => println!("Unknown command"),
    };
}

fn sandbox_fn() {
    /* */
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

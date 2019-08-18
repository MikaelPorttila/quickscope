mod connection;
mod migration;
mod schema;
mod functions;

pub use super::constants;

pub fn handle_db_command(command: &str) {
	match command {
		"install" => migration::setup(),
		_ => println!("unknown"),
	};
}

mod connection;
mod functions;
mod migration;
mod schema;

pub use super::constants;

pub fn handle_db_command(command: &str) {
	match command {
		"install" => migration::setup(),
		_ => println!("unknown"),
	};
}

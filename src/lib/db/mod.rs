pub mod func;
mod migration;
mod schema;

use super::entities::AppConfig;
pub use super::constants;
pub use super::entities;

pub fn handle_db_command(command: &str, app_config: &AppConfig) {
	match command {
		"install" => migration::setup(app_config),
		_ => println!("unknown"),
	};
}
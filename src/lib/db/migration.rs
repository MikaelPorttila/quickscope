/*
	TODO (MIKAEL) - Add or build db migration lib.
*/
use super::entities::AppConfig;
use super::schema::*;
use rusqlite::NO_PARAMS;

pub fn setup(app_config: &AppConfig) {
	let db_context = DBContext::new(app_config);

	println!("Database migration started...");

	let settings_table_exists = db_context.check_if_table_exists("Settings").unwrap();

	if !settings_table_exists {
		db_context
			.connection
			.execute(
				"CREATE TABLE Settings(
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			value TEXT NOT NULL)",
				NO_PARAMS,
			)
			.unwrap();

		let setting_row =
			DBSetting::from_values(DB_VERSION_SETTING_NAME.to_string(), 1.to_string());
		db_context.insert_db_setting(&setting_row).unwrap();
		print_database_version_update(1);
	}

	let mut version_row = db_context
		.get_db_setting_by_name(DB_VERSION_SETTING_NAME)
		.unwrap();
	let mut version = version_row.value.parse().unwrap();

	if version < 2 {
		version = 2;
		version_row.value = 2.to_string();
		db_context.update_db_setting(&version_row).unwrap();
		print_database_version_update(version);
	}
	println!("Database setup completed");
}

fn print_database_version_update(version: i32) {
	println!("v.{} installed", version);
}

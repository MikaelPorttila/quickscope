/*
	TODO(Mikael) - Add create table func
*/

use super::entities::AppConfig;
use super::schema::*;
use rusqlite::{params, Connection, Error as RusqliteError}; /* NO_PARAMS */
use std::path::PathBuf;

impl DBContext {
	pub fn new(app_config: &AppConfig) -> DBContext {
		let connection = Connection::open(app_config.database_path.clone()).unwrap();

		DBContext { connection }
	}

	/* Settings table
	=====================*/
	pub fn get_db_setting_by_name(&self, name: &str) -> Result<DBSetting, RusqliteError> {
		let result = self.connection.query_row::<DBSetting, _, _>(
			"SELECT id, name, value from Settings where name = ?1",
			params![name],
			|row| {
				Ok(DBSetting {
					id: row.get_unwrap(0),
					name: row.get_unwrap(1),
					value: row.get_unwrap(2),
				})
			},
		);

		return result;
	}

	pub fn insert_db_setting(&self, setting_row: &DBSetting) -> Result<usize, RusqliteError> {
		return self.connection.execute(
			"INSERT INTO Settings(name, value) values(?1, ?2)",
			params![setting_row.name, setting_row.value],
		);
	}

	pub fn update_db_setting(&self, setting_row: &DBSetting) -> Result<usize, RusqliteError> {
		return self.connection.execute(
			"UPDATE Settings SET value=?1 WHERE name=?2",
			params![setting_row.value, setting_row.name],
		);
	}

	/* Sqlite Master table queries
	==============================*/
	pub fn check_if_table_exists(&self, table_name: &str) -> Result<bool, RusqliteError> {
		let table_count = self
			.connection
			.query_row::<i32, _, _>(
				"SELECT count(name) 
			FROM sqlite_master 
			WHERE type='table' AND name=?1",
				params![table_name],
				|row| row.get(0),
			)
			.unwrap();
		let result = table_count > 0;
		return Ok(result);
	}
}

pub fn get_database_path() -> String {
	let exe_path = std::env::current_exe().unwrap();
	let target_dir = exe_path.parent().unwrap();

	// build database path
	let mut db_path = PathBuf::new();
	db_path.push(target_dir);
	db_path.push(super::constants::DATABASE_FILENAME);

	return format!("{}", db_path.display());
}

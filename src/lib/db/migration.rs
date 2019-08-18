/*
	TODO (MIKAEL) - Add or build db migration lib.
*/
use super::connection::open_connection;
use super::functions::*;
use super::schema::*;
use rusqlite::{params, Connection, NO_PARAMS};

pub fn setup() {
	println!("Database migration started...");

	let conn = open_connection();

	let mut version = conn
		.query_row::<u32, _, _>(
			"SELECT count(name) 
			FROM sqlite_master 
			WHERE type='table' AND name='Settings'",
			NO_PARAMS,
			|row| row.get(0),
		)
		.unwrap();

	/*
		VERSION 1:
	*/
	if version == 0 {
		conn.execute(
			"CREATE TABLE Settings(
			id INTEGER PRIMARY KEY,
			name TEXT NOT NULL,
			value TEXT NOT NULL
		)",
			NO_PARAMS,
		)
		.unwrap();

		version = 1;
		conn.execute(
			"INSERT INTO Settings(id, name, value) values(?1, ?2, ?3)",
			params![0, DB_VERSION_SETTING_NAME, version],
		)
		.unwrap();
		log_database_version_update(version);
	} else {
		let version_row = get_row_from_settings_by_name(&conn, DB_VERSION_SETTING_NAME).unwrap();
		version = version_row.value.parse().unwrap();
	}

	/*
		VERSION 2: Testing stuff...
	*/
	if version < 2 {
		version = 2;
		update_database_version(version, &conn);
	}

	println!("Database setup completed");
}

fn update_database_version(version: u32, conn: &Connection) {
	conn.execute(
		"UPDATE Settings set value=?1 where name = ?2",
		params![version, DB_VERSION_SETTING_NAME],
	)
	.unwrap();
	log_database_version_update(version);
}

fn log_database_version_update(version: u32) {
	println!("v.{} installed", version);
}

use super::schema::DBSetting;
use super::connection::open_connection;
use rusqlite::{Error as RusqliteError, params, Connection};

pub fn get_row_from_settings_by_name(conn: &Connection, name: &str) -> Result<DBSetting, RusqliteError> {
		let result = conn
			.query_row::<DBSetting, _, _>(
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

impl DBSetting {
	pub fn get_from_db_by(name: &str) -> Result<DBSetting, RusqliteError> {
		let conn = open_connection();
		let result = get_row_from_settings_by_name(&conn, name);
		return result; 
	}
}
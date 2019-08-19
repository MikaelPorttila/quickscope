/*
	TODO (MIKAEL) - Go banana with macros or t4 script...
*/
use rusqlite::Connection;
pub const DB_VERSION_SETTING_NAME: &'static str = "version";

pub struct DBContext {
	pub connection: Connection,
}

pub struct DBSetting {
	pub id: i32,
	pub name: String,
	pub value: String,
}

impl DBSetting {
	pub fn from_values(name: String, value: String) -> DBSetting {
		return DBSetting {
			id: -1,
			name,
			value,
		};
	}
}
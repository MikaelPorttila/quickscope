/*
	TODO (MIKAEL) - Go banana with macros or t4 script...
*/

pub const DB_VERSION_SETTING_NAME: &'static str = "version";

pub struct DBSetting {
	pub id: u32,
	pub name: String,
	pub value: String,
}

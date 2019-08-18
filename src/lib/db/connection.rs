use rusqlite::Connection;
use std::path::PathBuf;

pub fn open_connection() -> Connection {
	// resolve exe path
	let exe_path = std::env::current_exe().unwrap();
	let target_dir = exe_path.parent().unwrap();

	// build database path
	let mut db_path = PathBuf::new();
	db_path.push(target_dir);
	db_path.push(super::constants::DATABASE_FILENAME);

	// open connection
	let result = Connection::open(format!("{}", db_path.display())).unwrap();
	return result;
}

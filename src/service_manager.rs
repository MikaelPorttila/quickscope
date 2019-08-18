/* 
	TODO (Mikael) - Add cross-platform support and stop abusing sc.exe.
	TODO (Mikael) - Require ADMIN privileges before running commands.
	TODO (Mikael) - Logging.
*/

use super::lib::constants::APP_NAME;
use std::process::Command;

pub fn handle_service_command(command: &str) {
	match command {
		"install" => install_service(),
		"start" => start_service(),
		"uninstall" => uninstall_service(),
		_ => println!("Service commands: install, start, restart"),
	};
}

fn install_service() {
	let exe_path = std::env::current_exe().unwrap();
	run_cmd(
		"create",
		&format!(r#"binpath="{}" start=auto"#, exe_path.display()),
	);
	run_cmd("description", "Helps you find files and stuff.");
	run_cmd("start", "");
}

fn start_service() {
	run_cmd("start", "");
}

fn uninstall_service() {
	run_cmd("stop", "");
	run_cmd("delete", "");
}

fn run_cmd(cmd: &str, args: &str) {
	print!("{} service...", cmd);

	let args = format!("sc {} {} {}", cmd, APP_NAME, args);

	println!("\n{}\n", args);

	Command::new("cmd")
		.args(&["/C", &args])
		.output()
		.expect("failed to execute process");

	println!("complete");
}

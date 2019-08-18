/*
	TODO(MIKAEL) - Change target port.
*/

mod hello_world_controller;
mod index_controller;
use actix_web::{web, App, HttpServer};

use hello_world_controller::{hello_world_action, HELLO_WORLD_API_ROUTE};
use index_controller::{default_action, DEFAULT_ACTION_ROUTE};

pub fn start_web_server() {
	println!("Start web server...");
	HttpServer::new(|| {
		App::new()
			.route(DEFAULT_ACTION_ROUTE, web::get().to(default_action))
			.route(HELLO_WORLD_API_ROUTE, web::get().to(hello_world_action))
	})
	.bind("127.0.0.1:8000")
	.expect("Can't bind to port 8000")
	.run()
	.unwrap();

	println!("Web server shutdown");
}
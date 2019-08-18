/*
	TODO(MIKAEL) - Change target port.
*/

mod hello_world_controller;
mod index_controller;
use actix_web::{web, App, HttpServer};

pub fn start_web_server() {
	println!("Running web server");
	HttpServer::new(|| App::new().configure(config))
		.bind("127.0.0.1:8000")
		.expect("Can't bind to port 8000")
		.run()
		.unwrap();
	println!("Web server shutdown");
}

fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(hello_world_controller::route());
	cfg.service(index_controller::route());
}

use actix_web::{web, HttpRequest, Responder, Scope};

pub fn route() -> Scope {
	return web::scope("/hello")
		.route("/{name}", web::get().to(hello_world_action))
		.route("", web::get().to(index));
}

fn index(_req: HttpRequest) -> impl Responder {
	return format!("Hello World");
}

fn hello_world_action(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or_else(|| "");
	return format!("Hello {}", name);
}

use actix_web::{web, HttpRequest, Responder, Scope};

pub fn route() -> Scope {
	return web::scope("/")
		.route("", web::get().to(index));
}

pub fn index(_req: HttpRequest) -> impl Responder {
	return format!("");
}
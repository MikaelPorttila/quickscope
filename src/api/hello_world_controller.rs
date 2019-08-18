use actix_web::{HttpRequest, Responder};

pub const HELLO_WORLD_API_ROUTE: &'static str = "/hello/{name}";
pub fn hello_world_action(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or_else(|| "");
	return format!("Hello {}", name);
}

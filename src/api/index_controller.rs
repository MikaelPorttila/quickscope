use actix_web::{HttpRequest, Responder};

pub const DEFAULT_ACTION_ROUTE: &'static str = "/";
pub fn default_action(_req: HttpRequest) -> impl Responder {
	return format!("");
}

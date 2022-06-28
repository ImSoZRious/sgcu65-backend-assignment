pub mod user;
pub mod user_guard;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct Json(String);

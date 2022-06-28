pub mod user;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct Json(String);

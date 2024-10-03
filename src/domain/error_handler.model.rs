use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::Request;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub error: String,
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 404,
        message: "this page is not exist ".to_string(),
        error: "Error 404 not found".to_string(),
    })
}

#[catch(default)]
pub fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})", status, req.uri());
    status::Custom(status, msg)
}

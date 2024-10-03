use crate::ErrorResponse;
use ethers::types::H160;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
pub trait ToAddress {
    fn to_address(&self) -> Result<H160, Custom<Json<ErrorResponse>>>;
}
// Err(status::Custom(
//     Status::BadRequest,
//     Json(ErrorResponse {
//         status: 400,
//         message: "Missing required parameter: token_in".to_string(),
//         error: "req.uri()".to_string(),
//     }),
// ))
impl ToAddress for String {
    fn to_address(&self) -> Result<H160, Custom<Json<ErrorResponse>>> {
        match self.parse() {
            Ok(address) => Ok(address),
            Err(e) => Err(Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: format!("checksum of {} is wrong", self.to_string()),

                    error: e.to_string(),
                }),
            )),
        }
    }
}

impl ToAddress for &str {
    fn to_address(&self) -> Result<H160, Custom<Json<ErrorResponse>>> {
        self.to_string().to_address()
    }
}

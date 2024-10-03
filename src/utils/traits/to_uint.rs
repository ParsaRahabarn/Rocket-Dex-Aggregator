use crate::ErrorResponse;
use ethers::types::U256;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

pub trait ToUint {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>>;
}

impl ToUint for String {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        self.parse::<U256>().map_err(|e| {
            Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: Status::BadRequest.code,
                    message: format!("Failed to parse string as U256: {}", self),
                    error: e.to_string(),
                }),
            )
        })
    }
}

impl ToUint for &str {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let u256 = self.to_string();
        u256.to_uint()
    }
}
impl ToUint for u128 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}
impl ToUint for u64 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}
impl ToUint for u32 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}
impl ToUint for u16 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}

impl ToUint for u8 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}
impl ToUint for i32 {
    fn to_uint(&self) -> Result<U256, Custom<Json<ErrorResponse>>> {
        let x = U256::from(*self);
        Ok(x)
    }
}

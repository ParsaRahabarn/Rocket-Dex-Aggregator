// src/dex_api.rs
use crate::infrastructure::api::version_1::responses::RawTeapotJson
use crate::quote::get_all_quotes;
use crate::utils::traits::*;
use crate::ErrorResponse;
use crate::QuoteOutput;
use rocket;
use rocket::get;
use rocket::http::Status;

use rocket::response::status; // Import this to resolve the 'status' module issue
use rocket::serde::json::Json;

#[get("/all?<token_in>&<token_out>&<amount_in>")]

pub async fn get_dex_data(
    token_in: Option<String>,
    token_out: Option<String>,
    amount_in: Option<String>,
) -> Result<Json<Vec<QuoteOutput>>, status::Custom<Json<ErrorResponse>>> {
    let token_in = match token_in {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "token_in is not passed".to_string(),
                    error: "Missing required parameter: token_in".to_string(),
                }),
            ))
        }
    };

    let token_out = match token_out {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "token_out is not passed".to_string(),
                    error: "Missing required parameter: token_out".to_string(),
                }),
            ))
        }
    };

    let amount_in = match amount_in {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "amount_in is not passed".to_string(),
                    error: "Missing required parameter: amount_in".to_string(),
                }),
            ))
        }
    };
    let amount = match amount_in.parse::<u128>() {
        Ok(value) => Ok(value),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorResponse {
                status: 400,
                message: format!("failed to parse {} to number", amount_in),
                error: e.to_string(),
            }),
        )),
    }?;

    let mut data = get_all_quotes(
        token_in.to_address()?,
        token_out.to_address()?,
        amount.to_uint()?,
    )
    .await;
    data.sort_by(|a, b| a.amount_out.cmp(&b.amount_out));

    Ok(Json(data))
}

#[get("/dex?<dex_name>&<token_in>&<token_out>&<amount_in>")]
pub async fn get_specific_data(
    dex_name: Option<String>,
    token_in: Option<String>,
    token_out: Option<String>,
    amount_in: Option<String>,
) -> Result<Json<Vec<QuoteOutput>>, status::Custom<Json<ErrorResponse>>> {
    let url = format!(
        "/api/v1/get_dex_data/all?{}&{}&{}",
        token_in.clone().unwrap_or("".to_string()),
        token_out.clone().unwrap_or("".to_string()),
        amount_in.clone().unwrap_or("".to_string())
    );
    let _ = match dex_name {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "Missing required parameter: dex_name".to_string(),
                    error: url,
                }),
            ))
        }
    };

    let token_in = match token_in {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "Missing required parameter: token_in".to_string(),
                    error: url,
                }),
            ))
        }
    };

    let token_out = match token_out {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "Missing required parameter: token_out".to_string(),
                    error: url,
                }),
            ))
        }
    };

    let amount_in = match amount_in {
        Some(val) => val,
        None => {
            return Err(status::Custom(
                Status::BadRequest,
                Json(ErrorResponse {
                    status: 400,
                    message: "amount_in is not passed".to_string(),
                    error: "Missing required parameter: amount_in".to_string(),
                }),
            ))
        }
    };

    let amount = match amount_in.parse::<u128>() {
        Ok(value) => Ok(value),
        Err(e) => Err(status::Custom(
            Status::BadRequest,
            Json(ErrorResponse {
                status: 400,
                message: format!("failed to parse {} to number", amount_in),
                error: e.to_string(),
            }),
        )),
    }?;

    // Sample data
    let mut data = get_all_quotes(
        token_in.to_address()?,
        token_out.to_address()?,
        amount.to_uint()?,
    )
    .await;
    data.sort_by(|a, b| a.amount_out.cmp(&b.amount_out));

    Ok(Json(data))
}

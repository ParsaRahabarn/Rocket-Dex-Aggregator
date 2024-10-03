pub mod domain;

mod utils;
use domain::error_handler::{not_found, ErrorResponse};
use domain::QuoteOutput;

pub use utils::traits::{ToAddress, ToUint};
mod infrastructure;
use dotenv::dotenv;

pub use infrastructure::{get_dex_data, get_specific_data, quote};

use rocket::config::Config;
#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::log::info_;
use rocket::{Data, Request};

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let config = Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8000));

    rocket::custom(config)
        .attach(RequestLogger) // Attach the fairing globally
        .mount(
            "/api/v1/get_dex_data",
            routes![get_dex_data, get_specific_data],
        )
        .register("/", catchers![not_found])
}

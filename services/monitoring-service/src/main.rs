use std::env;

use monitoring_core::models::SystemInformation;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket::{get, launch, post, routes};
use service_lib::api_key::ApiKey;
use service_lib::api_key::ApiKeyVault;

#[get("/")]
fn version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[post("/error")]
fn error(_key: ApiKey<'_>) -> &'static str {
    "Hello, world!"
}

#[post("/system-info", data = "<info>")]
async fn system_info(_key: ApiKey<'_>, info: Json<SystemInformation>) -> std::io::Result<()> {
    let mut file = File::create("test.json").await?;
    file.write_all(serde_json::to_string(&info.0).unwrap().as_bytes())
        .await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    let api_key = env::var("MONITORING_API_KEY").expect("Expected api key in environment!");

    rocket::build()
        .manage(ApiKeyVault::new(&api_key))
        .mount("/", routes![error, system_info, version])
}

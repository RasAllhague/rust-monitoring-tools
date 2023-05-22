use monitoring_core::models::SystemInformation;
use rocket::serde::json::serde_json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket::{get, launch, routes, post};
use rocket::serde::{Deserialize, json::Json};

#[get("/")]
fn version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[post("/error")]
fn error() -> &'static str {
    "Hello, world!"
}

#[post("/system-info", data="<info>")]
async fn system_info(info: Json<SystemInformation>) -> std::io::Result<()> {
    let mut file = File::create("test.json").await?;
    file.write_all(serde_json::to_string(&info.0).unwrap().as_bytes()).await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![error, system_info, version])
}

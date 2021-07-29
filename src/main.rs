#![allow(non_snake_case)]
use crate::app_controller::addressFetchAll;
use crate::app_controller::addressSendOne;
use crate::app_module::AppModule;
use std::sync::Arc;
use actix_web::{App, HttpServer};
mod actix_handler;
mod app_controller;
mod app_http_controller;
mod app_module;
mod app_service;
mod actix_utils;
mod rx_utils;
mod models;
mod streams_utils;
use crate::app_controller::index;
use actix_web::middleware::Logger;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // dotenv::dotenv().expect("Failed to read .env file");
  dotenv::from_path("./development.env").ok();
  println!("NODE is set to: {:?}",std::env::var("NODE").expect("NODE not defined as environment var"));
  let appModule = Arc::new(AppModule::builder().build());
  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(appModule.clone())
      .service(index)
      .service(addressSendOne)
      .service(addressFetchAll)
  })
  .bind("0.0.0.0:3030")?
  .run()
  .await?;
  Ok(server)
}

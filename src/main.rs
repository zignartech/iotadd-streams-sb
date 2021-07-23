use actix_web::rt::System;
use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Responder};
use tokio::runtime::Builder;
// use tokio::
// use actix_web::rt::System;
// use tokio::runtime::Handle;
// use tokio::runtime::Builder;
// use tokio::runtime::Runtime;
// tokio::task::

#[get("/")]
async fn index() -> &'static str {
  // no system arbiter.
  // assert!(System::try_current().is_none());

  // can spawn on tokio runtime.
  // let _ = tokio::spawn(async {
  // tokio::task::yield_now().await;
  // }).await;

  "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  tokio::task::spawn(async move {}).await?;
  let server = HttpServer::new(move || App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind("0.0.0.0:3030")?
        .run()
        .await?;
  Ok(server)
}

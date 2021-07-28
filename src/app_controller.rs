use crate::models::dtos::fetch_all_dto::FetchAllQuery;
use std::collections::HashMap;
use serde_json::Value;
use crate::models::dtos::send_one_dto::SendOneQuery;
use crate::actix_handler::inject_component::Inject;
use crate::app_http_controller::IAppHttpController;
use crate::app_module::AppModule;
use crate::models::dtos::create_author_dto::{CreateAuthorBody, CreateAuthorQuery};
use actix_web::{post, web, web::Query, web::Json, Responder,};
use actix_web::{HttpResponse};
use crate::rx_utils::poll_observable::pollObservable;

#[post("/author")]
pub async fn index(
  httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<CreateAuthorQuery>,
  body: Json<CreateAuthorBody>,
) -> impl Responder {
  let author = httpController.createAuthor(query.0.clone(),body.0.clone());
  return HttpResponse::Ok().json(pollObservable(author).await);
}

#[post("/address/sendOne")]
pub async fn addressSendOne(
  httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<SendOneQuery>,
  bytes: web::Bytes
) -> impl Responder {
  let s = String::from_utf8(bytes.to_vec()).unwrap();
  let json: HashMap<String, Value> = serde_json::from_str(&s).unwrap();
  let address = httpController.sendOne(query.0.clone(),json);
  return HttpResponse::Ok().json(pollObservable(address).await);
}
#[post("/address/fetchAll")]
pub async fn addressFetchAll(
  httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<FetchAllQuery>,
) -> impl Responder {
  let address = httpController.fetchAll(query.0.clone());
  return HttpResponse::Ok().json(pollObservable(address).await);
}
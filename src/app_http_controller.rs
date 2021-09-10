// use futures::Future;
//use crate::app_service::IAppService;
use crate::models::dtos::create_author_dto::CreateAuthorBody;
use crate::models::dtos::create_author_dto::CreateAuthorQuery;
use crate::models::dtos::fetch_all_dto::FetchAllQuery;
use crate::models::dtos::send_one_dto::SendOneQuery;
use crate::models::schemas::author_schema::Address;
use crate::models::schemas::author_schema::AuthorSchema;
use rxrust::observable::of::OfEmitter;
use rxrust::observable::ObservableBase;
use serde_json::Value;
use shaku::{Component, Interface, };
use std::collections::HashMap;
use std::sync::Arc;
/*
pub trait IAppHttpController: Interface {
  fn createAuthor(
    &self,
    createAuthorQuery: CreateAuthorQuery,
    createAuthorBody: CreateAuthorBody,
  ) -> ObservableBase<OfEmitter<AuthorSchema>>;
  fn sendOne(
    &self,
    sendOneQuery: SendOneQuery,
    anyBody: HashMap<String, Value>,
 // ) -> ObservableBase<OfEmitter<Address>>;
  ) -> Result< ObservableBase<OfEmitter<Address>>, &str>;
  fn fetchAll(
    &self,
    fetchAllQuery: FetchAllQuery,
  ) -> ObservableBase<OfEmitter<Vec<HashMap<String, Value>>>>;
}

#[derive(Component)]
#[shaku(interface = IAppHttpController)]
pub struct AppHttpController {
  #[shaku(inject)]
  pub appService: Arc<dyn IAppService>,
}
impl IAppHttpController for AppHttpController {
  fn createAuthor(
    &self,
    createAuthorQuery: CreateAuthorQuery,
    createAuthorBody: CreateAuthorBody,
  ) -> ObservableBase<OfEmitter<AuthorSchema>> {
    return self
      .appService
      .createAuthor(createAuthorQuery, createAuthorBody);
  }

  fn sendOne(
    &self,
    sendOneQuery: SendOneQuery,
    anyBody: HashMap<String, Value>,
 // ) -> ObservableBase<OfEmitter<Address>> {
  )->Result< ObservableBase<OfEmitter<Address>>, &str>{
    return self.appService.sendOne(sendOneQuery, anyBody);
  }
  fn fetchAll(
    &self,
    fetchAllQuery: FetchAllQuery,
  ) -> ObservableBase<OfEmitter<Vec<HashMap<String, Value>>>> {
    return self.appService.fetchAll(fetchAllQuery);
  }
}
*/
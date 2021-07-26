use serde::{Deserialize,Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuthorSchema {
  pub seed: String,
  pub address: Address,
  pub author: Author,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Author {
  pub password: String,
  pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
  pub appInst: String,
  pub msgId: String,
}
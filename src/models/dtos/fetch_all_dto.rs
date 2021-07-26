use crate::actix_utils::deserialize_json_string::deserializeJsonString;
use crate::models::schemas::author_schema::Address;
use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FetchAllQuery {
  #[serde(deserialize_with = "deserializeJsonString")]
  pub address: Address,
}

// #[derive(Debug, Clone, Serialize, Deserialize, Validate)]
// pub struct SendOneBody {
//   pub seed: String,
// }

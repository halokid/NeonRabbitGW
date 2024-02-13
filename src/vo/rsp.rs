use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rsp {
  pub code:  i32,
  pub message: String,
  pub data: String,
}


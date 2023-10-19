use std::sync::{Arc, RwLock, RwLockReadGuard};
use crate::service::selector::selector::Selector;

pub struct Random {

}

impl Random {
  pub fn new() -> Self {
    Random{}
  }
}

impl Selector for Random {
  fn select_node(&mut self, nodes: RwLockReadGuard<Vec<String>>) -> String {
    todo!()
  }

  fn update_nodes(&mut self, service_nodes: Vec<String>) -> bool {
    todo!()
  }

  fn show_nodes(&self) -> String {
    todo!()
  }
}
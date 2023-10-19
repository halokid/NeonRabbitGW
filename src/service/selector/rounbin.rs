use std::sync::{Arc, RwLock, RwLockReadGuard};
use crate::service::selector::selector::Selector;

#[derive(Default)]
pub struct Roundbin {
  pub servers: Arc<RwLock<Vec<String>>>,
  index: usize,
}

unsafe impl Sync for Roundbin{}
unsafe impl Send for Roundbin{}

impl Roundbin {
  pub fn new() -> Self {
    Roundbin {
      servers: Arc::new(RwLock::new(Vec::new())),
      index: 0,
    }
  }
}

impl Selector for Roundbin {
  fn select_node(&mut self, nodes: RwLockReadGuard<Vec<String>>) -> String {
    // let nodes = (*self).servers.read().unwrap();
    let size = nodes.len();
    if size == 0 {
      return  String::new()
    }
    self.index = (self.index + 1) % size;
    let s = &nodes[self.index];
    String::from(s)
  }

  fn update_nodes(&mut self, service_nodes: Vec<String>) -> bool {
    let mut nodes = self.servers.write().unwrap();
    nodes.clear();
    for node in service_nodes {
      nodes.push(node)
    }
    true
  }

  fn show_nodes(&self) -> String {
    // let nodes = self.servers.read().unwrap();
    // format!("Roundbin show nodes -->>> {:?}", nodes)
    format!("-->>> use Roundbin, index is {}", self.index)
  }
}




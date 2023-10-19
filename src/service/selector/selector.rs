use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use crate::service::selector::random::Random;
use crate::service::selector::rounbin::Roundbin;

pub trait Selector {
  fn select_node(&mut self, nodes: RwLockReadGuard<Vec<String>>) -> String;
  fn update_nodes(&mut self, _: Vec<String>) -> bool;
  fn show_nodes(&self) -> String;
}

impl Debug for dyn Selector {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    // write!(f, "Selector")
    write!(f, "{}", self.show_nodes())
  }
}

pub enum Adaptee {
  Random,
  RoundRobin,
}

impl Debug for Adaptee {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    // write!(f, "{:?}", self)
    match *self {
      Adaptee::Random => {
        write!(f, "Random")
      }
      Adaptee::RoundRobin => {
        write!(f, "RoundRobin")
      }
    }
  }
}


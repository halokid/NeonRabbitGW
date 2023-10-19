
use std::fmt;

pub mod service;
pub mod websocket;
pub mod middleware;
pub mod pkg;
mod config;
pub mod registry;

#[derive(Debug)]
pub struct CustomErr (pub String);

impl fmt::Display for CustomErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "CustomErr => {}", self.0)
  }
}






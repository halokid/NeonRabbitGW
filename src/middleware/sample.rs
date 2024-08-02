use crate::middleware::middleware::MiddleWarePl;

pub struct SampleMw {}

impl SampleMw {
  pub fn new() -> Self {
    SampleMw{}
  }
}

impl MiddleWarePl for SampleMw {
  fn filter(&self) -> bool {
    return true
  }
}
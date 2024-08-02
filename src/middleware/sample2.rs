use crate::middleware::middleware::MiddleWarePl;

pub struct SampleMw2 {}

impl SampleMw2 {
  pub fn new() -> Self {
    SampleMw2{}
  }
}

impl MiddleWarePl for SampleMw2 {
  fn filter(&self) -> bool {
    return false;
  }
}
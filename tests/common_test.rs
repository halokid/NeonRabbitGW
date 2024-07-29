#[cfg(test)]
mod tests {
  // 导入需要测试的模块或文件
  // use super::my_module;

  use NeonRabbitGW::pkg::rsp::fail_rsp;

  #[test]
  fn test_example() {
    // 编写测试逻辑
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_example1() {
    // 编写测试逻辑
    assert_eq!(2 + 3, 5);
  }

  // 添加更多测试...
  #[test]
  fn test_rsp() {
    let rsp = fail_rsp("".to_string());
    println!("rsp -->>> {:?}", rsp);
  }
}



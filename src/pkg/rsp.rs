use crate::vo::rsp::Rsp;

pub fn SuccessRsp() -> Rsp {
  Rsp {
    code: 0,
    message: "success".to_string(),
    data: "".to_string()
  }
}

pub fn SuccessRspData(data: String) -> Rsp {
  Rsp {
    code: 0,
    message: "success".to_string(),
    data: data,
  }
}

pub fn FailRsp(mut message: String) -> Rsp {
  if message == "".to_string() {
    message = "Failt".to_string();
  }
  Rsp {
    code: 1,
    message: message,
    data: "".to_string()
  }
}


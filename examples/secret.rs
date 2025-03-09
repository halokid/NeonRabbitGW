use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rand::{Rng, RngCore};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// **生成随机 AES 密钥**
fn generate_key() -> Key<Aes256Gcm> {
  let mut key_bytes = [0u8; 32];
  rand::thread_rng().fill_bytes(&mut key_bytes);
  // Key::<Aes256Gcm>::generate(&mut OsRng)
  Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

/// **从文件加载 JSON**
fn load_json_from_file(file_path: &str) -> Value {
  let file_content = fs::read_to_string(file_path).expect("无法读取文件");
  serde_json::from_str(&file_content).expect("JSON 解析失败")
}

/// **加密 JSON 数据**
fn encrypt_json(key: &Key<Aes256Gcm>, json_data: &Value) -> (String, String) {
  let cipher = Aes256Gcm::new(key);

  // 生成 12 字节的随机 nonce
  let nonce_bytes: [u8; 12] = rand::thread_rng().gen();
  let nonce = Nonce::from_slice(&nonce_bytes);

  // JSON 转换为字符串
  let json_str = json_data.to_string();

  // 加密 JSON 数据
  let encrypted_bytes = cipher.encrypt(nonce, json_str.as_bytes()).expect("加密失败");

  // 使用 Base64 编码存储
  let encrypted_data = base64::encode(encrypted_bytes);
  let nonce_str = base64::encode(nonce_bytes);

  (nonce_str, encrypted_data)
}

/// **解密 JSON 数据**
fn decrypt_json(key: &Key<Aes256Gcm>, nonce_str: &str, encrypted_data: &str) -> Value {
  let cipher = Aes256Gcm::new(key);

  // 解码 Base64 格式的 nonce 和加密数据
  let nonce_bytes = base64::decode(nonce_str).expect("Nonce 解码失败");
  let encrypted_bytes = base64::decode(encrypted_data).expect("加密数据解码失败");

  // 还原 nonce
  let nonce = Nonce::from_slice(&nonce_bytes);

  // 解密
  let decrypted_bytes = cipher.decrypt(nonce, encrypted_bytes.as_ref()).expect("解密失败");

  // 转换回 JSON
  let json_str = String::from_utf8(decrypted_bytes).expect("解密结果不是合法 UTF-8");
  serde_json::from_str(&json_str).expect("解析 JSON 失败")
}

fn main() {
  // let file_path = "./data.json"; // 你的 JSON 文件
  let file_path = Path::new("examples/data.json");
  println!("File path: {:?}", file_path);
  let key = generate_key(); // 生成密钥（应存储）

  // **1. 读取 JSON 文件**
  let json_data = load_json_from_file(file_path.to_str().unwrap());
  println!("原始 JSON 数据: {}", json_data);

  // **2. 加密 JSON**
  let (nonce, encrypted_data) = encrypt_json(&key, &json_data);
  fs::write("encrypted_data.txt", format!("{}\n{}", nonce, encrypted_data))
    .expect("无法写入文件");
  println!("JSON 数据已加密并写入文件");

  // **3. 从文件读取加密数据**
  let content = fs::read_to_string("encrypted_data.txt").expect("无法读取加密文件");
  let mut lines = content.lines();
  let nonce_str = lines.next().unwrap();
  let encrypted_str = lines.next().unwrap();

  // **4. 解密 JSON**
  let decrypted_json = decrypt_json(&key, nonce_str, encrypted_str);
  println!("解密后的 JSON 数据: {}", decrypted_json);
}



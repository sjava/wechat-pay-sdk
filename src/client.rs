// use redis::aio::MultiplexedConnection;
use crate::WeChatPayError;
use aes_gcm::aead::{consts::U32, generic_array::GenericArray};
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct PlatformPubKey {
  pub serial_no: String,
  pub expire_time: String,
  pub effective_time: String,
  pub pub_key: String,
}
#[derive(Debug)]
pub struct Client {
  pub merchant_id: String,
  pub(crate) private_key: RsaPrivateKey,
  pub(crate) merchant_serial_number: String,
  pub(crate) api_key: GenericArray<u8, U32>,
  // pub(crate) redis: MultiplexedConnection,
  pub(crate) client: reqwest::Client,
  pub(crate) platform_pub_keys: Option<Vec<PlatformPubKey>>,
}

impl Client {
  /// Create a new client.
  ///
  /// # Arguments
  ///
  /// * `merchant_id` - 商户号
  /// * `private_key_path` - 商户 API 私钥路径
  /// * `merchant_serial_number` - 商户 API 证书序列号
  /// * `api_key` - 商户 APIv3 密钥
  pub fn new(
    merchant_id: &str,
    private_key_path: &str,
    merchant_serial_number: &str,
    api_key: &str,
    platform_pub_keys: Option<Vec<PlatformPubKey>>,
    // redis: MultiplexedConnection,
  ) -> Result<Self, WeChatPayError> {
    Ok(Self {
      merchant_id: merchant_id.to_string(),
      private_key: RsaPrivateKey::from_pkcs8_pem(&read_to_string(private_key_path)?)?,
      merchant_serial_number: merchant_serial_number.to_string(),
      api_key: GenericArray::from_slice(api_key.as_bytes()).to_owned(),
      // redis,
      client: reqwest::Client::new(),
      platform_pub_keys,
    })
  }
  pub fn get_pub_key(&self, serial_no: &str) -> Option<&PlatformPubKey> {
    self
      .platform_pub_keys
      .as_ref()?
      .iter()
      .find(|x| x.serial_no == serial_no)
  }
}

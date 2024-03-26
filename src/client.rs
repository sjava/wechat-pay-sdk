// use redis::aio::MultiplexedConnection;
use crate::WeChatPayError;
use aes_gcm::aead::{consts::U32, generic_array::GenericArray};
use chrono::Utc;
use rsa::{
  pkcs8::{DecodePrivateKey, DecodePublicKey},
  RsaPrivateKey, RsaPublicKey,
};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct PlatformPubKey {
  pub serial_no: String,
  pub expire_time: u64,
  pub effective_time: u64,
  pub key: String,
}
#[derive(Debug)]
pub struct PlatformPubKeyInner {
  pub serial_no: String,
  pub expire_time: u64,
  pub effective_time: u64,
  pub key: RsaPublicKey,
}
impl TryFrom<PlatformPubKey> for PlatformPubKeyInner {
  type Error = WeChatPayError;
  fn try_from(value: PlatformPubKey) -> Result<Self, Self::Error> {
    Ok(Self {
      serial_no: value.serial_no,
      expire_time: value.expire_time,
      effective_time: value.effective_time,
      key: RsaPublicKey::from_public_key_pem(&value.key)
        .map_err(|_| WeChatPayError::Unknown("public key parse error".to_string()))?,
    })
  }
}

#[derive(Debug)]
pub struct Client {
  pub merchant_id: String,
  pub(crate) private_key: RsaPrivateKey,
  pub(crate) merchant_serial_number: String,
  pub(crate) api_key: GenericArray<u8, U32>,
  pub(crate) public_keys: Vec<PlatformPubKeyInner>,
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
    platform_pub_keys: Vec<PlatformPubKey>,
    // redis: MultiplexedConnection,
  ) -> Result<Self, WeChatPayError> {
    let mut client = Self {
      merchant_id: merchant_id.to_string(),
      private_key: RsaPrivateKey::from_pkcs8_pem(&read_to_string(private_key_path)?)?,
      merchant_serial_number: merchant_serial_number.to_string(),
      api_key: GenericArray::from_slice(api_key.as_bytes()).to_owned(),
      public_keys: Vec::new(),
    };
    client.update_public_keys(platform_pub_keys);
    Ok(client)
  }
  pub fn get_public_key(&self, serial_no: &str) -> Option<&PlatformPubKeyInner> {
    self.public_keys.iter().find(|x| x.serial_no == serial_no)
  }
  pub fn update_public_keys(&mut self, platform_pub_keys: Vec<PlatformPubKey>) {
    let platform_pub_keys = platform_pub_keys
      .into_iter()
      .filter_map(|key| PlatformPubKeyInner::try_from(key).ok())
      .collect::<Vec<_>>();
    if !platform_pub_keys.is_empty() {
      self.public_keys = platform_pub_keys;
    }
  }
  // select the latest public key
  pub fn get_latest_public_key(&self) -> Option<&PlatformPubKeyInner> {
    self
      .public_keys
      .iter()
      .filter(|x| {
        let now = Utc::now().timestamp().try_into().unwrap();
        x.effective_time < now && x.expire_time > now
      })
      .max_by_key(|x| x.effective_time)
  }
}

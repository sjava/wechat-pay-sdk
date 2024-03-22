use crate::{Client, WeChatPayError};
use aes_gcm::aead::Payload;
use aes_gcm::{
  aead::{Aead, KeyInit, Nonce},
  Aes256Gcm,
};
use base64::{engine::general_purpose, Engine};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::{header, Method, Response, StatusCode, Url};
use std::time::{SystemTime, UNIX_EPOCH};

use rsa::{
  pkcs8::DecodePublicKey,
  sha2::{Digest, Sha256},
  Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey,
};

impl Client {
  pub fn sha256_with_rsa(
    &self,
    content: &[u8],
    private_key: Option<RsaPrivateKey>,
  ) -> Result<String, WeChatPayError> {
    let private_key = private_key.unwrap_or(self.private_key.clone());
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hex = hasher.finalize();
    let signature = private_key.sign(Pkcs1v15Sign::new::<Sha256>(), &hex)?;
    Ok(general_purpose::STANDARD.encode(signature))
  }

  pub fn aead_aes_256_gcm_encrypt(
    &self,
    nonce: &[u8],
    plaintext: &[u8],
    associated_data: Option<&[u8]>,
  ) -> Result<Vec<u8>, WeChatPayError> {
    let cipher = Aes256Gcm::new(&self.api_key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce);
    let ciphertext = if let Some(aad) = associated_data {
      cipher.encrypt(
        nonce,
        Payload {
          msg: plaintext,
          aad,
        },
      )?
    } else {
      cipher.encrypt(nonce, plaintext)?
    };
    Ok(ciphertext)
  }

  pub fn aead_aes_256_gcm_decrypt(
    &self,
    nonce: &[u8],
    ciphertext: &[u8],
    associated_data: Option<&[u8]>,
  ) -> Result<Vec<u8>, WeChatPayError> {
    let cipher = Aes256Gcm::new(&self.api_key);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce);
    let plaintext = if let Some(aad) = associated_data {
      cipher.decrypt(
        nonce,
        Payload {
          msg: ciphertext,
          aad,
        },
      )?
    } else {
      cipher.decrypt(nonce, ciphertext)?
    };
    Ok(plaintext)
  }

  pub fn request_authorization(
    &self,
    method: &Method,
    path: &str,
    content: &str,
  ) -> Result<String, WeChatPayError> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    // TODO: don't create ThreadRng every time
    let nonce = thread_rng()
      .sample_iter(Alphanumeric)
      .take(32)
      .map(char::from)
      .collect::<String>();
    let content = format!(
      "{}\n{}\n{}\n{}\n{}\n",
      method.as_str(),
      path,
      timestamp,
      nonce,
      content
    );
    let signature = self.sha256_with_rsa(content.as_bytes(), None)?;
    Ok(format!(
      "WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",signature=\"{}\",timestamp=\"{}\",serial_no=\"{}\"",
      self.merchant_id,
      nonce,
      signature,
      timestamp,
      self.merchant_serial_number,
    ))
  }

  /// # 发送请求
  /// 包含
  /// [签名生成](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_0.shtml)
  /// [签名验证](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)
  /// 逻辑，返回结果参考
  /// [parse_response](Self::parse_response)
  pub async fn send_request<Request, Response>(
    &self,
    method: Method,
    url: &str,
    query: Option<&[(&str, &str)]>,
    body: Option<&Request>,
  ) -> Result<Option<Response>, WeChatPayError>
  where
    Request: serde::Serialize,
    Response: serde::de::DeserializeOwned + Send + 'static,
  {
    let verify = url == "/v3/certificates";
    let api = format!("https://api.mch.weixin.qq.com{}", url);
    let (url, path) = if let Some(query) = query {
      let u = Url::parse_with_params(&api, query)?;
      let query = u.query().unwrap_or("").to_string();
      (u, format!("{}?{}", url, query))
    } else {
      (Url::parse(&api)?, url.to_string())
    };
    let mut req = reqwest::Request::new(method.clone(), url);
    let content = if let Some(body) = body {
      req.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
      );
      *req.body_mut() = Some(serde_json::to_vec(body)?.into());
      serde_json::to_string(body)?
    } else {
      "".to_string()
    };
    let signature = self.request_authorization(&method, &path, &content)?;
    req.headers_mut().insert(
      header::ACCEPT,
      header::HeaderValue::from_str("application/json")?,
    );
    req.headers_mut().insert(
      header::USER_AGENT,
      header::HeaderValue::from_str("wechat-pay-sdk-rs")?,
    );
    req.headers_mut().insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&signature)?,
    );
    req.headers_mut().insert(
      "Wechatpay-Serial",
      header::HeaderValue::from_str(&self.merchant_serial_number)?,
    );

    let res = self.client.execute(req).await?;
    let status = res.status();
    // let signature = res.headers().get("wechatpay-signature").unwrap().to_str()?.to_string();
    // let nonce = res.headers().get("wechatpay-nonce").unwrap().to_str()?.to_string();
    // let timestamp = res.headers().get("wechatpay-timestamp").unwrap().to_str()?.to_string();
    // let cert_serial = res.headers().get("wechatpay-serial").unwrap().to_str()?.to_string();
    let text = res
      .text()
      .await
      .map_err(|_| WeChatPayError::Unknown("Failed to decode response".to_string()))?;

    // TODO: check response signature
    Self::parse_response(status, text).await
  }

  pub async fn verify_signatrue(
    &self,
    response:Response,
  ) -> Result<(StatusCode, String), WeChatPayError> {
    let signature = Self::get_header(&response, "Wechatpay-Signature")?;
    let nonce = Self::get_header(&response, "Wechatpay-Nonce")?;
    let timestamp = Self::get_header(&response, "Wechatpay-Timestamp")?;
    let serial_no = Self::get_header(&response, "Wechatpay-Serial")?;

    let status = response.status();
    let body = response.text().await?;

    let message = format!(
      "{}\n{}\n{}\n",
      timestamp.as_str(),
      nonce.as_str(),
      body.as_str()
    );
    let pub_key =
      self
        .get_pub_key(serial_no.as_ref())
        .ok_or(WeChatPayError::VerifySignatureFail(
          "No public key found".to_string(),
        ))?;

    let pub_key = RsaPublicKey::from_public_key_pem(&pub_key.pub_key).map_err(|e| {
      WeChatPayError::VerifySignatureFail(format!("public key parser error: {}", e))
    })?;
    let hashed = Sha256::new().chain_update(message).finalize();
    let signatrue = general_purpose::STANDARD
      .decode(signature.as_str())
      .map_err(|e| WeChatPayError::VerifySignatureFail(format!("signature decode error: {}", e)))?;
    let scheme = Pkcs1v15Sign::new::<Sha256>();
    pub_key
      .verify(scheme, &hashed, signatrue.as_slice())
      .map_err(|e| WeChatPayError::VerifySignatureFail(e.to_string()))
      .map(|_| (status, body))
  }

  fn get_header(response: &Response, key: &str) -> Result<String, WeChatPayError> {
    Ok(
      response
        .headers()
        .get(key)
        .ok_or_else(|| WeChatPayError::VerifySignatureFail(format!("Missing {}", key)))?
        .to_str()?
        .to_string(),
    )
  }
}

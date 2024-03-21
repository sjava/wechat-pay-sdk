//! # [上传图片](https://pay.weixin.qq.com/docs/partner/apis/ecommerce-merchant-application/upload.html)
//! 部分微信支付业务指定商户需要使用图片上传 API来上报图片信息，从而获得必传参数的值：图片MediaID 。
//!
//! 请求 URL: <https://api.mch.weixin.qq.com/v3/merchant/media/upload>
//!
//! 请求方式: POST
//! 请求主体类型： multipart/form-data
use crate::sdk::common::EmptyRequest;
use crate::{Client, WeChatPayError};
use reqwest::multipart::{Form, Part};
use reqwest::{header, Method};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct EncryptCertificate {
  pub algorithm: String,
  pub nonce: String,
  pub associated_data: String,
  pub ciphertext: String,
}

#[derive(Debug, Deserialize)]
pub struct CertificateData {
  pub serial_no: String,
  pub effective_time: String,
  pub expire_time: String,
  pub encrypt_certificate: EncryptCertificate,
}

/// # [上传图片响应](self) 响应
#[derive(Debug, Deserialize)]
pub struct UploadImageResponse {
  pub media_id: String,
}

impl Client {
  pub async fn upload_image(
    &self,
    image: &[u8],
    filename: &str,
  ) -> Result<UploadImageResponse, WeChatPayError> {
    const MAX_SIZE: usize = 2 * 1024 * 1024;
    const URL: &str = "/v3/merchant/media/upload";
    // image's size must be less than 2M
    // if image.len() > MAX_SIZE {
    //   return Err(PayError::WechatError(format!(
    //     "Image size ({} bytes) exceeds the maximum allowed size ({} bytes)",
    //     image.len(),
    //     MAX_SIZE
    //   )));
    // }

    // calculate sha256
    let mut hasher = Sha256::new();
    hasher.update(&image);
    let hash = hasher.finalize();
    let hash = hex::encode(hash.as_slice());

    let meta = json!( {
        "filename": filename,
        "sha256": hash
    });

    let path = format!("https://api.mch.weixin.qq.com{}", URL);

    let mut headers = header::HeaderMap::new();
    headers.insert(
      header::CONTENT_TYPE,
      header::HeaderValue::from_static("multipart/form-data"),
    );
    headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_str("application/json")?,
    );
    headers.insert(
      header::USER_AGENT,
      header::HeaderValue::from_str("wechat-pay-sdk-rs")?,
    );
    let signature = self.request_authorization(&Method::POST, &path, &meta.to_string())?;
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&signature)?,
    );
    req.headers_mut().insert(
      "Wechatpay-Serial",
      header::HeaderValue::from_str(&self.merchant_serial_number)?,
    );

    let mut json_part_headers = header::HeaderMap::new();
    json_part_headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    let json_part = Part::text(meta.to_string()).headers(json_part_headers);

    let file_part = Part::bytes(image)
      .file_name(filename.to_string())
      .mime_str("image/jpeg")?;

    let form = Form::new().part("meta", json_part).part("file", file_part);

    let res = self
      .client
      .post(&path)
      .headers(headers)
      .multipart(form)
      .send()
      .await?;

    // Ok(
    //   self
    //     .send_request::<EmptyRequest, _>(Method::GET, "/v3/certificates", None, None)
    //     .await?
    //     .unwrap(),
    // )
  }
}

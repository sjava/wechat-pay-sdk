//! # [上传图片](https://pay.weixin.qq.com/docs/partner/apis/ecommerce-merchant-application/upload.html)
//! 部分微信支付业务指定商户需要使用图片上传 API来上报图片信息，从而获得必传参数的值：图片MediaID 。
//!
//! 请求 URL: <https://api.mch.weixin.qq.com/v3/merchant/media/upload>
//!
//! 请求方式: POST
//! 请求主体类型： multipart/form-data
use crate::{Client, WeChatPayError};
use reqwest::multipart::{Form, Part};
use reqwest::{header, Method};
use rsa::sha2::{Digest, Sha256};
use serde::Deserialize;
use serde_json::json;

/// # [上传图片响应](self) 响应
#[derive(Debug, Deserialize)]
pub struct UploadImageResponse {
  pub media_id: String,
}

impl Client {
  pub async fn upload_image(
    &self,
    image: Vec<u8>,
    filename: &str,
  ) -> Result<UploadImageResponse, WeChatPayError> {
    // calculate sha256
    let mut hasher = Sha256::new();
    hasher.update(&image);
    let hash = hasher.finalize();
    let hash = hex::encode(hash.as_slice());

    let api = "/v3/merchant/media/upload";
    let meta = json!( {
        "filename": filename,
        "sha256": hash
    });
    let signature = self.request_authorization(&Method::POST, api, &meta.to_string())?;
    let headers = self.build_header(signature)?;
    let form = build_form(&meta, image, filename)?;
    let url = format!("https://api.mch.weixin.qq.com{}", api);
    let client = reqwest::Client::new();
    let response = client
      .post(&url)
      .headers(headers)
      .multipart(form)
      .send()
      .await?;
    let (status, text) = self.verify_signatrue(response).await?;

    let response = Self::parse_response::<UploadImageResponse>(status, text)
      .await?
      .unwrap();
    Ok(response)
  }

  fn build_header(&self, signature: String) -> Result<header::HeaderMap, WeChatPayError> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
      header::CONTENT_TYPE,
      header::HeaderValue::from_str("multipart/form-data")?,
    );
    headers.insert(
      header::ACCEPT,
      header::HeaderValue::from_str("application/json")?,
    );

    let chrome_agent = "Mozilla/5.0 (Linux; Android 10; Redmi K30 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Mobile Safari/537.36";
    headers.insert(
      header::USER_AGENT,
      header::HeaderValue::from_str(chrome_agent)?,
    );
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&signature)?,
    );
    headers.insert(
      "Wechatpay-Serial",
      header::HeaderValue::from_str(&self.merchant_serial_number)?,
    );
    Ok(headers)
  }
}

fn build_form(
  meta: &serde_json::Value,
  image: Vec<u8>,
  filename: &str,
) -> Result<Form, WeChatPayError> {
  let mut json_part_headers = header::HeaderMap::new();
  json_part_headers.insert(
    header::CONTENT_TYPE,
    header::HeaderValue::from_str("application/json")?,
  );
  let json_part = Part::text(meta.to_string()).headers(json_part_headers);

  // according to the file extension, get the mime
  let ext = filename
    .split('.')
    .last()
    .ok_or_else(|| WeChatPayError::Unknown("Invalid filename, no extension found".to_string()))?;
  let mime = match ext {
    "jpg" | "jpeg" => Ok("image/jpeg"),
    "png" => Ok("image/png"),
    "bmp" => Ok("image/bmp"),
    _ => Err(WeChatPayError::Unknown(
      "Unsupported image format".to_string(),
    )),
  };

  let file_part = Part::bytes(image)
    .file_name(filename.to_string())
    .mime_str(mime?)?;

  let form = Form::new().part("meta", json_part).part("file", file_part);
  Ok(form)
}

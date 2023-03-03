//! # [获取平台证书](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/wechatpay5_1.shtml)
//! 获取商户当前可用的平台证书列表。微信支付提供该接口，帮助商户后台系统实现平台证书的平滑更换。该请求无需身份认证信息之外的其他参数。
//!
//! 请求 URL: <https://api.mch.weixin.qq.com/v3/certificates>
//!
//! 请求方式: GET
use crate::sdk::common::EmptyRequest;
use crate::{Client, WeChatPayError};
use reqwest::Method;
use serde::Deserialize;

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

/// # [获取平台证书](self) 响应
#[derive(Debug, Deserialize)]
pub struct GetCertificatesResponse {
  pub data: Vec<CertificateData>,
}

impl Client {
  pub async fn get_certificates(&self) -> Result<GetCertificatesResponse, WeChatPayError> {
    Ok(
      self
        .send_request::<EmptyRequest, _>(Method::GET, "/v3/certificates", None, None)
        .await?
        .unwrap(),
    )
  }
}

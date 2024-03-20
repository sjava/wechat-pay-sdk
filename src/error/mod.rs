//! # 错误处理
mod code;
mod external;

use crate::Client;
pub use code::WeChatPayApiErrorCode;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WeChatPayApiErrorDetailValue {
  String(String),
  Array(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct WeChatPayApiErrorDetail {
  /// 指示错误参数的位置。当错误参数位于请求 body 的 JSON 时，填写指向参数的 JSON Pointer。当错误参数位于请求的 url 或者 query string 时，填写参数的变量名。
  pub field: Option<String>,
  /// 错误的值
  pub value: Option<WeChatPayApiErrorDetailValue>,
  /// 具体错误原因
  pub issue: Option<String>,
  pub location: Option<String>,
}

/// # [错误码和错误提示](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay2_0.shtml#part-7)
/// 当请求处理失败时，除了 HTTP 状态码表示错误之外，API 将在消息体返回错误相应说明具体的错误原因。
/// # Example
/// ```json
/// {
///   "code": "PARAM_ERROR",
///   "message": "参数错误",
///   "detail": {
///     "field": "/amount/currency",
///     "value": "XYZ",
///     "issue": "Currency code is invalid",
///     "location" :"body"
///   }
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct WeChatPayApiError {
  /// 详细错误码
  pub code: WeChatPayApiErrorCode,
  /// 错误描述，使用易理解的文字表示错误的原因。
  pub message: Option<String>,
  pub detail: Option<WeChatPayApiErrorDetail>,
}

#[derive(Debug)]
pub enum WeChatPayError {
  NetworkError(reqwest::Error),
  RedisError(redis::RedisError),
  /// 加密错误，通常是由于配置错误
  CryptoError(String),
  WeChatApiError(Box<WeChatPayApiError>),
  /// 服务器已接受请求，但尚未处理，一般的解决方案是使用原参数重复请求一遍
  Accepted,
  /// 通常应该是 sdk 实现有不当之处
  Unknown(String),
  /// sdk 认为是服务器自身错误，但也有可能是 sdk 的实现错误
  InternalServerError(String),
}

impl Client {
  #[inline]
  fn parse_json<Response>(text: &str) -> Result<Response, WeChatPayError>
  where
    Response: DeserializeOwned + Send + 'static,
  {
    serde_json::from_str::<Response>(text).map_err(|_| {
      serde_json::from_str::<WeChatPayApiError>(text)
        .map(|err| WeChatPayError::WeChatApiError(Box::new(err)))
        .unwrap_or_else(|_| WeChatPayError::Unknown(format!("Failed to parse response: {}", text)))
    })
  }

  /// # [错误信息](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay2_0.shtml#part-6)
  /// 微信支付 API v3 使用 HTTP 状态码来表示请求处理的结果。
  /// - 处理成功的请求，如果有应答的消息体将返回 200，若没有应答的消息体将返回 204。
  /// - 已经被成功接受待处理的请求，将返回 202。
  /// - 请求处理失败时，如缺少必要的入参、支付时余额不足，将会返回 4xx 范围内的错误码。
  /// - 请求处理时发生了微信支付侧的服务系统错误，将返回 500 / 501 / 503 的状态码。这种情况比较少见。
  pub(crate) async fn parse_response<Response>(
    status: StatusCode,
    text: String,
  ) -> Result<Option<Response>, WeChatPayError>
  where
    Response: DeserializeOwned + Send + 'static,
  {
    match status.as_u16() {
      200 => Ok(Some(Self::parse_json::<Response>(&text)?)),
      204 => Ok(None),
      202 => Err(WeChatPayError::Accepted),
      400..=501 | 503 => Err(WeChatPayError::WeChatApiError(Box::new(
        Self::parse_json::<WeChatPayApiError>(&text)?,
      ))),
      _ => Err(WeChatPayError::Unknown(
        "Unknown response status code".to_string(),
      )),
    }
  }
}

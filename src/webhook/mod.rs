//! # 回调报文
//! ## [报文解密](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_2.shtml)
//! 为了保证安全性，微信支付在回调通知和平台证书下载接口中，对关键信息进行了 AES-256-GCM 加密。本章节详细介绍了加密报文的格式，以及如何进行解密。
//! ### 加密报文格式
//! AES-GCM 是一种 NIST 标准的[认证加密](https://zh.wikipedia.org/wiki/%E8%AE%A4%E8%AF%81%E5%8A%A0%E5%AF%86)算法，是一种能够同时保证数据的保密性、完整性和真实性的一种加密模式。它最广泛的应用是在 TLS 中。
//!
//! 证书和回调报文使用的加密密钥为 [APIv3 密钥](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay3_2.shtml)。
//!
//! 对于加密的数据，我们使用了一个独立的 JSON 对象来表示。为了方便阅读，示例做了 Pretty 格式化，并加入了注释。
//!
//! > 加密的随机串，跟签名时使用的随机串没有任何关系，是不一样的。
//! ```json
//! {
//!   // 加密前的对象类型
//!   "original_type": "transaction",
//!   // 加密算法
//!   "algorithm": "AEAD_AES_256_GCM",
//!   // Base64编码后的密文
//!   "ciphertext": "...",
//!   // 加密使用的随机串初始化向量）
//!   "nonce": "...",
//!   // 附加数据包（可能为空）
//!   "associated_data": ""
//! }
//! ```
//! ### 解密
//! 算法接口的细节，可以参考 [RFC 5116](https://datatracker.ietf.org/doc/html/rfc5116)。
//!
//! 大部分编程语言（较新版本）都支持了 AEAD_AES_256_GCM。开发者可以参考下列的示例，了解如何使用您的编程语言实现解密。
//!
//! ## 报文格式
//! 在官方文档中没有给出报文的统一格式，但根据 API 字典内容，回调报文的格式应该如 [WeChatWebhook] 所示。
//! ```no_run
//! use serde::Deserialize;
//! #[derive(Deserialize, Debug)]
//! pub struct WeChatWebhook<Resource: Deserialize + std::fmt::Debug> {
//!   pub id: String,
//!   pub create_time: String,
//!   pub event_type: String,
//!   pub resource_type: String,
//!   pub resource: Resource,
//!   pub summary: String,
//! }
//! ```
pub mod transaction;

use crate::{Client, WeChatPayError};
use base64::{engine::general_purpose, Engine};
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Resource {
  /// 加密算法类型
  ///
  /// 对开启结果数据进行加密的加密算法，目前只支持 AEAD_AES_256_GCM
  ///
  /// 示例值：AEAD_AES_256_GCM
  pub algorithm: String,
  /// 数据密文
  ///
  /// Base64编码后的开启 / 停用结果数据密文
  ///
  /// sadsadsadsad
  pub ciphertext: String,
  /// 附加数据
  ///
  /// 示例值：fdasfwqewlkja484w
  pub associated_data: Option<String>,
  /// 原始类型
  ///
  /// 原始回调类型，为transaction（对于支付结果通知）
  pub original_type: String,
  /// 随机串
  ///
  /// 加密使用的随机串
  ///
  /// 示例值：fdasflkja484w
  pub nonce: String,
}

/// # 回调报文
/// 在文档的不同位置都有对回调报文的描述，例如：
/// - [JSApi 支付 > 支付结果通知](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_5.shtml)
/// - [JSApi 支付 > 退款结果通知](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_11.shtml)
///
/// 尽管格式基本相同，但对字段的描述有所区别，这里使用第一个文档中的具体描述。
/// ## 参数解密
/// 下面详细描述对通知数据进行解密的流程：
///
/// 1. 用商户平台上设置的 APIv3 密钥【[微信商户平台](https://pay.weixin.qq.com/) —> 账户设置 —> API 安全 —> 设置 APIv3 密钥】，记为key；
/// 2. 针对 resource.algorithm 中描述的算法（目前为 AEAD_AES_256_GCM），取得对应的参数 nonce 和 associated_data；
/// 3. 使用 key、nonce 和 associated_data，对数据密文 resource.ciphertext 进行解密，得到 JSON 形式的资源对象；
///
/// 注： AEAD_AES_256_GCM 算法的接口细节，请参考 [rfc5116](https://datatracker.ietf.org/doc/html/rfc5116)。微信支付使用的密钥
/// key 长度为 32 个字节，随机串 nonce 长度 12 个字节，associated_data 长度小于 16 个字节并可能为空字符串。
///
/// ## 通知签名
/// 加密不能保证通知请求来自微信。微信会对发送给商户的通知进行签名，并将签名值放在通知的 HTTP 头 Wechatpay-Signature。商户应当验证签名，以确认请求来自微信，而不是其他的第三方。签名验证的算法请参考
/// [《微信支付API v3签名验证》](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)。
/// ## 回调示例
/// ### 支付成功结果通知
/// ```json
/// {
///     "id": "EV-2018022511223320873",
///     "create_time": "2015-05-20T13:29:35+08:00",
///     "resource_type": "encrypt-resource",
///     "event_type": "TRANSACTION.SUCCESS",
///     "summary": "支付成功",
///     "resource": {
///         "original_type": "transaction",
///         "algorithm": "AEAD_AES_256_GCM",
///         "ciphertext": "",
///         "associated_data": "",
///         "nonce": ""
///     }
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct WeChatWebhook {
  /// 通知 ID
  ///
  /// 通知的唯一ID
  ///
  /// 示例值：EV-2018022511223320873
  pub id: String,
  /// 通知创建时间
  ///
  /// 通知创建的时间，遵循 [rfc3339](https://datatracker.ietf.org/doc/html/rfc3339) 标准格式，格式为
  /// yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD 表示年月日，T 出现在字符串中，表示 time 元素的开头，HH:mm:ss
  /// 表示时分秒，TIMEZONE 表示时区（+08:00 表示东八区时间，领先 UTC 8 小时，即北京时间）。例如：2015-05-20T13:29:35+08:00
  /// 表示北京时间 2015 年 05 月 20 日 13 点 29 分 35 秒。
  ///
  /// 示例值：2015-05-20T13:29:35+08:00
  pub create_time: String,
  /// 通知类型
  ///
  /// 通知的类型，支付成功通知的类型为 TRANSACTION.SUCCESS
  ///
  /// 示例值：TRANSACTION.SUCCESS
  pub event_type: String,
  /// 通知数据类型
  ///
  /// 通知的资源数据类型，支付成功通知为 encrypt-resource
  ///
  /// 示例值：encrypt-resource
  pub resource_type: String,
  /// 通知数据
  ///
  /// 通知资源数据
  ///
  /// json格式，见示例
  pub resource: Resource,
  /// 回调摘要
  ///
  /// 示例值：支付成功
  pub summary: String,
}

impl WeChatWebhook {
  pub fn parse<Message: DeserializeOwned>(&self, cli: &Client) -> Result<Message, WeChatPayError> {
    let plaintext = cli.aead_aes_256_gcm_decrypt(
      self.resource.nonce.as_bytes(),
      general_purpose::STANDARD
        .decode(&self.resource.ciphertext)?
        .as_slice(),
      match &self.resource.associated_data {
        Some(associated_data) => Some(associated_data.as_bytes()),
        None => None,
      },
    )?;
    let plaintext = std::str::from_utf8(plaintext.as_slice())?;
    Ok(serde_json::from_str::<Message>(plaintext)?)
  }
}

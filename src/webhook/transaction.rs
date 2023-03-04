//! # [支付通知 API](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_5.shtml)
//! 最新更新时间：2020.05.26
//!
//! 微信支付通过支付通知接口将用户支付成功消息通知给商户
//!
//! > **注意：**
//! >
//! > · 同样的通知可能会多次发送给商户系统。商户系统必须能够正确处理重复的通知。推荐的做法是，当商户系统收到通知进行处理时，先检查对应业务数据的状态，并判断该通知是否已经处理。如果未处理，则再进行处理；如果已处理，则直接返回结果成功。在对业务数据进行状态检查和处理之前，要采用数据锁进行并发控制，以避免函数重入造成的数据混乱。
//! >
//! > · 如果在所有通知频率后没有收到微信侧回调，商户应调用查询订单接口确认订单状态。
//! >
//! > **特别提醒：** 商户系统对于开启结果通知的内容一定要做签名验证，并校验通知的信息是否与商户侧的信息一致，防止数据泄露导致出现“假通知”，造成资金损失。
use serde::Deserialize;
use crate::sdk::common::{TransactionAmount, Payer, TransactionScene, TransactionPromotion};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeType {
  /// 公众号支付
  JSApi,
  /// 扫码支付
  Native,
  /// APP 支付
  App,
  /// 付款码支付
  MicroPay,
  /// H5 支付
  MWeb,
  /// 刷脸支付
  FacePay,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeStatus {
  /// 支付成功
  Success,
  /// 转入退款
  Refund,
  /// 已关闭
  Closed,
  /// 已撤销（付款码支付）
  Revoked,
  /// 用户支付中（付款码支付）
  UserPaying,
  /// 支付失败(其他原因，如银行返回失败)
  PayError,
}

/// ## 接口说明
/// 适用对象：`直连商户`
///
/// 请求方式：POST
///
/// 回调 URL：该链接是通过基础下单接口中的请求参数 `notify_url` 来设置的，要求必须为 https 地址。请确保回调 URL
/// 是外部可正常访问的，且不能携带后缀参数，否则可能导致商户无法接收到微信的回调通知信息。回调 URL 示例：
/// `https://pay.weixin.qq.com/wxpay/pay.action`
/// ## 通知规则
/// 用户支付完成后，微信会把相关支付结果和用户信息发送给商户，商户需要接收处理该消息，并返回应答。
/// 
/// 对后台通知交互时，如果微信收到商户的应答不符合规范或超时，微信认为通知失败，微信会通过一定的策略定期重新发起通知，尽可能提高通知的成功率，但微信不保证通知最终能成功。（通知频率为15s/15s/30s/3m/10m/20m/30m/30m/30m/60m/3h/3h/3h/6h/6h - 总计 24h4m）
/// ## 通知报文
/// 支付结果通知是以 POST 方法访问商户设置的通知 url，通知的数据以 JSON 格式通过请求主体（BODY）传输。通知的数据包括了加密的支付结果详情。
///
/// （注：由于涉及到回调加密和解密，商户必须先设置好 apiv3 秘钥后才能解密回调通知，apiv3 秘钥设置文档指引详见
/// [APIv3秘钥设置指引](https://kf.qq.com/faq/180830E36vyQ180830AZFZvu.html)）
#[derive(Deserialize, Debug)]
pub struct TransactionSuccess {
  pub appid: String,
  pub mchid: String,
  pub out_trade_no: String,
  pub transaction_id: String,
  pub trade_type: TradeType,
  pub trade_state: TradeStatus,
  pub trade_state_desc: String,
  pub attach: Option<String>,
  pub success_time: String,
  pub payer: Payer,
  pub amount: TransactionAmount,
  pub scene_info: Option<TransactionScene>,
  pub promotion_detail: Option<Vec<TransactionPromotion>>,
}
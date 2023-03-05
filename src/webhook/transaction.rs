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
use crate::sdk::common::{Payer, TransactionAmount, TransactionPromotion, TransactionScene};
use serde::Deserialize;

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
/// 对后台通知交互时，如果微信收到商户的应答不符合规范或超时，微信认为通知失败，微信会通过一定的策略定期重新发起通知，尽可能提高通知的成功率，但微信不保证通知最终能成功。（通知频率为
/// 15s / 15s / 30s / 3m / 10m / 20m / 30m / 30m / 30m / 60m / 3h / 3h / 3h / 6h / 6h - 总计 24h4m）
#[derive(Deserialize, Debug)]
pub struct TransactionSuccess {
  /// 应用ID
  ///
  /// 直连商户申请的公众号或移动应用appid。
  ///
  /// 示例值：wxd678efh567hg6787
  pub appid: String,
  /// 商户号
  ///
  /// 商户的商户号，由微信支付生成并下发。
  ///
  /// 示例值：1230000109
  pub mchid: String,
  /// 微信支付订单号
  ///
  /// 微信支付系统生成的订单号。
  ///
  /// 示例值：1217752501201407033233368018
  pub out_trade_no: String,
  /// 交易类型
  ///
  /// 示例值：MICROPAY
  pub transaction_id: String,
  /// 交易状态
  ///
  /// 示例值：SUCCESS
  pub trade_type: TradeType,
  /// 交易状态描述
  ///
  /// 交易状态描述
  ///
  /// 示例值：支付成功
  pub trade_state: TradeStatus,
  /// 付款银行
  ///
  /// 银行类型，采用字符串类型的银行标识。银行标识请参考《[银行类型对照表](https://pay.weixin.qq.com/wiki/doc/apiv3/terms_definition/chapter1_1_3.shtml#part-6)》
  ///
  /// 示例值：CICBC_DEBIT
  pub trade_state_desc: String,
  /// 附加数据
  ///
  /// 附加数据，在查询 API 和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
  ///
  /// 示例值：自定义数据
  pub attach: Option<String>,
  /// 支付完成时间
  ///
  /// 支付完成时间，遵循 [rfc3339](https://datatracker.ietf.org/doc/html/rfc3339) 标准格式，格式为
  /// yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD 表示年月日，T 出现在字符串中，表示 time 元素的开头，HH:mm:ss
  /// 表示时分秒，TIMEZONE 表示时区（+08:00 表示东八区时间，领先 UTC 8 小时，即北京时间）。例如：2015-05-20T13:29:35+08:00
  /// 表示北京时间 2015 年 05 月 20 日 13 点 29 分 35 秒。
  ///
  /// 示例值：2018-06-08T10:34:56+08:00
  pub success_time: String,
  /// 支付者
  ///
  /// 支付者信息
  pub payer: Payer,
  /// 订单金额
  ///
  /// 订单金额信息
  pub amount: TransactionAmount,
  /// 场景信息
  ///
  /// 支付场景信息描述
  pub scene_info: Option<TransactionScene>,
  /// 优惠功能
  ///
  /// 优惠功能，享受优惠时返回该字段。
  pub promotion_detail: Option<Vec<TransactionPromotion>>,
}

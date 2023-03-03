use super::{Amount, Discount, Payer, Scene, Settle};
use serde::Serialize;

#[derive(Serialize)]
pub struct OrderRequest {
  /// 应用 ID
  ///
  /// 由微信生成的应用 ID，全局唯一。请求基础下单接口时请注意 APPID 的应用属性，例如公众号场景下，需使用应用属性为公众号的服务号 APPID
  ///
  /// 示例值：wxd678efh567hg6787
  pub appid: String,
  /// 直连商户号
  ///
  /// 直连商户的商户号，由微信支付生成并下发。
  ///
  /// 示例值：1230000109
  pub mchid: String,
  /// 商品描述
  ///
  /// 示例值：Image形象店-深圳腾大-QQ公仔
  pub description: String,
  /// 商户订单号
  ///
  /// 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一
  ///
  /// 示例值：1217752501201407033233368018
  pub out_trade_no: String,
  /// 交易结束时间
  ///
  /// 单失效时间，遵循 rfc3339 标准格式，格式为 yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD 表示年月日，T 出现在字符串中，表示 time 元素的开头，HH:mm:ss 表示时分秒，TIMEZONE 表示时区（+08:00 表示东八区时间，领先 UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00 表示，北京时间 2015 年 5 月 20 日 13 点 29 分 35 秒。
  ///
  /// 示例值：2018-06-08T10:34:56+08:00
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_expire: Option<String>,
  /// 附加数据
  ///
  /// 附加数据，在查询 API 和支付通知中原样返回，可作为自定义参数使用，实际情况下只有支付完成状态才会返回该字段。
  ///
  /// 示例值：自定义数据
  #[serde(skip_serializing_if = "Option::is_none")]
  pub attach: Option<String>,
  /// 通知地址
  ///
  /// 异步接收微信支付结果通知的回调地址，通知 url 必须为外网可访问的 url，不能携带参数。公网域名必须为 https，如果是走专线接入，使用专线 NAT IP 或者私有回调域名可使用 http
  ///
  /// 示例值：https://www.weixin.qq.com/wxpay/pay.php
  pub notify_url: String,
  /// 订单优惠标记
  ///
  /// 示例值：WXG
  #[serde(skip_serializing_if = "Option::is_none")]
  pub goods_tag: Option<String>,
  /// 电子发票入口开放标识
  ///
  /// 传入 true 时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
  ///
  /// true：是
  ///
  /// false：否
  ///
  /// 示例值：true
  #[serde(skip_serializing_if = "Option::is_none")]
  pub support_fapiao: Option<bool>,
  /// 订单金额信息
  pub amount: Amount,
  /// 支付者信息
  pub payer: Payer,
  /// 优惠功能
  #[serde(skip_serializing_if = "Option::is_none")]
  pub detail: Option<Discount>,
  /// 支付场景描述
  #[serde(skip_serializing_if = "Option::is_none")]
  pub scene_info: Option<Scene>,
  /// 结算信息
  #[serde(skip_serializing_if = "Option::is_none")]
  pub settle_info: Option<Settle>,
}

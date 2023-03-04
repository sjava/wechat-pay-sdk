use serde::{Deserialize, Serialize};

/// 商品信息
#[derive(Serialize)]
pub struct GoodInfo {
  /// 商户侧商品编码
  ///
  /// 由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
  ///
  /// 示例值：1246464644
  pub merchant_goods_id: String,
  /// 微信支付商品编码
  ///
  /// 微信支付定义的统一商品编号（没有可不传）
  ///
  /// 示例值：1001
  pub wechatpay_goods_id: Option<String>,
  /// 商品名称
  ///
  /// 商品的实际名称
  ///
  /// 示例值：iPhoneX 256G
  pub goods_name: Option<String>,
  /// 商品数量
  ///
  /// 用户购买的数量
  ///
  /// 示例值：1
  pub quantity: i64,
  /// 商品单价
  ///
  /// 单位为：分。如果商户有优惠，需传输商户优惠后的单价（例如：用户对一笔 100 元的订单使用了商场发的纸质优惠券 100-50，
  /// 则活动商品的单价应为原单价-50）
  ///
  /// 示例值：528800
  pub unit_price: i64,
}

/// 优惠信息
#[derive(Serialize)]
pub struct Discount {
  /// 订单原价
  ///
  /// 1. 商户侧一张小票订单可能被分多次支付，订单原价用于记录整张小票的交易金额。
  ///
  /// 2. 当订单原价与支付金额不相等，则不享受优惠。
  ///
  /// 3. 该字段主要用于防止同一张小票分多次支付，以享受多次优惠的情况，正常支付订单不必上传此参数。
  ///
  /// 示例值：608800
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cost_price: Option<i64>,
  /// 商家小票 ID
  ///
  /// 示例值：微信123
  #[serde(skip_serializing_if = "Option::is_none")]
  pub invoice_id: Option<String>,
  /// 单品列表
  ///
  /// 单品列表信息
  ///
  /// 条目个数限制：【1，6000】
  #[serde(skip_serializing_if = "Option::is_none")]
  pub goods_detail: Option<Vec<GoodInfo>>,
}

/// 商品信息
#[derive(Deserialize, Debug)]
pub struct TransactionGoodInfo {
  /// 商品编码
  ///
  /// 示例值：M1006
  pub goods_id: String,
  /// 商品数量
  ///
  /// 用户购买的数量
  ///
  /// 示例值：1
  pub quantity: i32,
  /// 商品单价
  ///
  /// 商品单价，单位为分
  ///
  /// 示例值：100
  pub unit_price: i32,
  /// 商品优惠金额
  ///
  /// 商品优惠金额
  ///
  /// 示例值：0
  pub discount_amount: i32,
  /// 商品备注
  ///
  /// 商品备注信息
  ///
  /// 示例值：商品备注信息
  pub goods_remark: Option<String>,
}

/// 优惠功能
#[derive(Deserialize, Debug)]
pub struct TransactionPromotion {
  /// 券 ID
  ///
  /// 示例值：109519
  pub coupon_id: String,
  /// 优惠名称
  ///
  /// 示例值：单品惠-6
  pub name: Option<String>,
  /// 优惠范围
  ///
  /// GLOBAL：全场代金券
  ///
  /// SINGLE：单品优惠
  ///
  /// 示例值：GLOBAL
  pub scope: Option<String>,
  /// 优惠类型
  ///
  /// CASH：充值型代金券
  ///
  /// NOCASH：免充值型代金券
  ///
  /// 示例值：CASH
  #[serde(rename = "type")]
  pub type_: Option<String>,
  /// 优惠券面额
  ///
  /// 示例值：100
  pub amount: i32,
  /// 活动ID
  ///
  /// 示例值：931386
  pub stock_id: Option<String>,
  /// 微信出资
  ///
  /// 微信出资，单位为分
  ///
  /// 示例值：0
  pub wechatpay_contribute: Option<i32>,
  /// 商户出资
  ///
  /// 商户出资，单位为分
  ///
  /// 示例值：0
  pub merchant_contribute: Option<i32>,
  /// 其他出资
  ///
  /// 其他出资，单位为分
  ///
  /// 示例值：0
  pub other_contribute: Option<i32>,
  /// 优惠币种
  ///
  /// CNY：人民币，境内商户号仅支持人民币。
  ///
  /// 示例值：CNY
  pub currency: Option<String>,
  /// 单品列表
  ///
  /// 单品列表信息
  pub goods_detail: Option<Vec<TransactionGoodInfo>>,
}

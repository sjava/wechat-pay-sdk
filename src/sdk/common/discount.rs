use super::{GoodInfo, RefundGoodsDetail, TransactionGoodInfo};
use serde::{Deserialize, Serialize};

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

/// 优惠功能
#[derive(Deserialize, Debug)]
pub struct RefundPromotion {
  /// 券 ID
  ///
  /// 示例值：109519
  pub coupon_id: String,
  /// 优惠范围
  ///
  /// GLOBAL：全场代金券
  ///
  /// SINGLE：单品优惠
  ///
  /// 示例值：SINGLE
  pub scope: Option<String>,
  /// 优惠类型
  ///
  /// COUPON：代金券，需要走结算资金的充值型代金券
  ///
  /// DISCOUNT：优惠券，不走结算资金的免充值型优惠券
  ///
  /// 示例值：DISCOUNT
  #[serde(rename = "type")]
  pub type_: String,
  /// 优惠券面额
  ///
  /// 用户享受优惠的金额（优惠券面额=微信出资金额+商家出资金额+其他出资方金额），单位为分
  ///
  /// 示例值：5
  pub amount: i32,
  /// 优惠退款金额
  ///
  /// 优惠退款金额<=退款金额，退款金额-代金券或立减优惠退款金额为用户支付的现金，说明详见代金券或立减优惠，单位为分
  ///
  /// 示例值：100
  pub refund_amount: i32,
  /// 单品列表
  ///
  /// 单品列表信息
  pub goods_detail: Option<Vec<RefundGoodsDetail>>,
}

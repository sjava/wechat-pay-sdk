use serde::{Deserialize, Serialize};

/// 订单金额信息
#[derive(Serialize)]
pub struct Amount {
  /// 订单总金额，单位为分。
  ///
  /// 示例值：100
  pub total: i32,
  /// 货币类型
  ///
  /// CNY：人民币，境内商户号仅支持人民币。
  ///
  /// 示例值：CNY
  #[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<String>,
}

/// 订单金额信息
#[derive(Deserialize, Debug)]
pub struct TransactionAmount {
  /// 订单总金额，单位为分。
  ///
  /// 示例值：100
  pub total: i32,
  /// 用户支付金额，单位为分。
  ///
  /// 示例值：100
  pub payer_total: i32,
  /// 货币类型
  ///
  /// CNY：人民币，境内商户号仅支持人民币。
  ///
  /// 示例值：CNY
  pub currency: String,
  /// 用户支付币种
  ///
  /// 示例值：CNY
  pub payer_currency: String,
}

/// 退款出资账户及金额 from array 否退款需要从指定账户出资时，传递此参数指定出资金额（币种的最小单位，只能为整数）。
#[derive(Deserialize, Serialize, Debug)]
pub struct RefundFrom {
  /// 出资账户类型
  ///
  /// 下面枚举值多选一。
  ///
  /// 枚举值：
  /// - AVAILABLE: 可用余额
  /// - UNAVAILABLE: 不可用余额
  ///
  /// 示例值：AVAILABLE
  pub account: String,
  /// 出资金额
  ///
  /// 对应账户出资金额
  ///
  /// 示例值：444
  pub amount: i32,
}

/// 退款金额信息
#[derive(Serialize, Debug)]
pub struct RefundAmount {
  /// 退款金额
  ///
  /// 退款金额，单位为分，只能为整数，不能超过原订单支付金额。
  ///
  /// 示例值：888
  pub refund: i32,
  /// 退款需要从指定账户出资时，传递此参数指定出资金额（币种的最小单位，只能为整数）。
  ///
  /// 同时指定多个账户出资退款的使用场景需要满足以下条件：
  /// 1. 未开通退款支出分离产品功能；
  /// 2. 订单属于分账订单，且分账处于待分账或分账中状态。
  ///
  /// 参数传递需要满足条件：
  /// 1. 基本账户可用余额出资金额与基本账户不可用余额出资金额之和等于退款金额；
  /// 2. 账户类型不能重复。
  ///
  /// 上述任一条件不满足将返回错误
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Vec<RefundFrom>>,
  /// 原订单金额
  ///
  /// 原支付交易的订单总金额，单位为分，只能为整数。
  ///
  /// 示例值：888
  pub total: i32,
  /// 退款币种
  ///
  /// 符合 ISO 4217 标准的三位字母代码，目前只支持人民币：CNY。
  ///
  /// 示例值：CNY
  pub currency: String,
}

/// 退款金额信息（响应）
#[derive(Deserialize, Debug)]
pub struct RefundAmountResponse {
  /// 订单金额
  ///
  /// 订单总金额，单位为分
  ///
  /// 示例值：100
  pub total: i32,
  /// 退款金额
  ///
  /// 退款标价金额，单位为分，可以做部分退款
  ///
  /// 示例值：100
  pub refund: i32,
  /// 退款出资账户及金额
  ///
  /// 退款出资的账户类型及金额信息
  pub from: Option<Vec<RefundFrom>>,
  /// 用户支付金额
  ///
  /// 现金支付金额，单位为分，只能为整数
  ///
  /// 示例值：90
  pub payer_total: i32,
  /// 用户退款金额
  ///
  /// 退款给用户的金额，不包含所有优惠券金额
  ///
  /// 示例值：90
  pub payer_refund: i32,
  /// 应结退款金额
  ///
  /// 去掉非充值代金券退款金额后的退款金额，单位为分，退款金额=申请退款金额-非充值代金券退款金额，退款金额<=申请退款金额
  ///
  /// 示例值：100
  pub settlement_refund: i32,
  /// 应结订单金额
  ///
  /// 应结订单金额=订单金额-免充值代金券金额，应结订单金额<=订单金额，单位为分
  ///
  /// 示例值：100
  pub settlement_total: i32,
  /// 优惠退款金额
  ///
  /// 优惠退款金额<=退款金额，退款金额-代金券或立减优惠退款金额为现金，说明详见代金券或立减优惠，单位为分
  ///
  /// 示例值：10
  pub discount_refund: i32,
  /// 退款币种
  ///
  /// 符合 ISO 4217 标准的三位字母代码，目前只支持人民币：CNY。
  ///
  /// 示例值：CNY
  pub currency: String,
  /// 手续费退款金额
  ///
  /// 手续费退款金额，单位为分。
  ///
  /// 示例值：10
  pub refund_fee: Option<i32>,
}

/// 退款金额信息
#[derive(Deserialize, Debug)]
pub struct RefundAmountWebHook {
  /// 订单金额
  ///
  /// 订单总金额，单位为分，只能为整数，详见支付金额
  ///
  /// 示例值：999
  pub total: i32,
  /// 退款金额
  ///
  /// 退款金额，币种的最小单位，只能为整数，不能超过原订单支付金额，如果有使用券，后台会按比例退。
  ///
  /// 示例值：999
  pub refund: i32,
  /// 用户支付金额
  ///
  /// 用户实际支付金额，单位为分，只能为整数，详见支付金额
  ///
  /// 示例值：999
  pub payer_total: i32,
  /// 用户退款金额
  ///
  /// 退款给用户的金额，不包含所有优惠券金额
  ///
  /// 示例值：999
  pub payer_refund: i32,
}

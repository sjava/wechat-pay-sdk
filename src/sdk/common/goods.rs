use serde::{Deserialize, Serialize};

/// 商品信息
#[derive(Serialize, Debug)]
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

/// 退款商品
#[derive(Deserialize, Serialize, Debug)]
pub struct RefundGoodsDetail {
  /// 商户侧商品编码
  ///
  /// 由半角的大小写字母、数字、中划线、下划线中的一种或几种组成
  ///
  /// 示例值：1217752501201407033233368018
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
  /// 示例值：iPhone6s 16G
  pub goods_name: Option<String>,
  /// 商品单价
  ///
  /// 商品单价金额，单位为分
  ///
  /// 示例值：528800
  pub unit_price: i32,
  /// 商品退款金额
  ///
  /// 商品退款金额，单位为分
  ///
  /// 示例值：528800
  pub refund_amount: i32,
  /// 商品退货数量
  ///
  /// 单品的退款数量
  ///
  /// 示例值：1
  pub refund_quantity: i32,
}

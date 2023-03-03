use serde::Serialize;

/*

商户侧商品编码	merchant_goods_id	string[1,32]	是	由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
示例值：1246464644
微信支付商品编码	wechatpay_goods_id	string[1,32]	否	微信支付定义的统一商品编号（没有可不传）
示例值：1001
商品名称	goods_name	string[1,256]	否	商品的实际名称
示例值：iPhoneX 256G
商品数量	quantity	int	是	用户购买的数量
示例值：1
商品单价	unit_price	int	是	单位为：分。如果商户有优惠，需传输商户优惠后的单价(例如：用户对一笔100元的订单使用了商场发的纸质优惠券100-50，则活动商品的单价应为原单价-50)
示例值：528800
*/

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

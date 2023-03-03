use serde::Serialize;

/// 支付者信息
#[derive(Serialize)]
pub struct Payer {
  /// 用户标识
  ///
  /// 用户在直连商户 appid 下的唯一标识。下单前需获取到用户的 Openid，[Openid获取详见](https://pay.weixin.qq.com/wiki/doc/apiv3/terms_definition/chapter1_1_3.shtml#part-3)
  ///
  /// 示例值：oUpF8uMuAJO_M2pxb1Q9zNjWeS6o
  pub openid: String,
}

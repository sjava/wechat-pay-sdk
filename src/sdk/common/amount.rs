use serde::Serialize;

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

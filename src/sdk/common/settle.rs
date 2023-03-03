use serde::Serialize;

/// 结算信息
#[derive(Serialize)]
pub struct Settle {
  /// 是否指定分账
  ///
  /// 示例值：false
  pub profit_sharing: bool,
}

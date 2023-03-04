use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct StoreInfo {
  /// 门店编号
  ///
  /// 商户侧门店编号
  ///
  /// 示例值：0001
  pub id: String,
  /// 门店名称
  ///
  /// 商户侧门店名称
  ///
  /// 示例值：腾讯大厦分店
  pub name: Option<String>,
  /// 地区编码
  ///
  /// 地区编码，详细请见省市区编号对照表。
  ///
  /// 示例值：440305
  pub area_code: Option<String>,
  /// 详细地址
  ///
  /// 详细的商户门店地址
  ///
  /// 示例值：广东省深圳市南山区科技中一道 10000 号
  pub address: Option<String>,
}

/// 场景信息
#[derive(Serialize)]
pub struct Scene {
  /// 用户终端 IP
  ///
  /// 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
  ///
  /// 示例值：14.23.150.211
  pub payer_client_ip: String,
  /// 商户端设备号
  ///
  /// 商户端设备号（门店号或收银设备ID）。
  ///
  /// 示例值：013467007045764
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device_id: Option<String>,
  /// 商户门店信息
  #[serde(skip_serializing_if = "Option::is_none")]
  pub store_info: Option<StoreInfo>,
}

/// 场景信息
#[derive(Deserialize, Debug)]
pub struct TransactionScene {
  /// 商户端设备号
  ///
  /// 商户端设备号（门店号或收银设备ID）。
  ///
  /// 示例值：013467007045764
  pub device_id: Option<String>,
}

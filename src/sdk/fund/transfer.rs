//! # [发起商家转账](https://pay.weixin.qq.com/docs/merchant/apis/batch-transfer-to-balance/transfer-batch/initiate-batch-transfer.html)
//! 更新时间：2023.02.16
//!
//! 发起商家转账接口。商户可以通过该接口同时向多个用户微信零钱进行转账操作。请求消息中应包含商家批次单号、转账名称、appid、转账总金额、转账总笔数、转账
//! openid、收款用户姓名等信息。
//! ## 接口说明
//! 支持商户：普通商户
//!
//! 请求方式：【POST】 /v3/transfer/batches
//!
//! 请求域名：
//! - 【主域名】<https://api.mch.weixin.qq.com> 使用该域名将访问就近的接入点
//! - 【备域名】<https://api2.mch.weixin.qq.com> 使用该域名将访问异地的接入点 ，指引[点击查看](https://pay.weixin.qq.com/wiki/doc/apiv3/Practices/chapter1_1_4.shtml)
use crate::{Client, WeChatPayError};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TransferDetailInput {
  /// 商户系统内部区分转账批次单下不同转账明细单的唯一标识，要求此参数只能由数字、大小写字母组成\
  pub out_detail_no: String,
  /// 转账金额单位为“分”
  pub transfer_amount: i32,
  /// 单条转账备注（微信用户会收到该备注），UTF8 编码，最多允许 32 个字符
  pub transfer_remark: String,
  /// 商户appid下，某用户的openid
  pub openid: String,
  /// 收款方真实姓名。支持标准 RSA 算法和国密算法，公钥由微信侧提供
  ///
  /// 明细转账金额 < 0.3 元时，不允许填写收款用户姓名
  ///
  /// 明细转账金额 >= 2,000 元时，该笔明细必须填写收款用户姓名
  ///
  /// 同一批次转账明细中的姓名字段传入规则需保持一致，也即全部填写、或全部不填写
  ///
  /// 若商户传入收款用户姓名，微信支付会校验用户 openID 与姓名是否一致，并提供电子回单
  pub user_name: Option<String>,
}

#[derive(Serialize)]
pub struct BatchTransferRequest {
  /// 申请商户号的 appid 或商户号绑定的 appid（企业号 corpid 即为此 appid）
  pub appid: String,
  /// 商户系统内部的商家批次单号，要求此参数只能由数字、大小写字母组成，在商户系统内部唯一
  pub out_batch_no: String,
  /// 该笔批量转账的名称
  pub batch_name: String,
  /// 转账说明，UTF8 编码，最多允许 32 个字符
  pub batch_remark: String,
  /// 转账金额单位为“分”。转账总金额必须与批次内所有明细转账金额之和保持一致，否则无法发起转账操作
  pub total_amount: i32,
  /// 一个转账批次单最多发起一千笔转账。转账总笔数必须与批次内所有明细之和保持一致，否则无法发起转账操作
  pub total_num: i32,
  /// 发起批量转账的明细列表，最多一千笔
  pub transfer_detail_list: Vec<TransferDetailInput>,
  /// 必填，指定该笔转账使用的转账场景 ID
  pub transfer_scene_id: Option<String>,
}

#[derive(Deserialize)]
pub struct BatchTransferResponse {
  /// 商户系统内部的商家批次单号，在商户系统内部唯一
  pub out_batch_no: String,
  /// 微信批次单号，微信商家转账系统返回的唯一标识
  pub batch_id: String,
  /// 批次受理成功时返回，按照使用 rfc3339 所定义的格式，格式为 YYYY-MM-DDThh:mm:ss+TIMEZONE
  pub create_time: String,
}

impl Client {
  pub async fn batch_transfer(
    &self,
    req: &BatchTransferRequest,
  ) -> Result<BatchTransferResponse, WeChatPayError> {
    Ok(
      self
        .send_request(Method::POST, "/v3/transfer/batches", None, Some(req))
        .await?
        .unwrap(),
    )
  }
}

//! # [JSAPI 下单](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_1.shtml)
//! 最新更新时间：2022.09.05
//!
//! 商户系统先调用该接口在微信支付服务后台生成预支付交易单，返回正确的预支付交易会话标识后再按 Native、JSAPI、APP 等不同场景生成交易串调起支付。
//! ## 接口说明
//! 适用对象：直连商户
//!
//! 请求 URL：<https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi>
//!
//! 请求方式：POST
use crate::{sdk::common::OrderRequest, Client, WeChatPayError};
use reqwest::Method;
use serde::Deserialize;

pub type JSApiOrderRequest = OrderRequest;

#[derive(Debug, Deserialize)]
pub struct JSApiOrderResponse {
  /// 预支付交易会话标识
  ///
  /// 预支付交易会话标识。用于后续接口调用中使用，该值有效期为 2 小时
  ///
  /// 示例值：wx201410272009395522657a690389285100
  pub prepay_id: String,
}

impl Client {
  pub async fn jsapi_order(
    &self,
    req: &JSApiOrderRequest,
  ) -> Result<JSApiOrderResponse, WeChatPayError> {
    Ok(
      self
        .send_request(Method::POST, "/v3/pay/transactions/jsapi", None, Some(req))
        .await?
        .unwrap(),
    )
  }
}

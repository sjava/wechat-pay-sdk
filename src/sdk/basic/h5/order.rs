//! # [H5 下单 API](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_3_1.shtml)
//! 最新更新时间：2022.09.05
//!
//! 商户系统先调用该接口在微信支付服务后台生成预支付交易单，返回正确的预支付交易会话标识后再按 Native、JSAPI、APP 等不同场景生成交易串调起支付。
//! ## 接口说明
//! 适用对象：直连商户
//!
//! 请求 URL：<https://api.mch.weixin.qq.com/v3/pay/transactions/h5>
//!
//! 请求方式：POST
use crate::{sdk::common::OrderRequest, Client, WeChatPayError};
use reqwest::Method;
use serde::Deserialize;

/// # [H5 下单 API](self) 请求
/// # Example
/// ```json
/// {
///   "mchid": "1900006XXX",
///   "out_trade_no": "H51217752501201407033233368018",
///   "appid": "wxdace645e0bc2cXXX",
///   "description": "Image形象店-深圳腾大-QQ公仔",
///   "notify_url": "https://weixin.qq.com/",
///   "amount": {
///     "total": 1,
///     "currency": "CNY"
///   },
///   "scene_info": {
///     "payer_client_ip": "127.0.0.1",
///     "h5_info": {
///       "type": "Wap"
///     }
///   }
/// }
/// ```
pub type H5OrderRequest = OrderRequest;

/// # [H5 下单 API](self) 响应
/// # Example
/// ```json
/// {
///   "h5_url": "https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx2916263004719461949c84457c735b0000&package=2150917749"
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct H5OrderResponse {
  /// 支付跳转链接
  ///
  /// h5_url 为拉起微信支付收银台的中间页面，可通过访问该 url 来拉起微信客户端，完成支付，h5_url 的有效期为5分钟。
  ///
  /// 示例值：https://wx.tenpay.com/cgi-bin/mmpayweb-bin/checkmweb?prepay_id=wx2016121516420242444321ca0631331346&package=1405458241
  pub h5_url: String,
}

impl Client {
  pub async fn h5_order(&self, req: &H5OrderRequest) -> Result<H5OrderResponse, WeChatPayError> {
    Ok(
      self
        .send_request(Method::POST, "/v3/pay/transactions/h5", None, Some(req))
        .await?
        .unwrap(),
    )
  }
}

//! # 小程序支付
//! 通过好友分享或扫描二维码在微信内打开小程序时，可以调用微信支付完成下单购买的流程。
//! - [JSAPI 下单](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_5_1.shtml)
//!
//!   该接口与 [JSAPI 支付 > JSAPI 下单](order) 完全一致，因此只是重复导出。
//! - [申请退款](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_2_9.shtml)
//!
//!   该接口与 [JSAPI 支付 > 申请退款](refund) 完全一致，因此只是重复导出。
pub use super::jsapi::{order, refund};

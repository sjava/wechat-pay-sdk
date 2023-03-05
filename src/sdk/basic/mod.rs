//! # 基础支付
//! - [JSAPI 支付](jsapi)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter1_1_1.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_1.shtml)）
//! - [APP 支付](app)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter2_5_0.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_2_1.shtml)）
//! - [H5 支付](h5)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter2_6_0.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_3_1.shtml)）
//! - [Native 支付](native)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter2_7_0.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_4_1.shtml)）
//! - [小程序支付](mini_app)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter2_8_0.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_5_1.shtml)）
//! - [合单支付](combine)（[产品介绍](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter2_9_1.shtml) | [API 接口](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter5_1_1.shtml)）
//! - 付款码支付 V2
//!
//!   用户出示微信钱包中的条码、二维码，商家通过扫描用户条码即可完成收款。
//! - [刷脸支付](https://pay.weixin.qq.com/wiki/doc/wxfacepay/)
//!
//!   用户在集成微信刷脸支付SDK的线下机具上"刷脸"即可完成支付。
pub mod app;
pub mod combine;
pub mod h5;
pub mod jsapi;
pub mod mini_app;
pub mod native;

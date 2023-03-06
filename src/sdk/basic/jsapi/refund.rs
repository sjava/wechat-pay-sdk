//! # [申请退款](https://pay.weixin.qq.com/wiki/doc/apiv3/apis/chapter3_1_9.shtml)
//! 最新更新时间：2022.08.29
//!
//! 当交易发生之后一年内，由于买家或者卖家的原因需要退款时，卖家可以通过退款接口将支付金额退还给买家，微信支付将在收到退款请求并且验证成功之后，将支付款按原路退还至买家账号上。
//!
//! > 注意：
//! > 1. 交易时间超过一年的订单无法提交退款
//! > 2. 微信支付退款支持单笔交易分多次退款（不超 50 次），多次退款需要提交原支付订单的商户订单号和设置不同的退款单号。申请退款总金额不能超过订单金额。 一笔退款失败后重新提交，请不要更换退款单号，请使用原商户退款单号
//! > 3. 错误或无效请求频率限制：6 qps，即每秒钟异常或错误的退款申请请求不超过 6 次
//! > 4. 每个支付订单的部分退款次数不能超过 50 次
//! > 5. 如果同一个用户有多笔退款，建议分不同批次进行退款，避免并发退款导致退款失败
//! > 6. 申请退款接口的返回仅代表业务的受理情况，具体退款是否成功，需要通过退款查询接口获取结果
//! > 7. 一个月之前的订单申请退款频率限制为：5000 / min
//! > 8. 同一笔订单多次退款的请求需相隔 1 分钟
//! ## 接口说明
//! 适用对象：直连商户
//!
//! 请求 URL：<https://api.mch.weixin.qq.com/v3/refund/domestic/refunds>
//!
//! 请求方式：POST
//!
//! 接口频率：150 qps
use crate::{
  sdk::common::{RefundAmount, RefundAmountResponse, RefundGoodsDetail, RefundPromotion},
  Client, WeChatPayError,
};
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RefundRequest {
  /// 微信支付订单号
  ///
  /// 原支付交易对应的微信订单号，与商户订单号必填一个
  ///
  /// 示例值：1217752501201407033233368018
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transaction_id: Option<String>,
  /// 商户订单号
  ///
  /// 原支付交易对应的商户订单号，与微信支付订单号必填一个
  ///
  /// 示例值：1217752501201407033233368018
  #[serde(skip_serializing_if = "Option::is_none")]
  pub out_trade_no: Option<String>,
  /// 商户退款单号
  ///
  /// 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
  ///
  /// 示例值：1217752501201407033233368018
  pub out_refund_no: String,
  /// 退款原因
  ///
  /// 若商户传入，会在下发给用户的退款消息中体现退款原因
  ///
  /// 示例值：商品已售完
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reason: Option<String>,
  /// 退款结果回调 url
  ///
  /// 异步接收微信支付退款结果通知的回调地址，通知 url 必须为外网可访问的 url，不能携带参数。如果参数中传了 notify_url，则商户平台上配置的回调地址将不会生效，优先回调当前传的这个地址。
  ///
  /// 示例值：https://weixin.qq.com
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notify_url: Option<String>,
  /// 退款资金来源
  ///
  /// 若传递此参数则使用对应的资金账户退款，否则默认使用未结算资金退款（仅对老资金流商户适用）
  ///
  /// 枚举值：
  /// - AVAILABLE：可用余额账户
  ///
  /// 示例值：AVAILABLE
  #[serde(skip_serializing_if = "Option::is_none")]
  pub funds_account: Option<String>,
  /// 金额信息
  ///
  /// 订单金额信息
  pub amount: RefundAmount,
  /// 退款商品
  ///
  /// 指定商品退款需要传此参数，其他场景无需传递
  ///
  /// 注：实际操作中如果无需此参数，则需要传空数组，否则会返回错误
  pub goods_detail: Vec<RefundGoodsDetail>,
}

#[derive(Deserialize, Debug)]
pub struct RefundResponse {
  /// 微信支付退款单号
  ///
  /// 微信支付退款单号
  ///
  /// 示例值：50000000382019052709732678859
  pub refund_id: String,
  /// 商户退款单号
  ///
  /// 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
  ///
  /// 示例值：1217752501201407033233368018
  pub out_refund_no: String,
  /// 微信支付订单号
  ///
  /// 微信支付交易订单号
  ///
  /// 示例值：1217752501201407033233368018
  pub transaction_id: String,
  /// 商户订单号
  ///
  /// 原支付交易对应的商户订单号
  ///
  /// 示例值：1217752501201407033233368018
  pub out_trade_no: String,
  /// 退款渠道
  ///
  /// 枚举值：
  /// - ORIGINAL：原路退款
  /// - BALANCE：退回到余额
  /// - OTHER_BALANCE：原账户异常退到其他余额账户
  /// - OTHER_BANKCARD：原银行卡异常退到其他银行卡
  ///
  /// 示例值：ORIGINAL
  pub channel: String,
  /// 退款入账账户
  ///
  /// 取当前退款单的退款入账方，有以下几种情况：
  /// 1. 退回银行卡：{银行名称}{卡类型}{卡尾号}
  /// 2. 退回支付用户零钱：支付用户零钱
  /// 3. 退还商户：商户基本账户商户结算银行账户
  /// 4. 退回支付用户零钱通：支付用户零钱通
  ///
  /// 示例值：招商银行信用卡0403
  pub user_received_account: String,
  /// 退款成功时间
  ///
  /// 退款成功时间，当退款状态为退款成功时有返回。
  ///
  /// 示例值：2020-12-01T16:18:12+08:00
  pub success_time: Option<String>,
  /// 退款创建时间
  ///
  /// 退款受理时间
  ///
  /// 示例值：2020-12-01T16:18:12+08:00
  pub create_time: String,
  /// 退款状态
  ///
  /// 退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往[商户平台](https://pay.weixin.qq.com/)-交易中心，手动处理此笔退款。
  ///
  /// 枚举值：
  /// - SUCCESS：退款成功
  /// - CLOSED：退款关闭
  /// - PROCESSING：退款处理中
  /// - ABNORMAL：退款异常
  ///
  /// 示例值：SUCCESS
  pub status: String,
  /// 资金账户
  ///
  /// 退款所使用资金对应的资金账户类型
  ///
  /// 枚举值：
  /// - UNSETTLED : 未结算资金
  /// - AVAILABLE : 可用余额
  /// - UNAVAILABLE : 不可用余额
  /// - OPERATION : 运营户
  /// - BASIC : 基本账户（含可用余额和不可用余额）
  ///
  /// 示例值：UNSETTLED
  pub funds_account: Option<String>,
  /// 金额信息
  ///
  /// 金额详细信息
  pub amount: RefundAmountResponse,
  /// 优惠退款信息
  pub promotion_detail: Option<Vec<RefundPromotion>>,
}

impl Client {
  pub async fn refund(&self, req: &RefundRequest) -> Result<RefundResponse, WeChatPayError> {
    Ok(
      self
        .send_request(Method::POST, "/v3/refund/domestic/refunds", None, Some(req))
        .await?
        .unwrap(),
    )
  }
}

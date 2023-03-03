mod amount;
mod discount;
mod order;
mod payer;
mod scene;
mod settle;

pub use amount::Amount;
pub use discount::{Discount, GoodInfo};
pub use order::OrderRequest;
pub use payer::Payer;
pub use scene::{Scene, StoreInfo};
use serde::Serialize;
pub use settle::Settle;

#[derive(Serialize)]
pub struct EmptyRequest;

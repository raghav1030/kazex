use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Side;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub price: i32,
    pub qty: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub filled: i32,
    pub side: Side,
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fill {
    pub price: String,
    pub qty: i32,
    #[serde(rename = "tradeId")]
    pub trade_id: i32,
    #[serde(rename = "otherUserId")]
    pub other_user_id: String,
    #[serde(rename = "markerOrderId")]
    pub market_order_id: String,
}

struct BalanceType{
    pub available : i64,
    pub locked : i64
}
pub struct UserBalance {
    balance : HashMap<String, BalanceType>
}

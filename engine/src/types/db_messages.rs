use crate::custom_types;

use super::common::{self, Side};
use serde::{Deserialize, Serialize};

enum DbMessageType {
    TRADE_ADDED,
    ORDER_UPDATE,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TradeData {
    pub id: String,
    #[serde(rename = "isBuyerMaker")]
    pub is_buy_maker: String,
    pub price: String,
    pub quantity: String,
    #[serde(rename = "quoteQuantitiy")]
    pub quote_quantitiy: String,
    pub timestamp: i32,
    pub market: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderUpdateData {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    pub market: String,
    pub price: i32,
    pub quantity: String,
    pub side: common::Side,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]

pub enum DBMessage {
    #[serde(rename = "TRADE_ADDED")]
    TradeData(TradeData),
    #[serde(rename = "ORDER_UPDATE")]
    OrderUpdateData(OrderUpdateData),
}



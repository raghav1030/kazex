use std::{cmp, fmt::format};

use serde::{Deserialize, Serialize};

use crate::types::{Fill, Order};

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "lastTradedId")]
    pub last_traded_id: i128,
    #[serde(rename = "currentPrice")]
    pub current_price: i128,
}

impl Orderbook {
    pub fn new(
        bids: Vec<Order>,
        asks: Vec<Order>,
        base_asset: String,
        quote_asset: String,
        last_traded_id: String,
        current_price: String,
    ) -> Orderbook {
        Orderbook {
            bids,
            asks,
            base_asset,
            quote_asset,
            last_traded_id,
            current_price,
        }
    }

    pub fn ticker(&self) -> String {
        format!("{}_{}", self.base_asset, self.quote_asset)
    }

    pub fn add_order(&self, order: Order) -> (i64, Vec<Fill>) {
        if order.side == "buy" {
            let (executedQty, fills) = self.match_bids(order);
        } else {
        }
    }

    pub fn match_bids(&mut self, order: Order) -> (i32, Vec<Fill>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut executedQty = 0;

        for i in 0..self.asks.len() {
            if self.asks[i].price <= order.price && executedQty <= order.qty {
                let filledQty = cmp::min(order.qty - executedQty, self.asks[i].qty);
                executedQty += &filledQty;
                self.asks[i].filled += filledQty;
                self.last_traded_id += 1;

                fills.push(Fill {
                    price: self.asks[i].price,
                    qty: filledQty,
                    trade_id: self.last_traded_id,
                    other_user_id: self.asks[i].user_id.clone(),
                    market_order_id: self.asks[i].order_id.clone(),
                })
            }
        }

        

        (executedQty, fills)
    }
}

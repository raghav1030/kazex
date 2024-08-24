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
    pub current_price: i32,
}

impl Orderbook {
    pub fn new(
        bids: Vec<Order>,
        asks: Vec<Order>,
        base_asset: String,
        quote_asset: String,
        last_traded_id: i128,
        current_price: i32,
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

    pub fn add_order(&mut self, order: &mut Order) -> (i32, Vec<Fill>) {
        if order.side == "buy" {
            let (executed_qty, fills) = self.match_bids(&order);
            order.filled = executed_qty;

            if executed_qty == order.qty {
                return (executed_qty, fills);
            }

            self.bids.push(order.clone());
            return (executed_qty, fills);
        } else {
            let (executed_qty, fills) = self.match_asks(&order);

            if order.qty == executed_qty {
                return (executed_qty, fills);
            } else {
                self.asks.push(order.clone());
                return (executed_qty, fills);
            }
        }
    }

    pub fn match_bids(&mut self, order: &Order) -> (i32, Vec<Fill>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut executedQty = 0;

        self.asks.sort_by(|a, b| {
            if a.price < b.price {
                std::cmp::Ordering::Less
            } else if a.price > b.price {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        for i in 0..self.asks.len() {
            if executedQty == order.qty {
                break;
            }

            if self.asks[i].price <= order.price
                && executedQty <= order.qty
                && self.asks[i].user_id != order.user_id
            {
                let remainingQty = order.qty - executedQty;
                let filledQty = cmp::min(remainingQty, self.asks[i].qty);
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

        self.asks.retain(|ask| ask.filled != ask.qty);

        (executedQty, fills)
    }

    pub fn match_asks(&mut self, order: &Order) -> (i32, Vec<Fill>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut executedQty = 0;

        self.bids.sort_by(|a, b| {
            if a.price < b.price {
                std::cmp::Ordering::Less
            } else if a.price > b.price {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        for i in 0..self.bids.len() {
            if executedQty == order.qty {
                break;
            } else {
                if self.bids[i].price >= order.price
                    && executedQty < order.qty
                    && self.bids[i].user_id != order.user_id
                {
                    let remaining_qty = self.bids[i].qty - order.qty;
                    let filled_qty = cmp::min(remaining_qty, self.bids[i].qty);

                    executedQty += filled_qty;
                    self.bids[i].filled += filled_qty;
                    self.last_traded_id += 1;

                    fills.push(Fill {
                        price: self.bids[i].price,
                        qty: filled_qty,
                        trade_id: self.last_traded_id,
                        other_user_id: self.bids[i].user_id.clone(),
                        market_order_id: self.bids[i].order_id.clone(),
                    });
                }
            }

            self.bids.retain(|bids| bids.filled != bids.qty);
        }

        (executedQty, fills)
    }
}

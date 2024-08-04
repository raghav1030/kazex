use std::fmt::format;

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
    pub last_traded_id: String,
    #[serde(rename = "currentPrice")]
    pub current_price: String,
}

impl Orderbook {

    pub fn new(bids : Vec<Order>, asks : Vec<Order>, base_asset : String, quote_asset : String, last_traded_id : String, current_price : String) -> Orderbook {
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

    
}
use std::convert::TryFrom;
use std::{env, fs};
use std::{collections::HashMap, fmt::Error};

use prost::Message;
use prost_types::Value;
use rand::{random, Rng};

use crate::types::{Fill, Order};
use crate::{
    custom_types::{
        message_to_engine::{
            self, message_to_engine::Payload, MessageToEngine, MessageToEngineType,
        },
        orderbook_engine_messages::{
            message_from_order_book, Fills, MessageFromOrderBook, MessageFromOrderbookType,
        },
    },
    types::{MessageFromApi, UserBalance},
};

use super::orderbook::Orderbook;
use base64::prelude::*;

pub struct Engine {
    pub orderbooks: Vec<Orderbook>,
    pub balances: HashMap<String, UserBalance>,
}

impl Engine {

    pub fn new(&self) -> Engine {
        match env::var("WITH_SNAPSHOT") {
            Ok(val) => {
                if val == "true" {
                    if let Ok(snapshot) = fs::read_to_string("../utils/snapshot.json") {
                        let v : Value = serde_json::from_str(&snapshot).expect("Failed to parse JSON string");
                    }
                }
            },
            Err(e) => println!("Failed to read environment variable: {}", e),
        }
        

        let orderbook : Vec<Orderbook> = Vec::new();
        let balances : HashMap<String, UserBalance> = HashMap::new();
        Engine {
            orderbooks: Vec::new(),
            balances: HashMap::new(),
        }
    }

    pub async fn process(&mut self, message: &MessageFromApi) {
        // Convert Box<Vec<u8>> to &[u8]
        // let payload = message.message.as_ref();
        // Decode the Protobuf message

        let base64_encoded = &message.message;
        let payload = match base64::decode(base64_encoded) {
            Ok(decoded) => decoded,
            Err(e) => {
                println!("Failed to decode base64 message: {}", e);
                return;
            }
        };

        println!("message from api: {:?}", message.message);
        println!("base64 decoded: {:?}", payload);
        match MessageToEngine::decode(payload.as_ref()) {
            Ok(decoded_message) => {
                println!("Decoded Protobuf message: {:?}", decoded_message);
                match MessageToEngineType::try_from(decoded_message.r#type) {
                    Ok(MessageToEngineType::CancelOrder) => {
                        println!("Cancel Order");
                        if let Some(
                            message_to_engine::message_to_engine::Payload::CancelOrderPayload(
                                payload,
                            ),
                        ) = decoded_message.payload
                        {
                            println!("Cancel Order payload: {:?}", payload);
                        }
                    }

                    Ok(MessageToEngineType::CreateOrder) => {
                        println!("Create Order");
                        if let Some(
                            message_to_engine::message_to_engine::Payload::CreateOrderPayload(
                                payload,
                            ),
                        ) = decoded_message.payload
                        {
                            println!("Create Order payload: {:?}", payload);

                            // let (executed_qty, fills, order_id) = Engine::create_order(&mut self, &payload);
                            let (executed_qty, fills, order_id) = self.create_order(&payload);
                        }
                    }

                    Ok(MessageToEngineType::GetDepth) => {
                        println!("Get Depth");
                        if let Some(
                            message_to_engine::message_to_engine::Payload::GetDepthPayload(payload),
                        ) = decoded_message.payload
                        {
                            println!("Get Depth payload: {:?}", payload);
                        }
                    }

                    Ok(MessageToEngineType::GetOpenOrders) => {
                        println!("Get Open Orders");
                        if let Some(
                            message_to_engine::message_to_engine::Payload::GetOpenOrdersPayload(
                                payload,
                            ),
                        ) = decoded_message.payload
                        {
                            println!("Get Open Order payload: {:?}", payload);
                        }
                    }

                    Ok(MessageToEngineType::OnRamp) => {
                        println!("On Ramp");

                        if let Some(message_to_engine::message_to_engine::Payload::OnRampPayload(
                            payload,
                        )) = decoded_message.payload
                        {
                            println!("OnRampPayload: {:?}", payload);
                            // Process OnRampPayload here
                        }
                    }

                    Err(_) => {
                        println!("Unknown message type: {}", decoded_message.r#type);
                    }
                }
            }
            Err(e) => {
                println!("Failed to decode Protobuf message: {}", e);
            }
        }
    }

    pub fn create_order(
        &mut self,
        payload: &message_to_engine::CreateOrderPayload,
    ) -> (i32, Vec<Fill>, String) {
        let base_asset = payload.market.split("_").next().unwrap();
        let quote_asset = payload.market.split("_").last().unwrap();

        match self.check_and_lock_funds(
            &base_asset.to_string(),
            &quote_asset.to_string(),
            &payload.side().as_str_name().to_string(),
            &payload.price,
            &payload.qty,
            &payload.user_id,
            // &quote_asset.to_string(),
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                return (0, Vec::new(), e.to_string());
            }
        }

        let orderbook = self
            .orderbooks
            .iter_mut()
            .find(|orderbook| orderbook.ticker() == payload.market);

        match orderbook {
            Some(ob) => {
                let mut order = Order {
                    price: payload.price,
                    qty: payload.qty,
                    order_id: Engine::generate_order_id(10),
                    filled: 0,
                    side: payload.side().as_str_name().to_string(),
                    user_id: payload.user_id.clone(),
                };

                let (executed_qty, fills) = ob.add_order(&mut order);
                (executed_qty, fills, order.order_id.clone())
            }

            None => {
                panic!("Orderbook not found for market: {}", payload.market);
            }
        }
    }

    pub fn check_and_lock_funds(
        &mut self,
        base_asset: &String,
        quote_asset: &String,
        side: &String,
        price: &i32,
        qty: &i32,
        user_id: &String,
        // asset: &String,
    ) -> Result<(), String> {
        // Check if the user has enough funds to place the order
        // Lock the funds

        if let Some(user_balance) = self.balances.get_mut(user_id) {
            if side == "buy" {
                if let Some(bal) = user_balance.balance.get_mut(quote_asset) {
                    if bal.available == 0 || bal.available < (qty * price) as i64 {
                        return Err("Insufficient funds to place".to_string());
                    } else {
                        bal.available -= (qty * price) as i64;
                        bal.locked += (qty * price) as i64;
                    }
                }
            } else {
                if let Some(bal) = user_balance.balance.get_mut(base_asset) {
                    if bal.available == 0 || bal.available < *qty as i64 {
                        return Err("Insufficient funds to place".to_string());
                    } else {
                        bal.available -= *qty as i64;
                        bal.locked += *qty as i64;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn generate_order_id(len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
        let mut rng = rand::thread_rng();

        let random_id: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        random_id
    }
}

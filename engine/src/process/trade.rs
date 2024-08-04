// use std::collections::HashMap;
// use std::convert::TryFrom;

// use prost::Message;

// use crate::{
//     custom_types::orderbook_engine_messages::{
//         message_from_order_book, MessageFromOrderBook, MessageFromOrderbookType,
//     },
//     types::{MessageFromApi, UserBalance},
// };

// use super::orderbook::Orderbook;

// pub struct Engine {
//     pub orderbook: Vec<Orderbook>,
//     pub balance: HashMap<String, UserBalance>,
// }

// impl Engine {
//     pub async fn process(message: &MessageFromApi) {
//         // Convert Box<Vec<u8>> to &[u8]
//         // let payload = &message.message.into_bytes().as_slice();
//         let payload = Box::new(&message.message.into_bytes().as_ref());
//         // Decode the Protobuf message
//         match MessageFromOrderBook::decode(payload) {
//             Ok(decoded_message) => {
//                 println!("Decoded Protobuf message: {:?}", decoded_message);
//                 match MessageFromOrderbookType::try_from(decoded_message.r#type) {
//                     Ok(MessageFromOrderbookType::OrderCancelled) => {
//                         if let Some(payload) = decoded_message.payload {
//                             if let message_from_order_book::Payload::OrderCancelledPayload(
//                                 order_cancelled_payload,
//                             ) = payload
//                             {
//                                 println!("Order cancelled payload: {:?}", order_cancelled_payload);
//                                 // Process order cancelled payload
//                             }
//                         }
//                     }
//                     Ok(MessageFromOrderbookType::Depth) => {
//                         if let Some(payload) = decoded_message.payload {
//                             if let message_from_order_book::Payload::DepthPayload(depth_payload) =
//                                 payload
//                             {
//                                 println!("Depth payload: {:?}", depth_payload);
//                                 // Process depth payload
//                             }
//                         }
//                     }
//                     Ok(MessageFromOrderbookType::OrderPlaced) => {
//                         if let Some(payload) = decoded_message.payload {
//                             if let message_from_order_book::Payload::OrderCompletedPayload(
//                                 order_completed_payload,
//                             ) = payload
//                             {
//                                 println!("Order completed payload: {:?}", order_completed_payload);
//                                 // Process order completed payload
//                             }
//                         }
//                     }
//                     Ok(MessageFromOrderbookType::OpenOrders) => {
//                         if let Some(payload) = decoded_message.payload {
//                             if let message_from_order_book::Payload::OpenOrdersPayload(
//                                 open_orders_payload,
//                             ) = payload
//                             {
//                                 println!("Open orders payload: {:?}", open_orders_payload);
//                                 // Process open orders payload
//                             }
//                         }
//                     }
//                     Err(_) => {
//                         println!("Unknown message type: {}", decoded_message.r#type);
//                     }
//                 }
//             }
//             Err(e) => {
//                 println!("Failed to decode Protobuf message: {}", e);
//             }
//         }
//     }
// }


use std::collections::HashMap;
use std::convert::TryFrom;


use prost::Message;

use crate::{
    custom_types::{
        message_to_engine::{self, MessageToEngine, MessageToEngineType, message_to_engine::Payload},
        orderbook_engine_messages::{
            message_from_order_book, MessageFromOrderBook, MessageFromOrderbookType,
        }
    },
    types::{MessageFromApi, UserBalance},
};



use super::orderbook::Orderbook;
use base64::{prelude::*};


pub struct Engine {
    pub orderbook: Vec<Orderbook>,
    pub balance: HashMap<String, UserBalance>,
}

impl Engine {
    pub async fn process(message: &MessageFromApi) {
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
                    },
                    
                    Ok(MessageToEngineType::CreateOrder) => {
                        println!("Create Order");
                    },

                    Ok(MessageToEngineType::GetDepth) => {
                        println!("Get Depth");

                    },

                    Ok(MessageToEngineType::GetOpenOrders) => {
                        println!("Get Open Orders");

                    },

                    Ok(MessageToEngineType::OnRamp) => {

                        println!("On Ramp");

                        if let Some(message_to_engine::message_to_engine::Payload::OnRampPayload(payload)) = decoded_message.payload {
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
}

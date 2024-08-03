use serde::{Deserialize, Serialize};

// use crate::custom_types;

#[derive(Deserialize, Serialize, Debug)]
pub enum Side {
    #[serde(rename="buy")]
    Buy,
    #[serde(rename="sell")]
    Sell,
}



use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Side{
    buy,
    sell
}
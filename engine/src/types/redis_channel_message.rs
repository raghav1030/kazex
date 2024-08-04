use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFromApi {
    pub clientId : String,
    pub message : String,
}
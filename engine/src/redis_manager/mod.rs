use once_cell::sync::Lazy;
use redis::{Client, RedisError};
use std::sync::Mutex;

pub mod redis_manager {
    use redis::{Commands, Connection};

    use crate::{custom_types, types::DBMessage};

    use super::*;

    pub struct RedisManager {
        client: Client,
    }

    impl RedisManager {
        fn get_redis_client() -> Result<Client, RedisError> {
            let redis_host_name = "localhost:6379";
            let redis_password = "";

            let redis_conn_url = if redis_password.is_empty() {
                format!("redis://{}", redis_host_name)
            } else {
                format!("redis://:{}@{}", redis_password, redis_host_name)
            };

            Client::open(redis_conn_url).map_err(|e| e.into())
        }

        pub fn new() -> Self {
            let client = Self::get_redis_client().expect("Failed to create Redis client");
            RedisManager { client }
        }

        pub fn instance() -> &'static Mutex<RedisManager> {
            static INSTANCE: Lazy<Mutex<RedisManager>> =
                Lazy::new(|| Mutex::new(RedisManager::new()));
            &INSTANCE
        }

        pub fn get_client(&self) -> &Client {
            &self.client
        }

        pub fn push_message(
            &self,
            message: &DBMessage,
            conn: &mut Connection,
        ) -> redis::RedisResult<()> {
            conn.lpush(
                "db_processor",
                serde_json::to_string(message).expect("Failed to serialize message"),
            )?;
            Ok(())
        }

        pub fn publish_message_on_ws(
            &self,
            channel: &String,
            message: &DBMessage,
            conn: &mut Connection,
        ) -> redis::RedisResult<()> {
            conn.publish(
                channel,
                serde_json::to_string(message).expect("Failed to serialize message"),
            )?;
            Ok(())
        }

        pub fn send_message_to_api(client_id : &String, message : &custom_types::orderbook_engine_messages::MessageFromOrderBook, conn : &mut Connection) -> redis::RedisResult<()> {
            conn.publish(client_id, serde_json::to_string(message).expect("Failed to serialize message"))?;
            Ok(())

        }
    }
}

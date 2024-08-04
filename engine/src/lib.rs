pub mod config;
pub mod redis_manager;
pub mod types;
pub mod process;
pub mod custom_types {
    pub mod orderbook_engine_messages {
        include!(concat!(
            env!("OUT_DIR"),
            "/custom_types.orderbook_engine_messages.rs"
        ));
    }

    pub mod message_to_engine {
        include!(concat!(
            env!("OUT_DIR"),
            "/custom_types.message_to_engine.rs"
        ));
    }

    pub mod common {
        include!(concat!(env!("OUT_DIR"), "/custom_types.common.rs"));
    }
}

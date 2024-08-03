use std::{env, path::PathBuf};

use prost_serde::build_with_serde;
use prost_wkt_build::{FileDescriptorSet, Message};


fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // prost_build::compile_protos("../protobuf/message_from_orderbook.proto")?;
//     // prost_build::compile_protos(&["../protobuf/message_to_engine.proto", "../protobuf/message_from_orderbook.proto", "../protobuf/common.proto"], &["../protobuf"])?;
//     tonic_build::configure()
//     .type_attribute("custom_types.orderbook_engine_messages.MessageFromOrderBook", "#[derive(prost_serde_derive::Deserialize, prost_serde_derive::Serialize)]")
//     // Add following if you have to allow missing non-nullable fields when deserializing (will be filled with the default value)
//     // .type_attribute("some.proto.SomeStruct", "#[prost_serde_derive(use_default_for_missing_fields)]")
//     // Add following if you have to allow deserializing even if there are type errors
//     // .type_attribute("some.proto.SomeStruct", "#[prost_serde_derive(omit_type_errors)]")
//     // Serializing/deserializing Prost enumerations are also available
//     .type_attribute("custom_types.orderbook_engine_messages.MessageFromOrderBook", "#[derive(prost_serde_derive::Deserialize, prost_serde_derive::Serialize)]")
//     .compile(&["../protobuf/message_to_engine.proto", "../protobuf/message_from_orderbook.proto", "../protobuf/common.proto"], &["../protobuf"])?;
// // let json = include_str!("src/config/build_config_proto.json");

// // build_with_serde(json);
    
    
//     Ok(())



let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_file = out.join("descriptors.bin");
    let mut prost_build = prost_build::Config::new();
    prost_build
        .type_attribute(
            ".",
            "#[derive(serde::Serialize,serde::Deserialize)]"
        )
        .extern_path(
            ".google.protobuf.Any",
            "::prost_wkt_types::Any"
        )
        .extern_path(
            ".google.protobuf.Timestamp",
            "::prost_wkt_types::Timestamp"
        )
        .extern_path(
            ".google.protobuf.Value",
            "::prost_wkt_types::Value"
        )
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(&["../protobuf/message_to_engine.proto", "../protobuf/message_from_orderbook.proto", "../protobuf/common.proto"], &["../protobuf"])
        .unwrap();

    let descriptor_bytes =
        std::fs::read(descriptor_file)
        .unwrap();

    let descriptor =
        FileDescriptorSet::decode(&descriptor_bytes[..])
        .unwrap();

    prost_wkt_build::add_serde(out, descriptor);
    Ok(())
}
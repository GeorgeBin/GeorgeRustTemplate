pub mod common;
pub mod config;
pub mod data;
pub mod network;
pub mod platform;
pub mod runtime;
pub mod security;
pub mod storage;

use crate::ErrorDescriptor;

pub const ALL_DESCRIPTORS: [&ErrorDescriptor; 16] = [
    &common::COMMON_INVALID_INPUT,
    &common::COMMON_INTERNAL,
    &config::CONFIG_MISSING_VALUE,
    &config::CONFIG_INVALID_VALUE,
    &runtime::RUNTIME_INVALID_STATE,
    &runtime::RUNTIME_TASK_CANCELLED,
    &network::NET_DNS_LOOKUP_FAILED,
    &network::NET_TCP_TIMEOUT,
    &network::NET_HTTP_NOT_FOUND,
    &storage::STORAGE_IO_FAILED,
    &storage::STORAGE_NOT_FOUND,
    &data::DATA_JSON_PARSE_FAILED,
    &data::DATA_PROTOCOL_DECODE_FAILED,
    &security::SEC_AUTH_PERMISSION_DENIED,
    &security::SEC_CERT_VERIFY_FAILED,
    &platform::PLATFORM_CALLBACK_UNAVAILABLE,
];

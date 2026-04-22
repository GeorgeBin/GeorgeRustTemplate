use std::collections::BTreeSet;

use crate::{
    BaseError, ErrorCode, ErrorContext, ErrorKind, NativeError,
    catalog::{
        self,
        data::DATA_JSON_DECODE_FAILED,
        network::{NET_HTTP_NOT_FOUND, NET_SOCKET_CONNECTION_REFUSED, NET_TCP_CONNECT_TIMEOUT},
        platform::PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE,
    },
};

#[test]
fn error_code_from_parts_encodes_fields() {
    let code = ErrorCode::from_parts(3, 6, 404);

    assert_eq!(code.as_u32(), 306_404);
    assert_eq!(code.domain(), 3);
    assert_eq!(code.subdomain(), 6);
    assert_eq!(code.reason(), 404);
}

#[test]
fn error_code_display_is_zero_padded() {
    let code = ErrorCode::from_parts(3, 6, 404);
    let min_code = ErrorCode::from_parts(0, 0, 1);

    assert_eq!(code.to_string(), "0306404");
    assert_eq!(min_code.to_string(), "0000001");
}

#[test]
#[should_panic(expected = "domain must be in 0..=99")]
fn error_code_panics_when_domain_is_out_of_range() {
    let _ = ErrorCode::from_parts(100, 0, 0);
}

#[test]
#[should_panic(expected = "subdomain must be in 0..=99")]
fn error_code_panics_when_subdomain_is_out_of_range() {
    let _ = ErrorCode::from_parts(0, 100, 0);
}

#[test]
#[should_panic(expected = "reason must be in 0..=999")]
fn error_code_panics_when_reason_is_out_of_range() {
    let _ = ErrorCode::from_parts(0, 0, 1000);
}

#[test]
fn base_error_new_binds_descriptor() {
    let error = BaseError::new(&NET_HTTP_NOT_FOUND);

    assert_eq!(error.desc.name, "NET_HTTP_NOT_FOUND");
    assert_eq!(error.desc.kind, ErrorKind::NotFound);
    assert_eq!(error.detail, None);
    assert_eq!(error.native, None);
    assert!(error.context.is_empty());
    assert_eq!(
        error.to_string(),
        "[0306404][NET_HTTP_NOT_FOUND] HTTP resource not found"
    );
    assert_eq!(error.code(), ErrorCode::from_parts(3, 6, 404));
    assert_eq!(error.name(), "NET_HTTP_NOT_FOUND");
    assert_eq!(error.kind(), ErrorKind::NotFound);
    assert_eq!(error.default_message(), "HTTP resource not found");
}

#[test]
fn base_error_builder_chains_and_keeps_context_order() {
    let error = BaseError::new(&NET_HTTP_NOT_FOUND)
        .detail("GET https://api.example.com/time returned 404")
        .native("http", Some("404"), Some("Not Found"))
        .context("method", "GET")
        .context("url", "https://api.example.com/time");

    assert_eq!(
        error.detail.as_deref(),
        Some("GET https://api.example.com/time returned 404")
    );

    let native = error.native.as_ref().expect("native error");
    assert_eq!(native.source, "http");
    assert_eq!(native.code.as_deref(), Some("404"));
    assert_eq!(native.message.as_deref(), Some("Not Found"));

    assert_eq!(error.context.len(), 2);
    assert_eq!(error.context[0].key, "method");
    assert_eq!(error.context[0].value, "GET");
    assert_eq!(error.context[1].key, "url");
    assert_eq!(error.context[1].value, "https://api.example.com/time");
    assert_eq!(
        error.to_string(),
        "[0306404][NET_HTTP_NOT_FOUND] HTTP resource not found | GET https://api.example.com/time returned 404"
    );
}

#[test]
fn base_error_debug_includes_core_descriptor_fields() {
    let error = BaseError::new(&NET_HTTP_NOT_FOUND).detail("detail");
    let debug = format!("{error:?}");

    assert!(debug.contains("code"));
    assert!(debug.contains("0306404"));
    assert!(debug.contains("name"));
    assert!(debug.contains("NET_HTTP_NOT_FOUND"));
    assert!(debug.contains("kind"));
    assert!(debug.contains("NotFound"));
    assert!(debug.contains("default_message"));
    assert!(debug.contains("HTTP resource not found"));
}

#[test]
fn native_error_constructor_preserves_values() {
    let native = NativeError::new("http", Some("404"), Some("Not Found"));

    assert_eq!(native.source, "http");
    assert_eq!(native.code.as_deref(), Some("404"));
    assert_eq!(native.message.as_deref(), Some("Not Found"));
}

#[test]
fn error_context_constructor_stringifies_value() {
    let context = ErrorContext::new("attempt", 3);

    assert_eq!(context.key, "attempt");
    assert_eq!(context.value, "3");
}

#[test]
fn catalog_formal_descriptors_have_expected_fields() {
    assert_eq!(NET_HTTP_NOT_FOUND.code, ErrorCode::from_parts(3, 6, 404));
    assert_eq!(NET_HTTP_NOT_FOUND.name, "NET_HTTP_NOT_FOUND");
    assert_eq!(NET_HTTP_NOT_FOUND.kind, ErrorKind::NotFound);
    assert_eq!(
        NET_HTTP_NOT_FOUND.default_message,
        "HTTP resource not found"
    );

    assert_eq!(NET_TCP_CONNECT_TIMEOUT.code, ErrorCode::from_parts(3, 3, 1));
    assert_eq!(NET_TCP_CONNECT_TIMEOUT.name, "NET_TCP_CONNECT_TIMEOUT");
    assert_eq!(NET_TCP_CONNECT_TIMEOUT.kind, ErrorKind::Timeout);
    assert_eq!(
        NET_TCP_CONNECT_TIMEOUT.default_message,
        "tcp connect timed out"
    );

    assert_eq!(
        NET_SOCKET_CONNECTION_REFUSED.code,
        ErrorCode::from_parts(3, 5, 1)
    );
    assert_eq!(
        NET_SOCKET_CONNECTION_REFUSED.name,
        "NET_SOCKET_CONNECTION_REFUSED"
    );
    assert_eq!(NET_SOCKET_CONNECTION_REFUSED.kind, ErrorKind::Unavailable);
    assert_eq!(
        NET_SOCKET_CONNECTION_REFUSED.default_message,
        "socket connection refused"
    );

    assert_eq!(DATA_JSON_DECODE_FAILED.code, ErrorCode::from_parts(5, 1, 1));
    assert_eq!(DATA_JSON_DECODE_FAILED.name, "DATA_JSON_DECODE_FAILED");
    assert_eq!(DATA_JSON_DECODE_FAILED.kind, ErrorKind::Decode);
    assert_eq!(
        DATA_JSON_DECODE_FAILED.default_message,
        "JSON decode failed"
    );

    assert_eq!(
        PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE.code,
        ErrorCode::from_parts(7, 2, 1)
    );
    assert_eq!(
        PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE.name,
        "PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE"
    );
    assert_eq!(
        PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE.kind,
        ErrorKind::Unavailable
    );
    assert_eq!(
        PLATFORM_JNI_CALLBACK_THREAD_UNAVAILABLE.default_message,
        "JNI callback thread is unavailable"
    );
}

#[test]
fn catalog_codes_are_unique() {
    let mut seen = BTreeSet::new();

    for desc in catalog::ALL_DESCRIPTORS {
        assert!(
            seen.insert(desc.code.as_u32()),
            "duplicate error code {} for {}",
            desc.code,
            desc.name
        );
    }
}

#[test]
fn catalog_names_are_unique() {
    let mut seen = BTreeSet::new();

    for desc in catalog::ALL_DESCRIPTORS {
        assert!(seen.insert(desc.name), "duplicate error name {}", desc.name);
    }
}

#[cfg(feature = "std")]
#[test]
fn base_error_source_preserves_std_error_chain() {
    let io_error = std::io::Error::other("socket closed");
    let error = BaseError::new(&NET_HTTP_NOT_FOUND).source(io_error);

    let source = std::error::Error::source(&error).expect("source");
    assert_eq!(source.to_string(), "socket closed");
}

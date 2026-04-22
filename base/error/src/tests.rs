use std::collections::BTreeSet;

use crate::{
    BaseError, ErrorCode, ErrorKind,
    catalog::{self, network::NET_HTTP_NOT_FOUND},
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

    assert_eq!(code.to_string(), "0306404");
}

#[test]
fn base_error_new_binds_descriptor() {
    let error = BaseError::new(&NET_HTTP_NOT_FOUND);

    assert_eq!(error.desc.name, "NET_HTTP_NOT_FOUND");
    assert_eq!(error.desc.kind, ErrorKind::NotFound);
    assert_eq!(error.detail, None);
    assert_eq!(error.native, None);
    assert!(error.context.is_empty());
    assert_eq!(error.to_string(), "http resource not found");
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
        "GET https://api.example.com/time returned 404"
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

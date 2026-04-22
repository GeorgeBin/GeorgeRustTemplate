mod request;
mod response;

pub use request::NtpRequest;
pub use response::NtpResponse;

#[cfg(test)]
mod tests {
    use george_base_types::NonEmptyString;

    use super::{NtpRequest, NtpResponse};

    #[test]
    fn ntp_request_export_path_is_usable() {
        let server = NonEmptyString::try_from("pool.ntp.org").expect("server should be valid");
        let request = NtpRequest::new(server, 123, 3_000);

        assert_eq!(request.port, 123);
        assert_eq!(request.timeout_millis, 3_000);
    }

    #[test]
    fn ntp_response_export_path_is_usable() {
        let response = NtpResponse::new(1_700_000_000_000, Some(24));

        assert_eq!(response.server_unix_millis, 1_700_000_000_000);
        assert_eq!(response.round_trip_millis, Some(24));
    }
}

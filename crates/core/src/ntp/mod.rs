mod error;
mod parser;
mod port;
mod service;

pub use error::NtpCoreError;
pub use parser::parse_transmit_timestamp_millis;
pub use port::{NtpTransport, NtpTransportError};
pub use service::NtpService;

#[cfg(test)]
mod tests {
    use george_base_types::NonEmptyString;

    use super::{NtpCoreError, NtpService, parse_transmit_timestamp_millis};
    use crate::ntp::{NtpTransport, NtpTransportError};
    use template_model::ntp::NtpRequest;

    struct SuccessTransport;

    impl NtpTransport for SuccessTransport {
        fn query(&self, _request: &NtpRequest) -> Result<[u8; 48], NtpTransportError> {
            let mut packet = [0u8; 48];
            let unix_secs = 1_700_000_000u64;
            let ntp_secs = unix_secs + super::parser::NTP_TIMESTAMP_DELTA;

            packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());

            Ok(packet)
        }
    }

    #[test]
    fn public_export_paths_are_usable() {
        let server = NonEmptyString::try_from("pool.ntp.org").expect("server should be valid");
        let request = NtpRequest::new(server, 123, 3_000);
        let service = NtpService::new(SuccessTransport);
        let response = service.query(&request).expect("query should succeed");

        let parsed = parse_transmit_timestamp_millis(&{
            let mut packet = [0u8; 48];
            let unix_secs = 1_700_000_000u64;
            let ntp_secs = unix_secs + super::parser::NTP_TIMESTAMP_DELTA;
            packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());
            packet
        })
        .expect("parser should work");

        assert_eq!(response.server_unix_millis, parsed);
        assert_eq!(NtpCoreError::InvalidRequest.to_string(), "invalid request");
    }
}

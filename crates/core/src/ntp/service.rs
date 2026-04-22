use crate::ntp::{NtpCoreError, NtpTransport, NtpTransportError, parse_transmit_timestamp_millis};
use template_model::ntp::{NtpRequest, NtpResponse};

#[derive(Debug, Clone)]
pub struct NtpService<T> {
    transport: T,
}

impl<T: NtpTransport> NtpService<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn query(&self, request: &NtpRequest) -> Result<NtpResponse, NtpCoreError> {
        if request.port == 0 || request.timeout_millis == 0 {
            return Err(NtpCoreError::InvalidRequest);
        }

        let packet = self.transport.query(request).map_err(map_transport_error)?;
        let server_unix_millis = parse_transmit_timestamp_millis(&packet)?;

        Ok(NtpResponse::new(server_unix_millis, None))
    }
}

fn map_transport_error(error: NtpTransportError) -> NtpCoreError {
    match error {
        NtpTransportError::Timeout => NtpCoreError::Timeout,
        NtpTransportError::Unavailable => NtpCoreError::TransportUnavailable,
        NtpTransportError::UpstreamFailed => NtpCoreError::UpstreamFailed,
    }
}

#[cfg(test)]
mod tests {
    use george_base_types::NonEmptyString;

    use super::NtpService;
    use crate::ntp::{NtpCoreError, NtpTransport, NtpTransportError, parser::NTP_TIMESTAMP_DELTA};
    use template_model::ntp::NtpRequest;

    #[derive(Clone, Copy)]
    enum FakeTransportMode {
        Success,
        Timeout,
        Unavailable,
        UpstreamFailed,
        InvalidPacket,
    }

    struct FakeTransport {
        mode: FakeTransportMode,
    }

    impl NtpTransport for FakeTransport {
        fn query(&self, _request: &NtpRequest) -> Result<[u8; 48], NtpTransportError> {
            match self.mode {
                FakeTransportMode::Success => {
                    let mut packet = [0u8; 48];
                    let unix_secs = 1_700_000_000u64;
                    let ntp_secs = unix_secs + NTP_TIMESTAMP_DELTA;

                    packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());

                    Ok(packet)
                }
                FakeTransportMode::Timeout => Err(NtpTransportError::Timeout),
                FakeTransportMode::Unavailable => Err(NtpTransportError::Unavailable),
                FakeTransportMode::UpstreamFailed => Err(NtpTransportError::UpstreamFailed),
                FakeTransportMode::InvalidPacket => {
                    let mut packet = [0u8; 48];
                    packet[40..44].copy_from_slice(&1u32.to_be_bytes());
                    Ok(packet)
                }
            }
        }
    }

    fn make_request(port: u16, timeout_millis: u32) -> NtpRequest {
        let server = NonEmptyString::try_from("pool.ntp.org").expect("server should be valid");
        NtpRequest::new(server, port, timeout_millis)
    }

    #[test]
    fn query_returns_response_on_success() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::Success,
        });
        let request = make_request(123, 3_000);

        let response = service.query(&request).expect("query should succeed");

        assert_eq!(response.server_unix_millis, 1_700_000_000_000);
        assert_eq!(response.round_trip_millis, None);
    }

    #[test]
    fn timeout_maps_to_core_timeout() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::Timeout,
        });

        let result = service.query(&make_request(123, 3_000));

        assert_eq!(result, Err(NtpCoreError::Timeout));
    }

    #[test]
    fn unavailable_maps_to_transport_unavailable() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::Unavailable,
        });

        let result = service.query(&make_request(123, 3_000));

        assert_eq!(result, Err(NtpCoreError::TransportUnavailable));
    }

    #[test]
    fn upstream_failure_maps_to_upstream_failed() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::UpstreamFailed,
        });

        let result = service.query(&make_request(123, 3_000));

        assert_eq!(result, Err(NtpCoreError::UpstreamFailed));
    }

    #[test]
    fn invalid_packets_map_to_invalid_response() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::InvalidPacket,
        });

        let result = service.query(&make_request(123, 3_000));

        assert_eq!(result, Err(NtpCoreError::InvalidResponse));
    }

    #[test]
    fn invalid_request_is_rejected_when_port_is_zero() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::Success,
        });

        let result = service.query(&make_request(0, 3_000));

        assert_eq!(result, Err(NtpCoreError::InvalidRequest));
    }

    #[test]
    fn invalid_request_is_rejected_when_timeout_is_zero() {
        let service = NtpService::new(FakeTransport {
            mode: FakeTransportMode::Success,
        });

        let result = service.query(&make_request(123, 0));

        assert_eq!(result, Err(NtpCoreError::InvalidRequest));
    }
}

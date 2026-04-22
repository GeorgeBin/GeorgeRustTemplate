#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpResponse {
    pub server_unix_millis: u64,
    pub round_trip_millis: Option<u32>,
}

impl NtpResponse {
    pub fn new(server_unix_millis: u64, round_trip_millis: Option<u32>) -> Self {
        Self {
            server_unix_millis,
            round_trip_millis,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NtpResponse;

    #[test]
    fn new_assigns_all_fields() {
        let response = NtpResponse::new(1_700_000_000_000, Some(18));

        assert_eq!(response.server_unix_millis, 1_700_000_000_000);
        assert_eq!(response.round_trip_millis, Some(18));
    }

    #[test]
    fn round_trip_millis_can_be_none() {
        let response = NtpResponse::new(1_700_000_000_000, None);

        assert_eq!(response.server_unix_millis, 1_700_000_000_000);
        assert_eq!(response.round_trip_millis, None);
    }

    #[test]
    fn round_trip_millis_can_be_some() {
        let response = NtpResponse::new(1_700_000_000_123, Some(42));

        assert_eq!(response.server_unix_millis, 1_700_000_000_123);
        assert_eq!(response.round_trip_millis, Some(42));
    }
}

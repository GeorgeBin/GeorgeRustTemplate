use george_base_types::NonEmptyString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtpRequest {
    pub server: NonEmptyString,
    pub port: u16,
    pub timeout_millis: u32,
}

impl NtpRequest {
    pub fn new(server: NonEmptyString, port: u16, timeout_millis: u32) -> Self {
        Self {
            server,
            port,
            timeout_millis,
        }
    }
}

#[cfg(test)]
mod tests {
    use george_base_types::NonEmptyString;

    use super::NtpRequest;

    #[test]
    fn new_assigns_all_fields() {
        let server = NonEmptyString::try_from("time.cloudflare.com")
            .expect("server should be a non-empty string");
        let request = NtpRequest::new(server.clone(), 123, 5_000);

        assert_eq!(request.server, server);
        assert_eq!(request.port, 123);
        assert_eq!(request.timeout_millis, 5_000);
    }

    #[test]
    fn server_uses_non_empty_string() {
        let server = NonEmptyString::try_from("pool.ntp.org").expect("server should be valid");
        let request = NtpRequest::new(server.clone(), 123, 1_500);

        assert_eq!(request.server.as_str(), server.as_str());
    }

    #[test]
    fn request_can_be_built_from_non_empty_string_value() {
        let server = NonEmptyString::try_from("ntp.aliyun.com")
            .expect("server should be a non-empty string");
        let request = NtpRequest::new(server, 123, 2_000);

        assert_eq!(request.server.as_str(), "ntp.aliyun.com");
        assert_eq!(request.port, 123);
        assert_eq!(request.timeout_millis, 2_000);
    }
}

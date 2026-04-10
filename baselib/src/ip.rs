use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn is_valid_ipv4(ip: &str) -> bool {
    Ipv4Addr::from_str(ip).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_ipv4_addresses() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(is_valid_ipv4("255.255.255.255"));
    }

    #[test]
    fn rejects_invalid_ipv4_addresses() {
        assert!(!is_valid_ipv4("256.1.1.1"));
        assert!(!is_valid_ipv4("192.168.1"));
        assert!(!is_valid_ipv4("a.b.c.d"));
        assert!(!is_valid_ipv4(" 127.0.0.1 "));
    }
}

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub const fn from_parts(domain: u8, subdomain: u8, reason: u16) -> Self {
        assert!(domain <= 99, "domain must be in 0..=99");
        assert!(subdomain <= 99, "subdomain must be in 0..=99");
        assert!(reason <= 999, "reason must be in 0..=999");

        Self((domain as u32) * 100_000 + (subdomain as u32) * 1_000 + reason as u32)
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub const fn domain(self) -> u8 {
        (self.0 / 100_000) as u8
    }

    pub const fn subdomain(self) -> u8 {
        ((self.0 / 1_000) % 100) as u8
    }

    pub const fn reason(self) -> u16 {
        (self.0 % 1_000) as u16
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07}", self.0)
    }
}

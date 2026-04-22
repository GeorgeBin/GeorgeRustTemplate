use crate::ntp::NtpCoreError;

pub(crate) const NTP_TIMESTAMP_DELTA: u64 = 2_208_988_800;

pub fn parse_transmit_timestamp_millis(packet: &[u8]) -> Result<u64, NtpCoreError> {
    if packet.len() < 48 {
        return Err(NtpCoreError::InvalidResponse);
    }

    let seconds = u32::from_be_bytes(
        packet[40..44]
            .try_into()
            .map_err(|_| NtpCoreError::InvalidResponse)?,
    ) as u64;
    let fraction = u32::from_be_bytes(
        packet[44..48]
            .try_into()
            .map_err(|_| NtpCoreError::InvalidResponse)?,
    ) as u64;

    if seconds < NTP_TIMESTAMP_DELTA {
        return Err(NtpCoreError::InvalidResponse);
    }

    let unix_seconds = seconds - NTP_TIMESTAMP_DELTA;
    let fraction_millis = (fraction * 1_000) >> 32;

    Ok(unix_seconds * 1_000 + fraction_millis)
}

#[cfg(test)]
mod tests {
    use super::{NTP_TIMESTAMP_DELTA, parse_transmit_timestamp_millis};
    use crate::ntp::NtpCoreError;

    #[test]
    fn parses_valid_packet_to_unix_millis() {
        let mut packet = [0u8; 48];
        let unix_secs = 1_700_000_000u64;
        let ntp_secs = unix_secs + NTP_TIMESTAMP_DELTA;
        let fraction = 0x8000_0000u32;

        packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());
        packet[44..48].copy_from_slice(&fraction.to_be_bytes());

        let parsed = parse_transmit_timestamp_millis(&packet).expect("packet should parse");

        assert_eq!(parsed, unix_secs * 1_000 + 500);
    }

    #[test]
    fn rejects_short_packets() {
        let packet = [0u8; 47];
        let parsed = parse_transmit_timestamp_millis(&packet);

        assert_eq!(parsed, Err(NtpCoreError::InvalidResponse));
    }

    #[test]
    fn rejects_packets_before_unix_epoch() {
        let mut packet = [0u8; 48];
        packet[40..44].copy_from_slice(&1u32.to_be_bytes());

        let parsed = parse_transmit_timestamp_millis(&packet);

        assert_eq!(parsed, Err(NtpCoreError::InvalidResponse));
    }

    #[test]
    fn converts_fraction_to_millis() {
        let mut packet = [0u8; 48];
        let unix_secs = 10u64;
        let ntp_secs = unix_secs + NTP_TIMESTAMP_DELTA;
        let fraction = 0x4000_0000u32;

        packet[40..44].copy_from_slice(&(ntp_secs as u32).to_be_bytes());
        packet[44..48].copy_from_slice(&fraction.to_be_bytes());

        let parsed = parse_transmit_timestamp_millis(&packet).expect("packet should parse");

        assert_eq!(parsed, 10_250);
    }
}

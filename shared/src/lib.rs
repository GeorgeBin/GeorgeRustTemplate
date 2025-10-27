uniffi::include_scaffolding!("shared");

pub fn is_valid_ipv4(ip: String) -> bool {
    core::utils::is_valid_ipv4(ip)
}
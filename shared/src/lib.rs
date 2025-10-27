uniffi::include_scaffolding!("shared");

pub fn is_valid_ipv4(ip: String) -> bool {
    corelib::utils::is_valid_ipv4(ip)
}

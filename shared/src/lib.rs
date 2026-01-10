uniffi::setup_scaffolding!("shared");

#[uniffi::export]
pub fn is_valid_ipv4(ip: String) -> bool {
    corelib::utils::is_valid_ipv4(ip)
}

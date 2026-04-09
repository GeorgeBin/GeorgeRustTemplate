pub fn is_valid_ipv4(ip: String) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }

    for part in parts {
        if part.is_empty() || part.len() > 3 {
            return false;
        }

        // 检查是否为数字且无前导零（除非是单独的0）
        if part.starts_with('0') && part.len() > 1 {
            return false;
        }

        // 解析数字并检查范围
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return false,
        }
    }

    true
}

pub fn extract_tx_version(raw: &str) -> Result<u32, String> {
    let hex = raw.strip_prefix("0x").unwrap_or(raw);

    if hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let ver_hex = &hex[..8];

    if !ver_hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Hex decode error".to_string());
    }

    let mut buf = [0u8; 4];
    for (i, chunk) in ver_hex.as_bytes().chunks(2).enumerate() {
        let s = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8")?;
        buf[i] = u8::from_str_radix(s, 16).map_err(|_| "Failed to parse hex")?;
    }

    Ok(u32::from_le_bytes(buf))
}

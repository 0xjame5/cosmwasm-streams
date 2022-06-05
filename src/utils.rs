use hex;

/// Encodes an array of bytes in hex
///
/// `input`: Bytes
///
/// returns `String`: hex encoded input
pub fn address_pretty(input: &[u8]) -> String {
    format!("0x{}", hex::encode(input))
}

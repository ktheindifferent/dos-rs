use crate::dos;

/// Writes raw bytes directly to the screen.
///
/// # Arguments
///
/// * `bytes` - The bytes to write
#[allow(dead_code)]
pub fn write_bytes(bytes: &[u8]) {
    bytes.iter().for_each(|&b| dos::print_character(b));
}

/// Writes a string to the screen with CP437 encoding.
///
/// # Arguments
///
/// * `s` - The string to write
pub fn write_str(s: &str) {
    s.chars().for_each(|c| dos::print_character(crate::text::cp437::encode_char_lossy(c)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::write_str($($arg)*));
}

use crate::rng::GlobalRng;

/// Generate a random 16-bit number using the global RNG
/// 
/// This function uses the safer RNG implementation that avoids
/// hardcoded memory addresses and provides better documentation.
pub fn random() -> u16 {
    GlobalRng::random()
}

/// Seed the random number generator using the system timer
/// 
/// This function uses DOS interrupt 0x1A to get the timer tick count
/// for backwards compatibility. For better entropy, consider using
/// `seed_random_improved()` which uses DOS interrupt 0x2C.
pub fn seed_random() {
    GlobalRng::seed_from_timer()
}

/// Seed the random number generator using system time for better entropy
/// 
/// This function uses DOS interrupt 0x2C which provides hour, minute,
/// second, and hundredths of second values for better seed quality.
pub fn seed_random_improved() {
    GlobalRng::seed_from_system_time()
}

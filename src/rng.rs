//! Random Number Generator module using XORSHIFT algorithm
//! 
//! This module implements a 16-bit XORSHIFT random number generator.
//! XORSHIFT is a class of pseudorandom number generators that use the
//! XOR and bit shift operations to generate random sequences.
//!
//! The specific variant used here is XORSHIFT with parameters (7, 9, 8)
//! which provides good randomness properties for 16-bit values.

use core::arch::asm;

/// Static storage for the RNG state
/// This replaces the hardcoded memory address [11h] with proper Rust memory management
static mut RNG_STATE: u16 = 0x1234; // Default seed value

/// Random number generator struct
pub struct Rng {
    state: u16,
}

impl Rng {
    /// Creates a new RNG with the given seed
    pub const fn new(seed: u16) -> Self {
        Rng { state: seed }
    }

    /// Creates a new RNG seeded from the system timer using DOS interrupt 0x2C
    /// 
    /// This uses DOS interrupt 0x2C which returns:
    /// - CH = hour (0-23)
    /// - CL = minutes (0-59)  
    /// - DH = seconds (0-59)
    /// - DL = hundredths of seconds (0-99)
    ///
    /// We combine these values to create a seed value.
    pub fn from_system_time() -> Self {
        let seed = unsafe {
            let cx: u16;
            let dx: u16;
            
            // DOS interrupt 0x2C - Get system time
            asm!(
                "mov ah, 0x2C",
                "int 0x21",
                out("cx") cx,
                out("dx") dx,
                out("ax") _,
            );
            
            // CX contains hours (CH) and minutes (CL)
            // DX contains seconds (DH) and hundredths (DL)
            // Combine all time components into a 16-bit seed
            let high = cx;
            let low = dx;
            high ^ low ^ ((dx & 0xFF) * 100) // Extra mixing with hundredths
        };
        
        Rng::new(if seed == 0 { 0x1234 } else { seed })
    }

    /// Generates the next random number using XORSHIFT algorithm
    /// 
    /// The XORSHIFT algorithm works by:
    /// 1. XORing the state with itself shifted left by 7 bits
    /// 2. XORing the result with itself shifted right by 9 bits
    /// 3. XORing the result with itself shifted left by 8 bits
    /// 
    /// This sequence of operations ensures good bit mixing and randomness.
    pub fn next(&mut self) -> u16 {
        // XORSHIFT algorithm with parameters (7, 9, 8)
        // These parameters have been chosen for good randomness properties
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
        self.state ^= self.state << 8;
        
        // Prevent the state from becoming 0 (which would break the generator)
        if self.state == 0 {
            self.state = 0x1234;
        }
        
        self.state
    }
}

/// Global RNG instance using static storage
/// This provides backwards compatibility with the original implementation
pub struct GlobalRng;

impl GlobalRng {
    /// Generates a random number using the global RNG state
    /// 
    /// # Safety
    /// This function uses static mutable state and is not thread-safe.
    /// In a DOS environment, this is generally not an issue as DOS is single-threaded.
    pub fn random() -> u16 {
        unsafe {
            // Apply XORSHIFT algorithm
            RNG_STATE ^= RNG_STATE << 7;
            RNG_STATE ^= RNG_STATE >> 9;
            RNG_STATE ^= RNG_STATE << 8;
            
            // Prevent the state from becoming 0
            if RNG_STATE == 0 {
                RNG_STATE = 0x1234;
            }
            
            RNG_STATE
        }
    }
    
    /// Seeds the global RNG using the system timer (DOS interrupt 0x1A)
    /// 
    /// This uses the original DOS interrupt 0x1A which returns the timer tick count
    /// in CX:DX registers. While not as good as interrupt 0x2C, this maintains
    /// compatibility with the original implementation.
    /// 
    /// # Safety
    /// This function uses static mutable state and is not thread-safe.
    pub fn seed_from_timer() {
        unsafe {
            let seed: u16;
            
            // DOS interrupt 0x1A, function 0 - Get system timer tick count
            asm!(
                "xor ax, ax",
                "int 0x1A",
                out("dx") seed,
                out("ax") _,
                out("cx") _,
            );
            
            RNG_STATE = if seed == 0 { 0x1234 } else { seed };
        }
    }
    
    /// Seeds the global RNG using DOS interrupt 0x2C for better entropy
    /// 
    /// This provides better seed quality than the timer tick method.
    /// 
    /// # Safety
    /// This function uses static mutable state and is not thread-safe.
    pub fn seed_from_system_time() {
        unsafe {
            let cx: u16;
            let dx: u16;
            
            // DOS interrupt 0x2C - Get system time
            asm!(
                "mov ah, 0x2C",
                "int 0x21",
                out("cx") cx,
                out("dx") dx,
                out("ax") _,
            );
            
            // CX contains hours (CH) and minutes (CL)
            // DX contains seconds (DH) and hundredths (DL)
            // Combine all time components into a 16-bit seed
            let seed = cx ^ dx ^ ((dx & 0xFF) * 100);
            
            RNG_STATE = if seed == 0 { 0x1234 } else { seed };
        }
    }
    
    /// Sets the global RNG seed to a specific value
    /// 
    /// # Safety
    /// This function uses static mutable state and is not thread-safe.
    pub fn set_seed(seed: u16) {
        unsafe {
            RNG_STATE = if seed == 0 { 0x1234 } else { seed };
        }
    }
    
    /// Gets the current global RNG state (mainly for testing)
    /// 
    /// # Safety
    /// This function uses static mutable state and is not thread-safe.
    #[cfg(test)]
    pub fn get_state() -> u16 {
        unsafe { RNG_STATE }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_basic() {
        let mut rng = Rng::new(0x1234);
        
        // Generate some numbers and ensure they're different
        let n1 = rng.next();
        let n2 = rng.next();
        let n3 = rng.next();
        
        assert_ne!(n1, n2);
        assert_ne!(n2, n3);
        assert_ne!(n1, n3);
    }
    
    #[test]
    fn test_rng_deterministic() {
        // Two RNGs with the same seed should produce the same sequence
        let mut rng1 = Rng::new(0x5678);
        let mut rng2 = Rng::new(0x5678);
        
        for _ in 0..10 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }
    
    #[test]
    fn test_zero_state_recovery() {
        // Test that the RNG recovers from a zero state
        let mut rng = Rng::new(0);
        let n1 = rng.next();
        assert_ne!(n1, 0);
        assert_eq!(rng.state, 0x1234); // Should reset to default seed
    }
    
    #[test]
    fn test_global_rng() {
        GlobalRng::set_seed(0x9ABC);
        
        let n1 = GlobalRng::random();
        let n2 = GlobalRng::random();
        
        assert_ne!(n1, n2);
    }
    
    #[test]
    fn test_distribution() {
        // Basic test for reasonable distribution
        let mut rng = Rng::new(0xDEAD);
        let mut buckets = [0u32; 16];
        
        for _ in 0..10000 {
            let val = rng.next();
            let bucket = (val >> 12) as usize; // Top 4 bits
            buckets[bucket] += 1;
        }
        
        // Each bucket should have roughly 625 values (10000/16)
        // Allow for some variance
        for count in &buckets {
            assert!(*count > 400 && *count < 850, "Poor distribution: {}", count);
        }
    }
    
    #[test]
    fn test_period_length() {
        // Ensure the RNG has a reasonable period (doesn't repeat too quickly)
        let mut rng = Rng::new(0xBEEF);
        let initial = rng.state;
        
        let mut count = 0;
        loop {
            rng.next();
            count += 1;
            
            if rng.state == initial {
                break;
            }
            
            if count > 65000 {
                // Good enough - period is at least near the maximum for 16-bit
                break;
            }
        }
        
        // XORSHIFT should have a period of 2^16 - 1 for non-zero seeds
        assert!(count > 60000, "Period too short: {}", count);
    }
}
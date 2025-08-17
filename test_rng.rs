// Standalone test file to verify RNG implementation logic
// This tests the XORSHIFT algorithm implementation outside of DOS context

#[derive(Debug)]
struct Rng {
    state: u16,
}

impl Rng {
    fn new(seed: u16) -> Self {
        Rng { state: seed }
    }

    fn next(&mut self) -> u16 {
        // XORSHIFT algorithm with parameters (7, 9, 8)
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

fn test_basic() {
    let mut rng = Rng::new(0x1234);
    
    // Generate some numbers and ensure they're different
    let n1 = rng.next();
    let n2 = rng.next();
    let n3 = rng.next();
    
    println!("Generated numbers: {:#x}, {:#x}, {:#x}", n1, n2, n3);
    
    assert!(n1 != n2, "First two numbers should be different");
    assert!(n2 != n3, "Second and third numbers should be different");
    assert!(n1 != n3, "First and third numbers should be different");
    
    println!("✓ Basic randomness test passed");
}

fn test_deterministic() {
    // Two RNGs with the same seed should produce the same sequence
    let mut rng1 = Rng::new(0x5678);
    let mut rng2 = Rng::new(0x5678);
    
    for i in 0..10 {
        let val1 = rng1.next();
        let val2 = rng2.next();
        assert!(val1 == val2, "Iteration {}: values should match", i);
    }
    
    println!("✓ Deterministic sequence test passed");
}

fn test_zero_recovery() {
    // Test that the RNG recovers from a zero state
    let mut rng = Rng::new(0);
    let n1 = rng.next();
    
    assert!(n1 != 0, "Should recover from zero state");
    assert!(rng.state == 0x1234, "Should reset to default seed");
    
    println!("✓ Zero state recovery test passed");
}

fn test_distribution() {
    // Basic test for reasonable distribution
    let mut rng = Rng::new(0xDEAD);
    let mut buckets = [0u32; 16];
    
    for _ in 0..10000 {
        let val = rng.next();
        let bucket = (val >> 12) as usize; // Top 4 bits
        buckets[bucket] += 1;
    }
    
    println!("Distribution across 16 buckets:");
    for (i, count) in buckets.iter().enumerate() {
        println!("  Bucket {}: {} values", i, count);
        // Each bucket should have roughly 625 values (10000/16)
        // Allow for some variance
        assert!(*count > 400 && *count < 850, "Poor distribution in bucket {}: {}", i, count);
    }
    
    println!("✓ Distribution test passed");
}

fn test_period() {
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
    
    println!("Period length: {}", count);
    
    // XORSHIFT should have a period of 2^16 - 1 for non-zero seeds
    assert!(count > 60000, "Period too short: {}", count);
    
    println!("✓ Period length test passed");
}

fn main() {
    println!("Testing XORSHIFT RNG implementation...\n");
    
    test_basic();
    test_deterministic();
    test_zero_recovery();
    test_distribution();
    test_period();
    
    println!("\n✅ All RNG tests passed successfully!");
}
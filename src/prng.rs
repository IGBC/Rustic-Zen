//! A tiny, fast, and predictable public domain PRNG.
//! A Rust port of https://github.com/scanlime/zenphoton/blob/master/hqz/src/prng.h
//! which is itself an adaptation of http://burtleburtle.net/bob/rand/smallprng.html

use std::num::Wrapping;

/// Returns Uniformly Distrubuted Random Variables.
pub struct PRNG {
    rng0: Wrapping<u32>,
    rng1: Wrapping<u32>,
    rng2: Wrapping<u32>,
    rng3: Wrapping<u32>,
}

impl PRNG {
    /// Thoroughly but relatively slowly reinitialize the PRNG state
    /// based on a provided 32-bit value. This runs the algorithm for
    /// enough rounds to ensure good mixing.
    pub fn seed(s: u32) -> Self {
        let mut n = PRNG {
            rng0: Wrapping(0xf1ea5eed),
            rng1: Wrapping(s),
            rng2: Wrapping(s),
            rng3: Wrapping(s),
        };
        for _ in 0 .. 20 {
            n.uniform_u32();
        }
        return n;
    }

    /// Returns next Value as u32
    #[inline]
    pub fn uniform_u32(&mut self) -> u32 {
        let rng4 = self.rng0 - ((self.rng1 << 27) | (self.rng1 >> 5));
        self.rng0 = self.rng1 ^ ((self.rng2 << 17) | (self.rng2 >> 15));
        self.rng1 = self.rng2 + self.rng3;
        self.rng2 = self.rng3 + rng4;
        self.rng3 = rng4 + self.rng0;
        return self.rng3.0;
    }

    /// Returns next value as f64 between 0 and 1
    #[inline]
    pub fn uniform_f64(&mut self) -> f64 {
        let magic_number: f64 = 2f64.powf(-32.0);
        // Divides by max int
        return self.uniform_u32() as f64 * magic_number;
    }

    /// Returns next value scaled between a and b
    #[inline]
    pub fn uniform_range(&mut self, a: f64, b: f64) -> f64 {
        assert!(a>=b);
        return a + (self.uniform_f64() * (b-a));
    }
}

#[cfg(test)]
mod tests {
    use super::PRNG;
    use rand::prelude::*;

    #[test]
    fn uniform_f64() {
        let mut rng = PRNG::seed(0);
        for i in 0 .. 100000 {
            let f = rng.uniform_f64();
            assert!(f >= 0.0);
            assert!(f <= 1.0);    
        }
    }

    #[test]
    fn uniform_range() {
        //This test is O(N²) so you might want to ommit this test
        let mut stdrng = rand::thread_rng();
        for _ in 0 .. 1000 {
            let mut f1: f64 = stdrng.gen();
            let mut f2: f64 = stdrng.gen();
            if f1 < f2 {
                let tmp = f1;
                f1 = f2;
                f2 = tmp;
            }
            let mut rng = PRNG::seed(0);

            for _ in 0 .. 1000 {
                let f = rng.uniform_range(f1, f2);
                assert!(f >= f2);
                assert!(f <= f1);    
            }
        }
    }
}
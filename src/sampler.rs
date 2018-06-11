use prng::PRNG;
use spectrum::blackbody_wavelength;
use std::f64;

#[derive(Clone, Copy)]
pub enum Sample {
        Constant(f64),
        Blackbody(f64),
        Range(f64,f64),
    }

/// Samplers stochastically sample a value, which may be:
///  - a constant
///  - linear range between two values
///  - A Blackbody Curve of temperature K
pub struct Sampler {
    random: PRNG
}

impl Sampler {
    pub fn new(seed: u32) -> Self {
        Sampler {
            random: PRNG::seed(seed),
        }
    }

    /// Returns next value of this sampler
    pub fn val(&mut self, sample: Sample) -> f64 {
        match sample {
            Sample::Constant(i) => i,
            Sample::Blackbody(k) => blackbody_wavelength(k, self.random.uniform_f64()),
            Sample::Range(l, u) => self.random.uniform_range(l, u), 
        }
    }

    // Returns upper and lower bounds of this Sample
    pub fn bounds(sample: Sample) -> (f64, f64) {
        match sample {
            Sample::Constant(i) => (i, i),
            //Blackbody(k) => (k, k), //TODO Actually work out what these are.
            Sample::Range(l,u) => (l, u),
            _ => (f64::MIN, f64::MAX),
        }
    }
}
use prng::PRNG;
use spectrum::blackbody_wavelength;

enum SamplerType {
        Constant(f64),
        Blackbody(f64),
        Range(f64,f64),
    }

/// Samplers stochastically sample a value, which may be:
///  - a constant
///  - linear range between two values
///  - A Blackbody Curve of temperature K
struct Sampler {
    random: PRNG,    
    type: SamplerType
}

impl Sampler {
    pub fn new(seed: u32, type: SamplerType) -> Self {
        Sampler {
            random: PRNG::seed(seed);
            type,
        }
    }

    /// Returns next value of this sampler
    pub fn val(&mut self) -> f64 {
        match self.type {
            Constant(i) => i,
            Blackbody(k) => blackbody_wavelength(k, self.random.uniform_f64()),
            Range(l, u) => self.random.uniform_range(l, u), 
        }
    }

    // Returns upper and lower bounds of this SamplerType
    pub fn bounds(t: SamplerType) -> (f64, f64) {
        match t {
            Constant(i) => (i, i),
            //Blackbody(k) => (k, k), //TODO Actually work out what these are.
            Range(l,u) => (l, u),
            _ => (f64::min_value(), f64::max_value()),
        }
    }
}
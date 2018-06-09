use prng::PRNG;
use spectrum::blackbody_wavelength;
use std::f64;

pub enum SamplerType {
        Constant(f64),
        Blackbody(f64),
        Range(f64,f64),
    }

/// Samplers stochastically sample a value, which may be:
///  - a constant
///  - linear range between two values
///  - A Blackbody Curve of temperature K
pub struct Sampler {
    random: PRNG,    
    typ: SamplerType
}

impl Sampler {
    pub fn new(seed: u32, typ: SamplerType) -> Self {
        Sampler {
            random: PRNG::seed(seed),
            typ,
        }
    }

    /// Returns next value of this sampler
    pub fn val(&mut self) -> f64 {
        match self.typ {
            SamplerType::Constant(i) => i,
            SamplerType::Blackbody(k) => blackbody_wavelength(k, self.random.uniform_f64()),
            SamplerType::Range(l, u) => self.random.uniform_range(l, u), 
        }
    }

    // Returns upper and lower bounds of this SamplerType
    pub fn bounds(t: SamplerType) -> (f64, f64) {
        match t {
            SamplerType::Constant(i) => (i, i),
            //Blackbody(k) => (k, k), //TODO Actually work out what these are.
            SamplerType::Range(l,u) => (l, u),
            _ => (f64::MIN, f64::MAX),
        }
    }
}
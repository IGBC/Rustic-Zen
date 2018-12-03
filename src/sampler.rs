use prng::PRNG;
use spectrum::blackbody_wavelength;
use std::f64;

#[derive(Clone, Copy, Debug)]
/// Samples stochastically sample a value, which may be:
///  - a constant
///  - linear range between two values
///  - A Blackbody Curve of temperature K
pub enum Sample {
        Constant(f64),
        Blackbody(f64),
        Range(f64,f64),
    }

impl Sample {
    /// Returns next value of this sampler
    pub fn val(&self, sampler: &mut PRNG) -> f64 {
        match self {
            Sample::Constant(i) => i.clone(),
            Sample::Blackbody(k) => blackbody_wavelength(k.clone(), sampler.uniform_f64()),
            Sample::Range(l, u) => sampler.uniform_range(l.clone(), u.clone()), 
        }
    }

    // Returns upper and lower bounds of this Sample
    pub fn bounds(&self) -> (f64, f64) {
        match self {
            Sample::Constant(i) => (i.clone(), i.clone()),
            //Sample::Blackbody(k) => (k, k), //TODO Actually work out what these are.
            Sample::Range(l,u) => (l.clone(), u.clone()),
            _ => (f64::MIN, f64::MAX),
        }
    }
}
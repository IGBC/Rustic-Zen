//! This module provides rustic-zen's sampler implementation, which
//! is needed for instanciating lights and objects.

use prng::PRNG;
use spectrum::blackbody_wavelength;
use std::f64;

#[derive(Clone, Copy, Debug)]
/// Samples a stochastically sampled value, which may be:
///  - a constant
///  - linear range between two values
///  - A Blackbody Curve of temperature K
pub enum Sample {
    /// A constant Value
    Constant(f64),
    /// A Blackbody Curve of given temprature
    ///
    /// Realistically only useful for light wavelengths.
    Blackbody(f64),
    /// A value sampled linierly from the given range.
    ///
    /// The larger value must be the first argument or it will panic,
    /// rustic-zen is about going fast not holding your hand and these samplers
    /// are in the critical path
    Range(f64, f64),
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

    /// Returns upper and lower bounds of this Sample.
    ///
    /// # Example
    /// ```
    /// use rustic_zen::prelude::Sample;
    ///
    /// let f = Sample::Range(1.0, 0.0);
    /// let (upper, lower) = f.bounds();
    /// assert_eq!(lower, 0.0);
    /// assert_eq!(upper, 1.0);
    /// ```
    pub fn bounds(&self) -> (f64, f64) {
        match self {
            Sample::Constant(i) => (i.clone(), i.clone()),
            //Sample::Blackbody(k) => (k, k), //TODO Actually work out what these are.
            Sample::Range(u, l) => (u.clone(), l.clone()),
            _ => (f64::MIN, f64::MAX),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sample;
    use prng::PRNG;

    use rand::prelude::*;

    #[test]
    fn const_bounds() {
        // test bounds with 1000 random numbers
        for _ in 0..1000 {
            let mut stdrng = rand::thread_rng();
            let f: f64 = stdrng.gen();
            let s = Sample::Constant(f);
            let (a, b) = s.bounds();
            assert_eq!(a, b);
            assert_eq!(a, f);
            assert_eq!(b, f);
        }
    }

    #[test]
    fn range_bounds() {
        for _ in 0..1000 {
            let mut stdrng = rand::thread_rng();
            let a: f64 = stdrng.gen();
            let b: f64 = stdrng.gen();
            let s = Sample::Range(a, b);

            let (c, d) = s.bounds();
            assert_eq!(a, c);
            assert_eq!(b, d);
        }
    }

    #[test]
    fn val_const() {
        let mut rng = PRNG::seed(0);

        let mut stdrng = rand::thread_rng();
        let f: f64 = stdrng.gen();
        let s = Sample::Constant(f);

        for _ in 0..1000 {
            let y = s.val(&mut rng);
            assert_eq!(y, f);
        }
    }

    #[test]
    fn val_range() {
        let mut rng = PRNG::seed(0);

        let mut stdrng = rand::thread_rng();
        let mut f1: f64 = stdrng.gen();
        let mut f2: f64 = stdrng.gen();
        if f1 < f2 {
            let tmp = f1;
            f1 = f2;
            f2 = tmp;
        }
        let s = Sample::Range(f1, f2);

        // This does actually run 100,000 times despite finishing so quickly
        for _ in 0..100000 {
            let y = s.val(&mut rng);
            assert!(y <= f1);
            assert!(y >= f2);
        }
    }

    #[test]
    fn blackbody_works() {
        let mut rng = PRNG::seed(0);
        // Get a sampled value from the range of valid wavelenghts
        let w = Sample::Range(780.0, 360.0);

        // create sample with random wavelenght
        let s = Sample::Blackbody(w.val(&mut rng));

        // Check s can be sampled without panicing
        s.val(&mut rng);
    }

    #[test]
    fn blackbody_white_light() {
        let mut rng = PRNG::seed(0);
        let s = Sample::Blackbody(0.0);
        // Check s can be sampled without panicing
        s.val(&mut rng);
    }
}

use geom::Vector;
use prng::PRNG;
use std::f64::consts::PI;

pub trait Material {
    /**
     * Computes fate of this ray,
     * returns option wrapping direciton of new ray,
     * none if no new ray.
     */
    fn outcome(
        &self,
        direction: &Vector,
        normal: &Vector,
        wavelength: f64,
        alpha: f64,
        rng: &mut PRNG,
    ) -> Option<Vector>;
}

#[derive(Copy, Clone)]
pub struct HQZLegacy {
    d: f64,
    r: f64,
    t: f64,
}

impl HQZLegacy {
    pub fn new(d: f64, r: f64, t: f64) -> Self {
        if d + r + t > 1.0 {
            panic!("HQZ Legacy shader cooefficents > 1.0");
        }

        Self { d, r, t }
    }
}

impl Material for HQZLegacy {
    fn outcome(
        &self,
        direction: &Vector,
        normal: &Vector,
        _wavelength: f64,
        _alpha: f64,
        rng: &mut PRNG,
    ) -> Option<Vector> {
        let f = rng.uniform_f64();

        if f <= self.d {
            let angle = rng.uniform_range(2.0 * PI, 0.0);
            return Some(Vector {
                x: f64::cos(angle),
                y: f64::sin(angle),
            });
        }

        if f <= self.d + self.r {
            let angle = direction.reflect(normal);
            return Some(angle);
        }

        if f <= self.d + self.r + self.t {
            let angle = direction.clone();
            return Some(angle);
        }

        None
    }
}

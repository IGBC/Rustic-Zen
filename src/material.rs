use geom::Vector;
use pcg_rand::Pcg64Fast;
use std::f64::consts::PI;
use rand::prelude::*;

/// Shader Trait
///
/// This trait exposes a single `outcome` method, this is used by Rustic Zen
/// to calulate the result of a ray interacting with an object.
///
/// The trait is exposed to allow library users to define their own shaders.
/// A lot of information is provided to the outcome method, to give the user
/// options on how they want their shader to work.
///
/// Wavelength is provided instead of colour to encourage the design of physically based
/// shaders. Colour can be calculated from the wavelength using Rustic's spectrum module.
pub trait Material {
    /**
     * This function computes the outcome of a ray to object interaction.
     *
     * It returns a wrapped vector of the direction of the bounced ray.
     * If the ray is absorbed then it returns `None`.
     *
     * # Parameters:
     *  - __direction__: Vector of the direction of the inbound ray.
     *  - __normal__: Computed normal to the hit surface. This can be used in `direction.reflect(normal)`, to get a mirror reflection.
     *  - __wavelength__: Wavelength of inbound ray (no way to change this for the outbound ray, sorry).
     *  - __alpha__: how far along the object the inbound ray hit. clamped 0.0 to 1.0
     *  - __rng__: random number generator for use during the function, (don't spawn your own, way to slow.)
     */
    fn outcome(
        &self,
        direction: &Vector,
        normal: &Vector,
        wavelength: f64,
        alpha: f64,
        rng: &mut Pcg64Fast,
    ) -> Option<Vector>;
}

/// Reference / Legacy implementation of Material trait.
///
/// This implementation models the behavour of shaders from the original HQZ.
#[derive(Copy, Clone)]
pub struct HQZLegacy {
    d: f64,
    r: f64,
    t: f64,
}

impl HQZLegacy {
    /// Creates new instance with provided parameters.
    ///
    /// # Parameters:
    ///  - d: probability ray will be diffuse reflected
    ///  - r: probability ray will be specularly reflected
    ///  - t: probability ray will be transmitted through surface
    ///
    /// Probablitity ray will be absorbed is defined from `1.0 - d - r - t`
    pub fn new(d: f64, r: f64, t: f64) -> Self {
        if d + r + t > 1.0 {
            panic!("HQZ Legacy shader cooefficents > 1.0");
        }
        Self { d, r, t }
    }

    /// Creates a new instance with default arguments,
    /// This is used in many of the the tests and examples.
    /// d: 0.1, r: 0.4, t: 0.4, a: 0.1
    pub fn default() -> Self {
        Self {
            d: 0.1,
            r: 0.4,
            t: 0.4,
        }
    }
}

impl Material for HQZLegacy {
    fn outcome(
        &self,
        direction: &Vector,
        normal: &Vector,
        _wavelength: f64,
        _alpha: f64,
        rng: &mut Pcg64Fast,
    ) -> Option<Vector> {
        let f: f64 = rng.gen_range(0.0,1.0);

        if f <= self.d {
            let angle = rng.gen_range(0.0, 2.0 * PI);
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

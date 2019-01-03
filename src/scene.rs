use sampler::Sample;

/// Data only struct which defines a Light Source
/// 
/// # Examples
/// Make a warm white small round light:
/// ```
/// Light {
///     power: Sample::Constant(1.0),
///     x: Sample::Constant(512.0),
///     y: Sample::Constant(512.0),
///     polar_angle: Sample::Constant(0.0),
///     polar_distance: Sample::Constant(0.0),
///     ray_angle: Sample::Range(360.0, 0.0),
///     wavelength: Sample::Blackbody(5800.0),
/// };
/// ```
#[derive(Clone, Copy)]
pub struct Light {
    /// Brightness of this light relative to other lights in the scene
    pub power: Sample,
    /// x coordinate of the light.
    pub x: Sample,
    /// y coordinate of the light.
    pub y: Sample,
    /// distance from x,y that photon will spawn
    pub polar_distance: Sample,
    /// angle that polar_distance will be at 
    pub polar_angle: Sample,
    /// Angle which spawned ray will be at
    pub ray_angle: Sample,
    /// Wavelength of spawned ray
    pub wavelength: Sample,
}


/// Data only struct which defines a Material
/// 
/// **Note:** This struct is artistically limiting 
/// and due to be replaced with a trait.
/// 
/// # Usage:
/// The three fields are probablities, if they sum
/// to less than 1 then the remainder is the probability
/// the ray is absorbed by the material.
/// 
/// Don't set the values to a sum greater than 1 that's very
/// undefined behaviour.
/// 
/// # Example
/// Basic material:
/// ```
/// let m = Material { d: 0.3, r: 0.3, t: 0.3 }; // absorbsion is 0.1
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Material {
    /// Diffuse reflection - Random angle.
    pub d: f64, 
    /// Transparent interaction - Incident angle copied.
    pub t: f64, 
    /// Specular reflection - Ray relects on normal.
    pub r: f64,
}

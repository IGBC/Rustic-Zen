use sampler::Sample;

/// Data only struct which defines a Light Source
/// 
/// # Examples
/// Make a warm white small round light:
/// ```
/// use rustic_zen::prelude::{Light, Sample};
/// 
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
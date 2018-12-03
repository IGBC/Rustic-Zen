use sampler::Sample;
use object::Object;

#[derive(Clone, Copy)]
pub struct Light {
    pub power: Sample,
    pub x: Sample,
    pub y: Sample,
    pub polar_angle: Sample,
    pub polar_distance: Sample,
    pub ray_angle: Sample,
    pub wavelength: Sample,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    d: f64,
    t: f64,
    r: f64,
}

#[derive(Clone, Copy)]
pub struct Viewport {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Clone)]
pub struct Scene<'a> {
    pub resolution_x: usize,
    pub resolution_y: usize,
    pub viewport: Viewport,

    pub seed: u32,

    pub rays: usize,
    pub timelimit: usize,

    pub exposure: f64,
    pub gamma: f64,

    pub lights: Vec<Light>,
    pub objects: Vec<Object<'a>>,
    pub materials: Vec<Material>,
}

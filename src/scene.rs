use sampler::Sample;
use object::Object;

#[derive(Clone, Copy)]
pub struct Light {
    power: Sample,
    x: Sample,
    y: Sample,
    polar_angle: Sample,
    polar_distance: Sample,
    ray_angle: Sample,
    wavelength: Sample,
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
    resolution_x: usize,
    resolution_y: usize,
    viewport: Viewport,

    seed: u32,

    rays: usize,
    timelimit: usize,

    exposure: f64,
    gamma: f64,

    lights: Vec<Light>,
    objects: Vec<Object<'a>>,
    materials: Vec<Material>,
}


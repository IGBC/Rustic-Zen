use sampler::Sample;
use object::Object;
use aabb_quadtree::geom::Rect;

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
    pub d: f64,
    pub t: f64,
    pub r: f64,
}

#[derive(Clone)]
pub struct Scene {
    pub resolution_x: usize,
    pub resolution_y: usize,
    pub viewport: Rect,

    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

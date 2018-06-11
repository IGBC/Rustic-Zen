use sampler::Sample;

#[derive(Clone, Copy)]
struct Light {
    power: Sample,
    x: Sample,
    y: Sample,
    polar_angle: Sample,
    polar_distance: Sample,
    ray_angle: Sample,
    wavelength: Sample,
}

#[derive(Clone, Copy)]
enum Object<'a> {
    Line{
        material: &'a Material,
        x0: Sample,
        y0: Sample,
        dx: Sample,
        dy: Sample,
    },
    Curve{
        material: &'a Material,
        x0: Sample,
        y0: Sample,
        a0: Sample,
        dx: Sample,
        dy: Sample,
        da: Sample,
    },
}

#[derive(Clone, Copy)]
struct Material {
    d: f64,
    t: f64,
    r: f64,
}

#[derive(Clone, Copy)]
struct Viewport {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Clone)]
struct Scene<'a> {
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
use scene::Light;
use object::Object;
use image::Image;
use prng::PRNG;
use sampler::Sample;
use ray::Ray;
use aabb_quadtree::geom::Rect;

pub struct Renderer {
    lights: Vec<Light>,
    objects: Vec<Object>,
    seed: u32, //current seed
    total_light_power: f64,
    resolution_x: usize,
    resolution_y: usize,
    viewport: Rect,
}


impl Renderer {
    pub fn new(resolution_x: usize, resolution_y: usize, viewport: Rect) -> Self {
        Renderer {
            seed: 0,
            lights: vec!(),
            objects: vec!(),
            viewport,
            resolution_x,
            resolution_y,
            total_light_power: 0.0,
        }
    }

    pub fn with_light(mut self, light: Light) -> Self {
        self.total_light_power += light.power.bounds().1;
        self.lights.push(light);
        self
    }

    pub fn with_object(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self
    }
    
    fn choose_light(&self, rng: &mut PRNG) -> &Light {
        let sample = Sample::Range(self.total_light_power, 0.0);
        let threshold = sample.val(rng);
        let mut sum: f64 = 0.0;
        for light in &self.lights {
            sum += light.power.val(rng);
            if threshold <= sum {
                return light;
            }
        }
        return self.lights.last().expect("Scene has no lights");
    }

    fn trace_ray(&self, img: &mut Image, rng: &mut PRNG) {
        let l = self.choose_light(rng);
        let mut ray = Some(Ray::new(l, rng));
        while ray.is_some() {
            ray = ray.unwrap().collision_list(&self.objects, self.viewport, img, rng);
        }
    }

    pub fn render(self, rays: usize) -> Image {
        let mut rng = PRNG::seed(self.seed);
        let mut image = Image::new(self.resolution_x, self.resolution_y);
        for _i in 0..rays {
            self.trace_ray(&mut image, &mut rng);
        }

        // return rendered image.
        image
    }
}

#[cfg(test)]
mod tests {
    use super::Renderer;
    use object::Object;
    use scene::{Material, Light};
    use sampler::Sample;
    use aabb_quadtree::geom::{Point,Rect};
    
    #[test]
    fn nrt_works(){
        let m = Material {
            d: 0.3, r: 0.3, t: 0.3, 
        };

        let obj = Object::Line{
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: m.clone(),
        };

        let l = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(10.0),
            y: Sample::Constant(10.0),
            polar_angle: Sample::Range(360.0, 0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        let viewport = Rect::from_points(&Point{x: 0.0,y: 0.0}, &Point{x: 160.0,y: 90.0});

        let r = Renderer::new(1920, 1080, viewport).with_light(l).with_object(obj);
        r.render(100);
    }
}



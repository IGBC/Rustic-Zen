use scene::{Scene, Light};
use image::Image;
use prng::PRNG;
use sampler::Sample;
use ray::Ray;

pub struct Renderer {
    scene: Scene,
    seed: u32, //current seed
    total_light_power: f64
}


impl Renderer {
    fn new(scene: Scene) -> Self {
        let mut total_light_power = 0.0;
        for i in scene.lights.iter() {
            total_light_power += i.power.bounds().1;
        }
        let seed = scene.seed;

        let r = Renderer {
            scene,
            seed,
            total_light_power
        };

        return r; 
    }
    
    fn choose_light(&self, rng: &mut PRNG) -> &Light {
        let sample = Sample::Range(self.total_light_power, 0.0);
        let threshold = sample.val(rng);
        let mut sum: f64 = 0.0;
        for light in &self.scene.lights {
            sum += light.power.val(rng);
            if threshold <= sum {
                return light;
            }
        }
        return self.scene.lights.last().expect("Scene has no lights");
    }

    fn trace_ray(&self, img: &mut Image, rng: &mut PRNG) {
        let l = self.choose_light(rng);
        let mut ray = Some(Ray::new(l, rng));
        while ray.is_some() {
            ray = ray.unwrap().collision_list(&self.scene.objects, self.scene.viewport, img, rng);
        }
    }

    pub fn render(self) -> Image {
        let mut rng = PRNG::seed(self.scene.seed);
        let mut image = Image::new(self.scene.resolution_x, self.scene.resolution_y);
        for _i in 0..self.scene.rays {
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
    use scene::{Material, Light, Scene};
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
            polar_distance: Sample::Constant(1.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(0.0),
        };

        let s = Scene {
            resolution_x: 1920,
            resolution_y: 1080,
            viewport: Rect::from_points(&Point{x: 0.0,y: 0.0},&Point{x: 160.0,y: 90.0}),
            seed: 0,
            rays: 100,
            timelimit: 0,

            exposure: 1.0,
            gamma: 1.0,

            lights: vec!(l),
            objects: vec!(obj),
            materials: Vec::new(),
        };

        let r = Renderer::new(s);
        let img = r.render();
    }
}



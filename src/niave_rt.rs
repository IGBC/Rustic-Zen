use scene::{Scene, Light};
use image::Image;
use prng::PRNG;
use sampler::Sample;
use ray::Ray;

pub struct Renderer<'a> {
    scene: Scene<'a>,
    image: Image,
    seed: u32, //current seed
    total_light_power: f64
}


impl<'a> Renderer<'a> {
    fn choose_light(&self, rng: &mut PRNG) -> &Light {
        let sample = Sample::Range(0.0, self.total_light_power);
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

    pub fn render(&self) -> Image {
        let mut img = Image::new(self.scene.resolution_x, self.scene.resolution_y);
        let mut rng = PRNG::seed(self.scene.seed);
        for _i in 0..self.scene.rays {
            self.trace_ray(&mut img, &mut rng);
        }

        // return rendered image.
        img
    }
}


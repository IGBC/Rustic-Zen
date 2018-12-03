use scene::{Scene, Light};
use image::Image;
use prng::PRNG;
use sampler::Sample;
use object::Object;
use ray::Vec2;
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
        return self.scene.lights.last().unwrap();
    }

    fn trace_ray(&self, rng: &mut PRNG) {
        let l = self.choose_light(rng);
        //TODO: init ray

    }
}


fn trace_ray() {
    //get start pos
    //collide with all opjects
    //
}

pub fn render(s: Scene) -> Image {
    let mut img = Image::new(s.resolution_x, s.resolution_y);
    
    
    // return rendered image.
    img
}
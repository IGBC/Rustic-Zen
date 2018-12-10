use scene::{Scene, Light};
use image::Image;
use prng::PRNG;
use sampler::Sample;
use ray::Ray;
use aabb_quadtree::QuadTree;
use object::Object;

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

    fn trace_ray(&self, rng: &mut PRNG) {
        let l = self.choose_light(rng);
        let ray = Ray::new(l, rng);
        for _bounces in 0 .. 1000 {
            for obj in self.scene.objects.iter() {
                let mut i: Vec<Ray> = Vec::new();
                match ray.bounce(obj, rng) {
                    Some(r) => { i.push(r) },
                    None => {},
                };
                if i.is_empty() {
                    return;
                }
            }
        }
    }
}

pub fn render(s: &Scene) -> Image {
    let mut img = Image::new(s.resolution_x, s.resolution_y);
    let mut qt: QuadTree<&Object> = QuadTree::default(s.viewport);

    for o in s.objects.iter() {
        qt.insert_with_box(o, o.bounds());
    }

    // return rendered image.
    img
}


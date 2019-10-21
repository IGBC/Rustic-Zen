use geom::{Point, Rect};
use image::Image;
use object::Object;
use ray::{Ray, HitData};
use sampler::Sample;
use std::thread;
use std::sync::{mpsc};
use plumbing::{CompleteRay};
use pcg_rand::Pcg64Fast;
use rand::prelude::*;
use pcg_rand::seeds::PcgSeeder;

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

/// Holds scene Configuration and logic
pub struct Scene {
    lights: Vec<Light>,
    objects: Vec<Object>,
    seed: u128, //current seed
    total_light_power: f64,
    resolution_x: usize,
    resolution_y: usize,
    viewport: Rect,
}

impl Scene {
    /// Creates new Renderer ready for defining a scene.
    pub fn new(resolution_x: usize, resolution_y: usize) -> Self {
        Self {
            // 128 bit numbers are getting a bit too long even in hex
            seed: 0xDEADBEEF00000000F00DBABE00000000, //It just can't be 0
            lights: vec![],
            objects: vec![],
            viewport: Rect::from_points(&Point{ x: 0.0, y: 0.0 }, &Point { x: resolution_x as f64, y: resolution_y as f64 }),
            resolution_x,
            resolution_y,
            total_light_power: 0.0,
        }
    }

    /// Adds Light to the scene - Chainable varient
    pub fn with_light(mut self, light: Light) -> Self {
        self.total_light_power += light.power.bounds().1;
        self.lights.push(light);
        self
    }

    /// Adds object to the scene - Chainable varient
    pub fn with_object(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }

    /// Sets the seed for the scene random number generator - Chainable varient
    pub fn with_seed(mut self, seed: u128) -> Self {
        if seed == 0 {
            panic!("Sorry a seed of 0 causes a PCG failure, please try something else");
        }
        self.seed = seed;
        self
    }

    fn choose_light(&self, rng: &mut Pcg64Fast) -> &Light {
        let sample = Sample::Range(self.total_light_power, 0.0);
        let threshold = sample.val(rng);
        let mut sum: f64 = 0.0;
        for light in self.lights {
            sum += light.power.val(rng);
            if threshold <= sum {
                return &light
            }
        }
        return &(self.lights.last().expect("Scene has no lights"));
    }

    fn trace_ray(&self, img: &mut Image, rng: &mut Pcg64Fast) {
        let l = self.choose_light(rng);
        let mut ray = Some(Ray::new(l, rng));
        // while ray.is_some() {
        //     ray = ray
        //         .unwrap()
        //         .collision_list(&self.objects, self.viewport, img);
        // }
    }

    /// Starts the ray tracing process.
    ///
    /// Naturally this call is very expensive. It also consumes the Renderer
    /// and returns an Image class containing the rendered image data.
    pub fn render(self, rays: usize) -> Image {
        let (dispatch_tx, dispatch_rx) = mpsc::channel::<Ray>();
        let (rasterise_tx, rasterise_rx) = mpsc::channel::<CompleteRay>();
        let (shader_tx, shader_rx) = mpsc::channel::<HitData>();

        let mut rng = Pcg64Fast::from_seed(PcgSeeder::seed(self.seed));
        
        let mut image = Image::new(self.resolution_x, self.resolution_y, self.total_light_power);

        let seed = self.seed;
        let lights = &self.lights;
        let total_light_power = self.total_light_power;

        // thread::spawn(move || {
        //     let tx = dispatch_tx.clone(); 
        //     let mut rng = PRNG::seed(seed);
        //     for _i in 0..rays {
        //         let l = Self::choose_light(lights, total_light_power, &mut rng);
        //         let mut ray = Ray::new(&l, &mut rng);
        //         tx.send(ray).unwrap();
        //     }
        // });

        let seed = self.seed;
        let viewport = self.viewport;
        let objects = &self.objects;

        // thread::spawn(move || {
        //     let mut rng = PRNG::seed(seed);
        //     let thread_tx = dispatch_tx.clone();
        //     for ray in dispatch_rx {
        //         let (r, hit) = ray.collision_list(objects, viewport, &mut rng);
        //         match hit {
        //             Some(h) => {
        //                 rasterise_tx.send(CompleteRay {
        //                     start: ray.get_origin().clone(),
        //                     end: h,
        //                     wavelength: ray.get_wavelength(),
        //                 }).unwrap();
        //             },
        //             None => {},
        //         };
        //         match r {
        //             Some(r) => {
        //                 thread_tx.send(r).unwrap();
        //             },
        //             None => {}
        //         }
        //     }
        // });

        for comp_ray in rasterise_rx {
            image.draw_line(comp_ray.wavelength, comp_ray.start.x, comp_ray.start.y, comp_ray.end.x, comp_ray.end.y);
        }

        // return rendered image.
        image
    }
}

#[cfg(test)]
mod tests {
    use super::Scene;
    use material::HQZLegacy;
    use object::Object;
    use sampler::Sample;
    use scene::Light;

    #[test]
    fn nrt_works() {
        let m = Box::new(HQZLegacy::new(0.3, 0.3, 0.3));

        let obj = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: m,
        };

        let l = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(10.0),
            y: Sample::Constant(10.0),
            polar_angle: Sample::Range(360.0, 0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        let r = Scene::new(1920, 1080)
            .with_light(l)
            .with_object(obj);
        r.render(100);
    }

    #[test]
    #[should_panic]
    fn seed_eq_zero() {
        let _r = Scene::new(1920, 1080).with_seed(0);
    }
}

//! Photon Garden Raytracer Impementation.
//!
//! Rustic Zen renders artworks from a scene definition by simulating individual
//! photons and tracing their path as they bounce through a 2D. space.
//!
//! Photons are generated by Lights and interact with Objects. Each interaction
//! results in either the photon being absorbed, or being allowed to continue with
//! a new direction, based on the rules defined by the Object's Material.
//!
//! # Example usage:
//! ```
//! extern crate rustic_zen;
//! use rustic_zen::prelude::*;
//!
//! fn main() {
//!     // Set up constants.
//!     let width: f64 = 1920.0;
//!     let height: f64 = 1080.0;
//!     let rays = 10_000;
//!     // This would be better but these doctests have to run in reasonable time
//!     // let rays = ((width * height).round() / 2.0) as usize;
//!
//!     // Build a basic Material
//!     let m = Box::new(HQZLegacy::new(0.3, 0.3, 0.3));
//!
//!     // Build a basic Object
//!     let o = Object::Line {
//!         x0: Sample::Constant(0.0),
//!         y0: Sample::Constant(height*0.75),
//!         dx: Sample::Constant(width),
//!         dy: Sample::Constant(0.0),
//!         material: m,
//!     };
//!
//!     // Build a basic Light
//!     let l = Light{
//!         power: Sample::Constant(1.0),
//!         x: Sample::Constant(width/2.0),
//!         y: Sample::Constant(height/2.0),
//!         polar_angle: Sample::Constant(0.0),
//!         polar_distance: Sample::Constant(0.0),
//!         ray_angle: Sample::Range(360.0, 0.0),
//!         wavelength: Sample::Blackbody(4500.0),
//!     };
//!
//!     // Construct a renderer object and add the light and object to it.
//!     let s = Scene::new(width as usize, height as usize).with_object(o).with_light(l);
//!     // Render Image
//!     println!("Tracing Rays");
//!     let image = s.render(rays);
//!
//!     // Output the Image as a Vec<u8>
//!     println!("Serializing!");
//!     let data = image.to_rgb8(0.7, 1.2);
//!     
//!     // Do Export to a PNG or whatever you want here.
//! }
//! ```
//!
#![warn(missing_docs)]

#[cfg(test)]
extern crate png;
extern crate pcg_rand;
extern crate rand;

pub mod geom;

mod material;
mod object;
mod sampler;
mod scene;
mod plumbing;
mod colliderpool;
mod renderer;
mod shaderpool;

/// This prelude contains everything to quickstart using Rustic Zen.
pub mod prelude {
    pub use geom::{Point};
    pub use material::{HQZLegacy, Material};
    pub use object::Object;
    pub use sampler::Sample;
    pub use scene::{Light, Scene};
}

// Rexport everything for documentation use.
pub use material::{HQZLegacy, Material};
pub use object::Object;
pub use sampler::Sample;
pub use scene::{Light, Scene};
pub use image::Image;

mod image;
mod ray;
mod spectrum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // For reading and opening files
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;
    // To use encoder.set()
    use png::HasParameters;

    //Scene Parameters
    use object::Object;
    use sampler::Sample;
    use scene::Light;
    use scene::Scene;

    use material::HQZLegacy;

    #[test]
    fn png_test() {
        let width = 1024.0;
        let height = 1024.0;

        let l = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(512.0),
            y: Sample::Constant(512.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(6900.0),
        };

        let r = Scene::new(width as usize, height as usize).with_light(l);
        let image = r.render(10_000);

        let data = image.to_rgb8(0.5, 0.5);
        //let data = image.dumb_to_rgb8();

        let path = Path::new(r"lib.png_test.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 1024, 1024);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap(); // Save
    }

    #[test]
    fn png_test_2() {
        let width: f64 = 1024.0;
        let height: f64 = 1024.0;
        let rays = (width * height / 100.0).round() as usize;

        let m = Box::new(HQZLegacy::default());

        let o = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(height * 0.75),
            dx: Sample::Constant(width),
            dy: Sample::Constant(0.0),
            material: m,
        };

        let l = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(width / 2.0),
            y: Sample::Constant(height / 2.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(6900.0),
        };

        let r = Scene::new(width as usize, height as usize)
            .with_light(l)
            .with_object(o);
        let image = r.render(rays);

        let data = image.to_rgb8(0.5, 0.5);
        //let data = image.dumb_to_rgb8();

        let path = Path::new(r"lib.png_test_2.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 1024, 1024);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap(); // Save
    }
}

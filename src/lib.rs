#![allow(dead_code)]
//#![warn(missing_docs)]
//#![feature(cmp_partial)]
//#![warn(dead_code)]
extern crate aabb_quadtree;

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate png;

mod sampler;
mod spectrum;
mod prng;
mod image;
mod scene;
mod object;
//mod raytrace;
mod niave_rt;
mod ray;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // To use encoder.set()
    use png::HasParameters;

    //Scene Parameters
    use scene::{Light, Scene};
    use sampler::Sample;
    use niave_rt::Renderer;

    use aabb_quadtree::geom::{Point, Rect};

    #[test]
    fn png_test() {
        let l = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(100.0),
            y: Sample::Constant(100.0),
            polar_angle: Sample::Range(360.0, 0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Constant(0.0),
        };

        let s = Scene {
            resolution_x: 1920,
            resolution_y: 1080,
            viewport: Rect::from_points(&Point{x: 0.0,y: 0.0},&Point{x: 200.0,y: 200.0}),
            seed: 0,
            rays: 10_000,
            timelimit: 0,

            exposure: 1.0,
            gamma: 1.0,

            lights: vec!(l),
            objects: vec!(),
            materials: Vec::new(),
        };

        let r = Renderer::new(s);
        
        let image = r.render();

        let scale = image.calculate_scale(1.0, 10_000, 0.2);

        let data = image.to_rgb8(scale, 1.0/2.2);
        println!("data size = {}", data.len());
        
         let path = Path::new(r"lib.png_test.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, 1920, 1080); // Width is 2 pixels and height is 1.
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap(); // Save
    }
}

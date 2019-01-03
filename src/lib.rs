#![warn(missing_docs)]
extern crate aabb_quadtree;

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate png;

mod sampler;
mod scene;
mod object;
mod niave_rt;

pub mod prelude {
    pub use sampler::Sample;
    pub use scene::{Light, Material};
    pub use object::Object;
    pub use niave_rt::Renderer;
    pub use aabb_quadtree::geom::{Point, Rect};
}

mod spectrum;
mod prng;
mod ray;
mod image;


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
    use object::Object;
    use scene::{Material, Light};
    use sampler::Sample;
    use niave_rt::Renderer;

    use aabb_quadtree::geom::{Point, Rect};

    #[test]
    fn png_test() {
        let width = 1024.0;
        let height = 1024.0;
        
        let l = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(512.0),
            y: Sample::Constant(512.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(6900.0),
        };

        let viewport = Rect::from_points(&Point{x: 0.0,y: 0.0},&Point{x: width,y: height});
        let r = Renderer::new(width as usize, height as usize, viewport).with_light(l);
        let image = r.render(10_000);

        let mut count: f64 = 0.0;
        for p in image.pixels.iter() {
            count += p.0;
        };
        assert_ne!(count, 0.0);

        let scale = image.calculate_scale(1.0, 0.5);

        let data = image.to_rgb8(scale, 0.5);
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

        let m = Material {
            d: 0.3, r: 0.3, t: 0.3,
        };

        let o = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(height*0.75),
            dx: Sample::Constant(width),
            dy: Sample::Constant(0.0),
            material: m.clone(),
        };

        let l = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(width/2.0),
            y: Sample::Constant(height/2.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(6900.0),
        };

        let viewport = Rect::from_points(&Point{x: 0.0,y: 0.0},&Point{x: width,y: height});

        let r = Renderer::new(width as usize, height as usize, viewport).with_light(l).with_object(o);
        let image = r.render(rays);

        let mut count: u128 = 0;
        for p in image.pixels.iter() {
            count += p.0 as u128;
        };
        assert_ne!(count, 0);

        let scale = image.calculate_scale(1.0, 0.5);

        let data = image.to_rgb8(scale, 0.5);
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

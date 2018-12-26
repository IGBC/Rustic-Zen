extern crate png;
extern crate aabb_quadtree;
extern crate rustic_zen;

// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;

//Scene Parameters
use rustic_zen::object::Object;
use rustic_zen::scene::{Material, Light, Scene};
use rustic_zen::sampler::Sample;
use rustic_zen::niave_rt::Renderer;

use aabb_quadtree::geom::{Point, Rect};

fn laser(angle: f64, wavelength: f64) -> Light {
    Light {
        power: Sample::Constant(1.0),
        x: Sample::Constant(180.0),
        y: Sample::Constant(80.0), 
        polar_angle: Sample::Range(360.0, 0.0),
        polar_distance: Sample::Range(2.0, 0.0),
        ray_angle: Sample::Constant(angle),
        wavelength: Sample::Constant(wavelength),
    }
}

fn main() {
    let width: f64 = 1920.0;
    let height: f64 = 1080.0;
    let rays = 10_000; //(width * height).round() as usize;

    let wall_m = Material  { d: 1.0, r: 0.0, t: 0.0, };
    let floor_m = Material { d: 0.1, r: 0.3, t: 0.5, };

    /*let top = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(width),
        dy: Sample::Constant(0.0),
        material: wall_m.clone(),
    };*/

    let bottom = Object::Line {
        x0: Sample::Constant(height),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(width),
        dy: Sample::Constant(0.0),
        material: wall_m.clone(),
    };

    let left = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(0.0),
        dy: Sample::Constant(height),
        material: wall_m.clone(),
    };

    let right = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(width),
        dx: Sample::Constant(0.0),
        dy: Sample::Constant(height),
        material: wall_m.clone(),
    };

    let floor = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(height*0.72),
        dx: Sample::Constant(width),
        dy: Sample::Constant(0.0),
        material: floor_m.clone(),
    };

    let s = Scene {
        resolution_x: width as usize,
        resolution_y: height as usize,
        viewport: Rect::from_points(&Point{x: 0.0,y: 0.0},&Point{x: width+1.0,y: height+1.0}),
        seed: 0,
        rays: rays,
        timelimit: 0,

        exposure: 1.0,
        gamma: 1.0,

        lights: vec!(
            laser(30.0, 694.0),
            laser(31.0, 676.0),
            laser(32.0, 647.0),
            laser(33.0, 635.0),
            laser(34.0, 633.0),
            laser(35.0, 628.0),
            laser(36.0, 612.0),
            laser(37.0, 594.0),
            laser(38.0, 578.0),
            laser(39.0, 568.0),
            laser(40.0, 543.0),
            laser(41.0, 532.0),
            laser(42.0, 530.0),
            laser(43.0, 514.0),
            laser(44.0, 511.0),
            laser(45.0, 501.0),
            laser(46.0, 496.0),
            laser(47.0, 488.0),
            laser(48.0, 475.0),
            laser(49.0, 458.0),
            laser(50.0, 442.0),
            laser(51.0, 428.0),
            laser(52.0, 416.0),
        ),
        objects: vec!(/*top,*/ bottom, left, right, floor),
        materials: Vec::new(),
    };

    println!("Tracing Rays!");
    let r = Renderer::new(s);
    let image = r.render();

    println!("Serializing!");
    let scale = image.calculate_scale(23.0, rays, 0.12);
    let data = image.to_rgb8(scale, 0.0);

    println!("Saving!");
    let path = Path::new(r"laser-rainbow.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

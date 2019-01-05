extern crate png;
extern crate rustic_zen;

// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use png::HasParameters;

use rustic_zen::prelude::*;

fn main() {
    let width: f64 = 3440.0;
    let height: f64 = 1440.0;
    let rays = ((width * height).round() / 2.0) as usize;

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
        wavelength: Sample::Blackbody(4500.0),
    };

    let viewport = Rect::from_points(
        &Point { x: 0.0, y: 0.0 },
        &Point {
            x: width,
            y: height,
        },
    );

    println!("Tracing Rays!");
    let r = Renderer::new(width as usize, height as usize, viewport)
        .with_object(o)
        .with_light(l);
    let image = r.render(rays);

    println!("Serializing!");
    let scale = image.calculate_scale(0.7);
    let data = image.to_rgb8(scale, 1.2);

    println!("Saving!");
    let path = Path::new(r"dawn.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

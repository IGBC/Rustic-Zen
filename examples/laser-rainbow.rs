extern crate png;
extern crate rustic_zen;

// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
// To use encoder.set()
use png::HasParameters;

//Scene Parameters
use rustic_zen::prelude::*;

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
    let rays = 1_000_000; //(width * height).round() as usize;

    let wall_m = HQZLegacy::new(1.0, 0.0, 0.0);
    let floor_m = HQZLegacy::new(0.1, 0.3, 0.5);

    let top = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(width),
        dy: Sample::Constant(0.0),
        material: Box::new(wall_m.clone()),
    };

    let bottom = Object::Line {
        x0: Sample::Constant(height),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(width),
        dy: Sample::Constant(0.0),
        material: Box::new(wall_m.clone()),
    };

    let left = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(0.0),
        dx: Sample::Constant(0.0),
        dy: Sample::Constant(height),
        material: Box::new(wall_m.clone()),
    };

    let right = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(width),
        dx: Sample::Constant(0.0),
        dy: Sample::Constant(height),
        material: Box::new(wall_m.clone()),
    };

    let floor = Object::Line {
        x0: Sample::Constant(0.0),
        y0: Sample::Constant(height * 0.72),
        dx: Sample::Constant(width),
        dy: Sample::Range(70.0, -20.0),
        material: Box::new(floor_m.clone()),
    };

    let viewport = Rect::from_points(
        &Point { x: 0.0, y: 0.0 },
        &Point {
            x: width + 1.0,
            y: height + 1.0,
        },
    );

    println!("Tracing Rays!");
    let r = Scene::new(width as usize, height as usize, viewport)
        .with_light(laser(30.0, 694.0))
        .with_light(laser(31.0, 676.0))
        .with_light(laser(32.0, 647.0))
        .with_light(laser(33.0, 635.0))
        .with_light(laser(34.0, 633.0))
        .with_light(laser(35.0, 628.0))
        .with_light(laser(36.0, 612.0))
        .with_light(laser(37.0, 594.0))
        .with_light(laser(38.0, 578.0))
        .with_light(laser(39.0, 568.0))
        .with_light(laser(40.0, 543.0))
        .with_light(laser(41.0, 532.0))
        .with_light(laser(42.0, 530.0))
        .with_light(laser(43.0, 514.0))
        .with_light(laser(44.0, 511.0))
        .with_light(laser(45.0, 501.0))
        .with_light(laser(46.0, 496.0))
        .with_light(laser(47.0, 488.0))
        .with_light(laser(48.0, 475.0))
        .with_light(laser(49.0, 458.0))
        .with_light(laser(50.0, 442.0))
        .with_light(laser(51.0, 428.0))
        .with_light(laser(52.0, 416.0))
        .with_object(floor)
        .with_object(top)
        .with_object(bottom)
        .with_object(left)
        .with_object(right);
    let image = r.render(rays);

    println!("Serializing!");
    let data = image.to_rgb8(0.2, 1.0 / 2.2); //1.0/2.2

    println!("Saving!");
    let path = Path::new(r"laser-rainbow.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap(); // Save
}

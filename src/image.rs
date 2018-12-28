use std::mem::swap;
use prng::PRNG;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use spectrum::wavelength_to_colour;

pub struct Image {
    width: usize,
    height: usize,
    pub pixels: Vec<(f64, f64, f64)>,
    file: File,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut file = File::create("rays.csv").unwrap();
        file.write_all(b"r, g, b, x0, y0, x1, y1").unwrap();
        let len = width * height;
        let pixels: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); len];
        Image {
            width,
            height,
            pixels,
            file,
        }
    }

    #[inline]
    fn plot(&mut self, colour: (f64, f64, f64), pixel: usize, intensity: f64) {
        // Bounds checking;
        /*if (x < 0) || (y < 0) { return; };
        let x = x as usize;
        let y = y as usize;
        if x > self.width { return; };
        if y > self.height { return; };

        let i = (x + (y * self.width)).saturating_sub(1);*/
        if pixel >= self.pixels.len() { return; };
        let mut p = self.pixels[pixel];
        
        p.0 = p.0 + (colour.0 * intensity);
        p.1 = p.1 + (colour.1 * intensity);
        p.2 = p.2 + (colour.2 * intensity);

        self.pixels[pixel] = p;
    }

    pub fn draw_line(&mut self, wavelength: f64, mut x0: f64, mut y0: f64, mut x1: f64, mut y1: f64) {
        /*
         * Modified version of Xiaolin Wu's antialiased line algorithm:
         * http://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
         *
         * Brightness compensation:
         *   The total brightness of the line should be proportional to its
         *   length, but with Wu's algorithm it's proportional to dx.
         *   We scale the brightness of each pixel to compensate.
         */
        

        let colour: (f64, f64, f64) = wavelength_to_colour(wavelength);
        //println!("draw_line [{},{},{}] ({},{}), ({},{})", colour.0, colour.1, colour.2, x0, y0, x1, y1);
        let s: String = format!("{},{},{},{},{},{},{}\n", colour.0, colour.1, colour.2, x0, y0, x1, y1);
        self.file.write_all(s.as_bytes()).unwrap();

        let mut hx: i64 = 1;
        let mut hy: i64 = self.width as i64;

        let dx: f64 = (x1 - x0).abs();
        let dy: f64 = (y1 - y0).abs();

        // Axis swap. The virtual 'x' is always the major axis.
        if dy > dx {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            swap(&mut hx, &mut hy);
        }

        // We expect x0->x1 to be in the +X direction
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        // calculate gradient
        let dx: f64 = x1 - x0;
        let dy: f64 = y1 - y0;
        let gradient: f64 = if dx == 0.0 {
            1.0
        } else {
            dy / dx
        };

        // Brightness Calculation
        let br: f64 = 128.0 * f64::sqrt(dx*dx + dy*dy) / dx;

        // TODO clipping here

        // Handle first endpoint
        let xend: f64 = x0.round();
        let yend: f64 = y0 + gradient * (xend - x0);
        let xpxl1: i64 = xend as i64;
        let ypxl1: i64 = yend.floor() as i64;
 
        let xgap: f64 = br * (1.0 - (x0 + 0.5) + xend); // 0 to br
        let ygap: f64 = yend - yend.floor(); // 0 to 1

        self.plot(colour, (xpxl1 * hx + ypxl1 * hy) as usize, xgap * (1.0 - ygap));
        self.plot(colour, (xpxl1 * hx + (ypxl1 + 1) * hy) as usize, xgap * ygap);

        let mut intery: f64 = yend + gradient;

        //Handle Second endpoint
        let xend: f64 = x1.round();
        let yend: f64 = y1 + gradient * (xend - x1);
        let xpxl2: i64 = xend as i64;
        let ypxl2: i64 = yend.floor() as i64;

        let xgap: f64 = br * (1.0 - (x1 + 0.5) + xend); // 0 to br
        let ygap: f64 = yend - yend.floor(); // 0 to 1

        self.plot(colour, (xpxl2 * hx + ypxl2 * hy) as usize, xgap * (1.0 - ygap));
        self.plot(colour, (xpxl2 * hx + (ypxl2 + 1) * hy) as usize, xgap * ygap);

        // Loop Over line
        for x in xpxl1 + 1 .. xpxl2 - 1 {
            let iy: i64 = intery.floor() as i64;
            let fy: f64 = intery - intery.floor(); // 0 to 1

            self.plot(colour, (x * hx + iy * hy) as usize, br * (1.0 - fy));
            self.plot(colour, (x * hx + (iy + 1) * hy) as usize, br * fy);

            intery += gradient;
        }
    }

    fn max(a: f64, b: f64) -> f64 {
        if a < b {
            b
        } else {
            a
        }
    }

    fn min(a: f64, b: f64) -> f64 {
        if a > b {
            b
        } else {
            a
        }
    }

    pub fn calculate_scale(&self, lightpower: f64, num_rays: usize, exposure: f64) -> f64 {
        let area_scale = f64::sqrt((self.width as f64 * self.height as f64) / (1024.0 * 576.0));
        let intensity_scale = lightpower / (255.0 * 8192.0);
        let scale = f64::exp(1.0 + 10.0 * exposure) * area_scale * intensity_scale / num_rays as f64;
        scale
    }

    pub fn to_rgb8(&self, scale: f64, exponent: f64) -> Vec<u8> {
        let mut rng = PRNG::seed(0);
        let mut rgb: Vec<u8> = Vec::new();
        for i in self.pixels.iter() {
            // green
            let u:f64 = Self::max(0.0,i.1.clone() as f64 * scale);
            let dither = rng.uniform_f64();
            let v:f64 = 255.0 * Self::max(u, exponent) + dither;
            let g8 = Self::max(0.0, Self::min(255.9,v));
            rgb.push(g8 as u8);

            // red
            let u:f64 = Self::max(0.0,i.0.clone() as f64 * scale);
            let dither = rng.uniform_f64();
            let v:f64 = 255.0 * Self::max(u, exponent) + dither;
            let r8 = Self::max(0.0, Self::min(255.9,v));
            rgb.push(r8 as u8);

            // blue
            let u:f64 = Self::max(0.0,i.2.clone() as f64 * scale);
            let dither = rng.uniform_f64();
            let v:f64 = 255.0 * Self::max(u, exponent) + dither;
            let b8 = Self::max(0.0, Self::min(255.9,v));
            rgb.push(b8 as u8);
        }
        return rgb;
    }

    pub fn dumb_to_rgb8(&self) -> Vec<u8> {
        let mut rgb: Vec<u8> = Vec::new();
        for i in self.pixels.iter() {
            let g8 = Self::min(255.0,i.1.clone());
            rgb.push(g8 as u8);
            let r8 = Self::min(255.0,i.0.clone());
            rgb.push(r8 as u8);
            let b8 = Self::min(255.0,i.2.clone());
            rgb.push(b8 as u8);
        }
        return rgb;
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    
    // For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // To use encoder.set()
    use png::HasParameters;

    #[test]
    fn traced_ray_is_not_black() {
        let mut i = Image::new(100, 100);
        i.draw_line(620.0, 10.0, 10.0, 90.0, 90.0); //red 
        i.draw_line(520.0, 20.0, 10.0, 90.0, 80.0); //green
        i.draw_line(470.0, 10.0, 20.0, 80.0, 90.0); //blue
        let mut count: f64 = 0.0;
        for p in i.pixels.iter() {
            count += p.0;
        }
        assert_ne!(count, 0.0);

        let path = Path::new(r"image.ray_not_black.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let scale = i.calculate_scale(1.0, 3, 0.12);
        let data = i.to_rgb8(scale ,0.0);

        let mut encoder = png::Encoder::new(w, 100, 100);
        encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&data).unwrap(); // Save
    }
    
    #[test]
    fn empty_image_is_black() {
        let i = Image::new(1920, 1080);
        let v = i.dumb_to_rgb8();
        for i in v.iter() {
            assert_eq!(i.clone(), 0u8);
        }
    }

    #[test]
    fn output_len() {
        let i = Image::new(1920, 1080);
        let v = i.to_rgb8(1.0, 1.0);
        assert_eq!(v.len(), 1920*1080*3);
    }
}
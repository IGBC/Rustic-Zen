use std::mem::swap;
use prng::PRNG;
use std::cmp;

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<(u64, u64, u64)>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        let pixels: Vec<(u64, u64, u64)> = vec![(0, 0, 0); len];
        Image {
            width,
            height,
            pixels,
        }
    }

    #[inline]
    fn plot(&mut self, colour: (u16, u16, u16), x: usize, y: usize, intensity: f64) {
        let i = x + (y * self.width);
        let mut p = self.pixels[i];
        
        p.0 = p.0.saturating_add((colour.0 as f64 * intensity).round() as u64);
        p.1 = p.1.saturating_add((colour.1 as f64 * intensity).round() as u64);
        p.2 = p.2.saturating_add((colour.2 as f64 * intensity).round() as u64);

        self.pixels[i] = p;
    }

    pub fn draw_line(&mut self, colour: (u16, u16, u16), mut x0: f64, mut y0: f64, mut x1: f64, mut y1: f64) {
        let dx: f64 = (x1 - x0).abs();
        let dy: f64 = (y1 - y0).abs();

        // Axis swap. The virtual 'x' is always the major axis.
        if dy > dx {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
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
        let xpxl1: usize = xend as usize;
        let ypxl1: usize = yend.floor() as usize;

        let xgap: f64 = br * (1.0 - (x0 + 0.5) + xend); // 0 to br
        let ygap: f64 = yend - yend.floor(); // 0 to 1

        self.plot(colour, xpxl1, ypxl1, xgap * (1.0 - ygap));
        self.plot(colour, xpxl1, ypxl1 + 1, xgap * ygap);

        let mut intery: f64 = yend + gradient;

        //Handle Second endpoint
        let xend: f64 = x1.round();
        let yend: f64 = y1 + gradient * (xend - x1);
        let xpxl2: usize = xend as usize;
        let ypxl2: usize = yend.floor() as usize;

        let xgap: f64 = br * (1.0 - (x1 + 0.5) + xend); // 0 to br
        let ygap: f64 = yend - yend.floor(); // 0 to 1

        self.plot(colour, xpxl2, ypxl2, xgap * (1.0 - ygap));
        self.plot(colour, xpxl2, ypxl2 + 1, xgap * ygap);

        // Loop Over line
        for x in xpxl1 + 1 .. xpxl2 - 1 {
            let iy: usize = intery.floor() as usize;
            let fy: f64 = intery - intery.floor(); // 0 to 1

            self.plot(colour, x, iy, br * (1.0 - fy));
            self.plot(colour, x, iy + 1, br * fy);

            intery += gradient;
        }
    }

    // fn to_rgb8(&self, scale: f64, exponent: f64) -> Vec<u8> {
    //     let rng = PRNG::seed(0);
    //     let rgb: Vec<u8> = Vec::new();
    //     for i in self.pixels.iter() {
    //         let (r,g,b) = i;
    //         // red
    //         let u:f64 = cmp::partial_max(0.0,r.clone() as f64 * scale);
    //         let dither = rng.uniform_f64();
    //         let v:f64 = 255.0 * cmp::partial_max(u, exponent) + dither;
    //         let r8 = cmp::partial_max(0.0, cmp::partial_min(255.9,v));
    //     }
    //     return vec![];
    // }
}
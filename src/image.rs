use std::mem::swap;

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<(i64, i64, i64)>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        let pixels: Vec<(i64, i64, i64)> = vec![(0, 0, 0); len];
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
        
        p.0 = p.0 + ((colour.0 as f64 * intensity).round() as i64);
        p.1 = p.1 + ((colour.1 as f64 * intensity).round() as i64);
        p.2 = p.2 + ((colour.2 as f64 * intensity).round() as i64);

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
}
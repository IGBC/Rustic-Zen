use scene::Light;
use prng::PRNG;
use spectrum::wavelength_to_colour;
use std::f64::consts::PI;
use object::Object;

pub struct Vec2 {
    x: f64, 
    y: f64,
}


struct IntersectionData<'a> {
    ray: Ray,
    point: Vec2,
    normal: Vec2,
    distance: f64,
    hit: Option<&'a Object<'a>>,
}

impl<'a> IntersectionData<'a> {
   pub fn new() {

   }

   pub fn ray_intersect()->bool {
       return false;
   }
}


pub struct Ray {
    origin: Vec2,
    direction: Vec2,
    colour: (u16, u16, u16),
    slope: f64,
}

impl Ray {
    pub fn new(light: &Light, rng: &mut PRNG)->Self {
        let cart_x = light.x.val(rng);
        let cart_y = light.y.val(rng);
        let polar_angle = light.polar_angle.val(rng);
        let polar_dist  = light.polar_distance.val(rng);
        let origin = Vec2 {
            x: cart_x + f64::cos(polar_angle) * polar_dist,
            y: cart_y + f64::sin(polar_angle) * polar_dist,
        };
        let ray_angle = light.ray_angle.val(rng) * (PI / 180.0);
        // Set Angle
        let direction = Vec2 {
            x: f64::cos(ray_angle),
            y: f64::sin(ray_angle),
        };
        let slope = direction.y / direction.x;
        // Set Colour
        let mut visible = false;
        let mut tries = 1000;
        let mut colour = (0, 0, 0);
        while !visible {
            let wavelen = light.wavelength.val(rng);
            colour = wavelength_to_colour(wavelen);
            if colour.0 == 0 &&  colour.1 == 0 && colour.2 == 0 {
                tries -= 1;
                if tries == 0 {
                    // Unlikely
                    panic!("Colour Sampling ran out of tries");
                } else {
                    visible = true;
                }
            }

        }
        // wrap in an object
        Ray {
            origin,
            direction,
            slope,
            colour,
        }
    }


    /**
     * Does *not* require 'normal' to already be normalized
     */
    fn reflect(&self, normal: &Vec2) -> Vec2 {
        let t: f64 = 2.0 * (normal.x * self.direction.x + normal.y * self.direction.y) /
            (normal.x * normal.x + normal.y * normal.y);
        let x = self.direction.x - t * normal.x;
        let y = self.direction.y - t * normal.y;
        Vec2 {x, y}
    }



    pub fn bounce(&self, normal: &Vec2) -> Self {
        let origin = self.origin;

        let direction = self.reflect(normal);
        let slope = direction.y / direction.x;
        Ray {
            origin,
            direction,
            slope,
            colour: self.colour,
        }
    }
}
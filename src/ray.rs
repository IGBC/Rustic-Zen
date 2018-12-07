use scene::Light;
use prng::PRNG;
use spectrum::wavelength_to_colour;
use std::f64::consts::PI;
use object::Object;

#[derive(Clone)]
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
}

impl Ray {

    /**
     * Creates new ray from light source, sampling the light apropriately.
     */
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
            colour,
        }
    }


    /**
     * Computes fate of this ray, 
     * returns option wrapping direciton of new ray, 
     * none if no new ray.
     */
    fn outcome(&self, obj: &Object, normal: &Vec2, rng: &mut PRNG) -> Option<Vec2> {
        let mat = obj.get_material();

        Some(Vec2{
            x: 0.0,
            y: 0.0,
        })
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


    /**
     * Returns the resulting ray from colliding with object,
     * returns none if it does not actually hit the object.
     * Objects are sampled so two identical rays may not have the same outcome.
     */
    pub fn bounce(&self, obj: &Object, rng: &mut PRNG) -> Option<Self> {
        /*
        // Todo get actual ray start. And do an actual collision test
        let origin = self.origin.clone();

        //let (hit, normal) = obj.get_hit(self.origin, self.direction, rng);

        //let direction = self.outcome(obj, normal, rng);
        if direction.is_none() { return None; }
        let direction = direction.unwrap();


        let slope = direction.y / direction.x;
        Option::Some(Ray {
            origin: hit,
            direction,
            colour: self.colour,
        })
        */

        None
    }
}
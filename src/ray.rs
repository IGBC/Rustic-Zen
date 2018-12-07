use scene::Light;
use prng::PRNG;
use spectrum::wavelength_to_colour;
use std::f64::consts::PI;
use object::Object;
use aabb_quadtree::geom::{Point, Vector};


pub struct Ray {
    origin: Point,
    direction: Vector,
    colour: (u16, u16, u16),
    bounces: u32,
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
        let origin = Point {
            x: cart_x + f64::cos(polar_angle) * polar_dist,
            y: cart_y + f64::sin(polar_angle) * polar_dist,
        };
        let ray_angle = light.ray_angle.val(rng) * (PI / 180.0);
        // Set Angle
        let direction = Vector {
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
            bounces: 1000,
        }
    }


    /**
     * Computes fate of this ray, 
     * returns option wrapping direciton of new ray, 
     * none if no new ray.
     */
    fn outcome(&self, obj: &Object, normal: &Vector, rng: &mut PRNG) -> Option<Vector> {
        let mat = obj.get_material();
        let f = rng.uniform_f64();
        
        if f <= mat.d {
            let angle = rng.uniform_range(2.0 * PI, 0.0);
            return Some(Vector{x: f64::cos(angle), y: f64::sin(angle)});
        }

        if f <= mat.d + mat.r {
            let angle = self.reflect(normal);
            return Some(angle);
        }

        if f <= mat.d + mat.r + mat.t {
            let angle = self.direction.clone();
            return Some(angle);
        }

        None
    }

    /**
     * Does *not* require 'normal' to already be normalized
     */
    fn reflect(&self, normal: &Vector) -> Vector {
        let t: f64 = 2.0 * (normal.x * self.direction.x + normal.y * self.direction.y) /
            (normal.x * normal.x + normal.y * normal.y);
        let x = self.direction.x - t * normal.x;
        let y = self.direction.y - t * normal.y;
        Vector {x, y}
    }


    /**
     * Returns the resulting ray from colliding with object,
     * returns none if it does not actually hit the object.
     * Objects are sampled so two identical rays may not have the same outcome.
     */
    pub fn bounce(&self, obj: &Object, rng: &mut PRNG) -> Option<Self> {
        // Todo get actual ray start. And do an actual collision test
        let (hit, normal) = obj.get_hit(&self.origin, &self.direction, rng).unwrap();

        let outcome = self.outcome(obj, &normal, rng);
        let direction = match outcome {
            Some(o) => o,
            None => {return None;}, 
        };

        Option::Some(Ray {
            origin: hit,
            direction,
            colour: self.colour,
            bounces: self.bounces - 1,
        })
    }
}

#[cfg(test)]
mod test {
    use scene::Light;
    use sampler::Sample;
    use prng::PRNG;
    use super::Ray;

    #[test]
    fn new_works() {
        let mut rng = PRNG::seed(0); 

        let l = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(100.0),
            y: Sample::Constant(100.0),
            polar_angle: Sample::Range(360.0, 0.0),
            polar_distance: Sample::Constant(1.0),
            ray_angle: Sample::Range(360.0, 0.0),
            wavelength: Sample::Blackbody(0.0),
        };

        Ray::new(&l, &mut rng);
    }
}
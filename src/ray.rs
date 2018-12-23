use scene::Light;
use prng::PRNG;
use spectrum::wavelength_to_colour;
use std::f64::consts::PI;
use object::Object;
use aabb_quadtree::geom::{Point, Vector, Rect};
use image::Image;
use std::num;


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

    pub fn get_origin(&self) -> &Point {
        return &self.origin;
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

    pub fn collision_list(&self, obj_list: &Vec<Object>, viewport: Rect, image: &mut Image, rng: &mut PRNG) -> Option<Self> {
        // get closest Collision
        // Mercifully O(N)
        let mut c_distance = std::f64::MAX;
        let mut c_hit: Option<Point> = None;
        let mut c_res: Option<Self> = None;
        for i in obj_list.iter() {
            let result = self.bounce(i, rng);
            match result {
                None => {},
                Some(i) => {
                    let dist = self.origin.distance(i.get_origin());
                    if dist < c_distance {
                        c_distance = dist;
                        c_hit = Some(i.get_origin().clone());
                        c_res = Some(i);
                    }
                }
            }
        }

        let end = match c_hit {
            None =>  // We hit nothing, we need to test on the viewport!
                self.furthest_aabb(viewport).expect("Ray exists outside of Viewport"),
            Some(p) => p, //this is the closest point we hit!
        };

        image.draw_line(self.colour, self.origin.x, self.origin.y, end.x, end.y);

        // if we have bounces left Return the result else None.
        if self.bounces > 0 {
            c_res
        } else {
            None
        }
    }


    /**
     * Returns the resulting ray from colliding with object,
     * returns none if it does not actually hit the object.
     * Objects are sampled so two identical rays may not have the same outcome.
     */
    pub fn bounce(&self, obj: &Object, rng: &mut PRNG) -> Option<Self> {
        // Todo get actual ray start. And do an actual collision test
        let (hit, normal) = match obj.get_hit(&self.origin, &self.direction, rng){
            None => {return None},
            Some((hit, normal)) => (hit, normal),
        };

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

    fn intersect_edge(&self, s1: Point, sd: Point) -> Option<f64> {
        /*
        let sdy = (sd.y - s1.y);
        let sdx = (sd.x - s1.x);

        let sm = sdy / sdx;
        let sc = s1.y - sm * s1.x;
        
        let ody = (self.origin.y - self.direction.y);
        let odx = (self.origin.x - self.direction.x);

        let om = ody / odx;
        let oc = self.origin.y - om * self.origin.x;

        let x = (om - sm) / (sc - oc);
        let y = om * x + oc;

        let dx = (x - self.origin.x);
        let dy = (y - self.origin.y);

        let dist = ((dx * dx) + (dy * dy)).sqrt();

        return Some(dist);
        */

        let slope = self.direction.y / self.direction.x;
        let alpha = ((s1.x - self.origin.x) * slope + (self.origin.y - s1.y)) / (sd.y - sd.x * slope);
        if alpha < 0.0 || alpha > 1.0 { return None; }
        
        let distance = (s1.x + sd.x * alpha - self.origin.x) / self.direction.x;
        if distance < 0.0 { return None; } 
        return Some(distance);
    }

    pub fn furthest_aabb(&self, aabb: Rect) -> Option<Point> {
        let mut max_dist: Option<f64> = None;
        
        let horizontal = Point{ x: aabb.top_right().x - aabb.top_left().x, y: 0.0 };
        let vertical = Point{ x: 0.0, y: aabb.bottom_left().y - aabb.top_left().y };

        // top
        match self.intersect_edge(aabb.top_left(), horizontal) {
            None => (),
            Some(d) => {
                max_dist = match max_dist {
                    None => Some(d),
                    Some(md) => {
                        if d > md {
                            Some(d)
                        } else {
                            max_dist
                        }
                    }
                };  
            }
        }

        // bottom
        match self.intersect_edge(aabb.bottom_left(), horizontal) {
            None => (),
            Some(d) => {
                max_dist = match max_dist {
                    None => Some(d),
                    Some(md) => {
                        if d > md {
                            Some(d)
                        } else {
                            max_dist
                        }
                    }
                };  
            }
        }

        // left
        match self.intersect_edge(aabb.top_left(), vertical) {
            None => (),
            Some(d) => {
                max_dist = match max_dist {
                    None => Some(d),
                    Some(md) => {
                        if d > md {
                            Some(d)
                        } else {
                            max_dist
                        }
                    }
                };  
            }
        }

        // right
        match self.intersect_edge(aabb.top_right(), vertical) {
            None => (),
            Some(d) => {
                max_dist = match max_dist {
                    None => Some(d),
                    Some(md) => {
                        if d > md {
                            Some(d)
                        } else {
                            max_dist
                        }
                    }
                };  
            }
        }
        

        match max_dist {
            None => {return None;},
            Some(d) => {
                return Some(Point {
                    x: self.origin.x + d * self.direction.x,
                    y: self.origin.y + d * self.direction.y,
                });
            }
        }
    }

    pub fn closest_aabb(&self, aabb: Rect) -> Option<Point> {
        let mut min_dist = std::f64::MAX;
        let mut min: Option<Point> = None;
        // top
        match self.intersect_edge(aabb.top_left(), aabb.top_right()) {
            None => (),
            Some(d) => {
                if d < min_dist {
                    min_dist = d;
                    min = Some(Point {
                        x: self.origin.x + d * self.direction.x,
                        y: self.origin.y + d * self.direction.y,
                    })
                }
            }
        }

        // bottom
        match self.intersect_edge(aabb.bottom_left(), aabb.bottom_right()) {
            None => (),
            Some(d) => {
                if d < min_dist {
                    min_dist = d;
                    min = Some(Point {
                        x: self.origin.x + d * self.direction.x,
                        y: self.origin.y + d * self.direction.y,
                    })
                }
            }
        }

        // left
        match self.intersect_edge(aabb.top_left(), aabb.bottom_left()) {
            None => (),
            Some(d) => {
                if d < min_dist {
                    min_dist = d;
                    min = Some(Point {
                        x: self.origin.x + d * self.direction.x,
                        y: self.origin.y + d * self.direction.y,
                    })
                }
            }
        }

        // right
        match self.intersect_edge(aabb.top_right(), aabb.bottom_right()) {
            None => (),
            Some(d) => {
                if d < min_dist {
                    min_dist = d;
                    min = Some(Point {
                        x: self.origin.x + d * self.direction.x,
                        y: self.origin.y + d * self.direction.y,
                    })
                }
            }
        }
        
        min
    }
}

#[cfg(test)]
mod test {
    use scene::Light;
    use sampler::Sample;
    use prng::PRNG;
    use super::Ray;
    use aabb_quadtree::geom::{Point,Rect};

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

    #[test]
    fn furthest_aabb_hits_horziontal() {
        let mut rng = PRNG::seed(0); 

        let x_plus_light = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(0.0),
            y: Sample::Constant(0.0),
            polar_angle: Sample::Range(0.0, 0.0),
            polar_distance: Sample::Constant(0.0),
            // x = cos(0) = 1; y = sin(0) = 0
            ray_angle: Sample::Range(0.0, 0.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        //Firing a ray in x+, 0 from origin
        let ray = Ray::new(&x_plus_light, &mut rng);

        // wall from 1,-10 to 11, +10 should be in the way
        let p1 = Point{ x: 1.0, y: -10.0, };
        let p2 = Point{ x: 11.0, y: 10.0, };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);
        
        // check hit!
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.x, 11.0);
        assert_eq!(result.y,  0.0);
    }

    #[test]
    fn furthest_aabb_hits_vertical() {
        let mut rng = PRNG::seed(0); 

        let x_plus_light = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(0.0),
            y: Sample::Constant(0.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            // x = cos(90) = 0; y = sin(90) = 1
            ray_angle: Sample::Constant(90.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        //Firing a ray in 0, +y from origin
        let ray = Ray::new(&x_plus_light, &mut rng);

        // wall from 1,-10 to 11, +10 should be in the way
        let p1 = Point{ x: -10.0, y: 1.0, };
        let p2 = Point{ x: 10.0, y: 11.0, };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);
        
        // check hit!
        assert!(result.is_some());
        let result = result.unwrap();
        println!("result: ({},{})", result.x, result.y);
        assert_eq!(result.x,  0.0);
        assert_eq!(result.y, 11.0);
    }

    #[test]
    fn furthest_aabb_hits_almost_vertical() {
        let mut rng = PRNG::seed(0); 

        let x_plus_light = Light{
            power: Sample::Constant(1.0),
            x: Sample::Constant(0.0),
            y: Sample::Constant(0.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            // x = cos(45) = rt(2); y = sin(45) = rt(2)
            ray_angle: Sample::Constant(45.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        //Firing a diagonal ray +x, +y from origin
        let ray = Ray::new(&x_plus_light, &mut rng);

        // wall from 1,-10 to 11, +10 should be in the way
        let p1 = Point{ x: -10.0, y: 1.0, };
        let p2 = Point{ x: 20.0, y: 11.0, };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);
        
        // check hit!
        assert!(result.is_some());
        let result = result.unwrap();
        println!("result: ({},{})", result.x, result.y);
        assert_eq!(result.x.round(), 11.0);
        assert_eq!(result.y.round(), 11.0);
    }
}
use geom::{Point, Rect, Vector, Matrix};
use object::Object;
use pcg_rand::{Pcg64Fast};
use scene::Light;
use std::f64::consts::PI;
use rand::prelude::*;
use pcg_rand::seeds::PcgSeeder;

pub type Hit = Option<(Point, Option<HitData>)>;

#[derive(Copy, Clone)]
pub struct HitData {
    pub normal: Vector,
    pub wavelength: f64,
    pub distance: f64,
    pub alpha: f64,
    pub bounces: u32,
    pub material: usize,
}


pub struct Ray {
    origin: Point,
    direction: Vector,
    wavelength: f64,
    bounces: u32,
    ray_rng: Pcg64Fast,
}

impl Ray {
    /**
     * Creates new ray from light source, sampling the light apropriately.
     */
    pub fn new(light: &Light, rng: &mut Pcg64Fast) -> Self {
        let cart_x = light.x.val(rng);
        let cart_y = light.y.val(rng);
        let polar_angle = light.polar_angle.val(rng) * (PI / 180.0);
        let polar_dist = light.polar_distance.val(rng);
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
        let wavelength = light.wavelength.val(rng);
        // wrap in an object
        let mut pcg = Pcg64Fast::from_seed(PcgSeeder::seed(rng.gen()));
        // PCG's act weird when you initialise them so we're gonna throw away the first value
        pcg.gen::<f64>();
        pcg.gen::<f64>();
        Ray {
            origin,
            direction,
            wavelength,
            bounces: 1000,
            ray_rng: pcg,
        }
    }

    pub fn get_origin(&self) -> &Point {
        return &self.origin;
    }

    pub fn get_wavelength(&self) -> f64 {
        return self.wavelength;
    }

    pub fn collision_list(&mut self, obj_list: &Vec<Object>,viewport: &Rect,) -> Hit {
        // get closest Collision
        // Mercifully O(N)
        let mut c_distance = std::f64::MAX;
        let mut c_hit: Option<Point> = None;
        let mut c_norm: Option<Vector> = None;
        let mut c_alpha: Option<f64> = None;
        let mud c_material: Option<usize> = None;
        for obj in obj_list.iter() {
            match obj.get_hit(&self.origin, &self.direction, &mut self.ray_rng) {
                None => {},
                Some((hit, normal, alpha)) => {
                    let dist = self.origin.distance(&hit);

                    if dist >= 3.0 {
                        if dist < c_distance {
                            c_distance = dist;
                            c_hit = Some(hit);
                            c_norm = Some(normal);
                            c_alpha = Some(alpha);
                            c_material = Some(obj.get_material());
                        }
                    }    
                },
            };
        }

        match c_hit {
            None =>  { // We hit nothing, we need to test on the viewport!
                match self.furthest_aabb(viewport) {
                    None => None,
                    Some(p) => Some((p, None)),
                }
            },
            Some(p) => {
                // if we have bounces left Return the result else None.
                if self.bounces > 1 {
                    Some((p, Some(HitData{
                            normal: c_norm.unwrap(), 
                            alpha: c_alpha.unwrap(), 
                            distance: c_distance,
                            bounces: self.bounces,
                            wavelength: self.wavelength,
                            material: c_material,
                        }
                    )))
                } else {
                    Some((p, None))
                }
            },
        }
    }

    /**
     * Returns the resulting ray from colliding with object,
     * returns none if it does not actually hit the object.
     * Objects are sampled so two identical rays may not have the same outcome.
     */
    fn bounce(&mut self, obj: &Object) -> Option<Self> {
        // Todo get actual ray start. And do an actual collision test
        let (hit, normal, alpha) = match obj.get_hit(&self.origin, &self.direction, &mut self.ray_rng) {
            None => return None,
            Some((hit, normal, alpha)) => (hit, normal, alpha),
        };

        let dist = self.origin.distance(&hit);

        if dist < 3.0 {
            return None;
        }

        let mat = obj.get_material();
        let outcome = None; //mat.outcome(&self.direction, &normal, self.wavelength, alpha, &mut self.ray_rng);
        let direction = match outcome {
            Some(o) => o,
            None => {
                return None;
            }
        };

        Option::Some(Ray {
            origin: hit,
            direction,
            wavelength: self.wavelength,
            bounces: self.bounces - 1,
            ray_rng: Pcg64Fast::from_seed(PcgSeeder::seed(self.ray_rng.gen())),
        })
    }

    fn intersect_edge(&self, s1: Point, sd: Vector) -> Option<f64> {
        let mat_a = Matrix {
            a1: sd.x, b1: -self.direction.x,
            a2: sd.y, b2: -self.direction.y,
        };

        let omega = self.origin - s1;
        
        let result = match mat_a.inverse() {
            Some(m) => m * omega,
            None => {
                return None; // Probably cos rays are parallel
            }
        };
        if (result.x >= 0.0) && (result.x <= 1.0) && (result.y > 0.0) {
            Some(result.y)
        } else {
            None
        }


        // let slope = self.direction.y / self.direction.x;
        // let alpha =
        //     ((s1.x - self.origin.x) * slope + (self.origin.y - s1.y)) / (sd.y - sd.x * slope);
        // if alpha < 0.0 || alpha > 1.0 {
        //     return None;
        // }

        // let distance = (s1.x + sd.x * alpha - self.origin.x) / self.direction.x;
        // if distance < 0.0 {
        //     return None;
        // }
        // return Some(distance);
    }

    pub fn furthest_aabb(&self, aabb: &Rect) -> Option<Point> {
        let mut max_dist: Option<f64> = None;

        let horizontal = Vector {
            x: aabb.top_right().x - aabb.top_left().x,
            y: 0.0,
        };
        let vertical = Vector {
            x: 0.0,
            y: aabb.bottom_left().y - aabb.top_left().y,
        };

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
            None => {
                return None;
            }
            Some(d) => {
                return Some(Point {
                    x: self.origin.x + d * self.direction.x,
                    y: self.origin.y + d * self.direction.y,
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Ray;
    use geom::{Point, Rect};
    use sampler::Sample;
    use scene::Light;
    use rand::prelude::*;
    use pcg_rand::Pcg64Fast;

    #[test]
    fn new_works() {
        let mut rng = Pcg64Fast::from_entropy();

        let l = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(100.0),
            y: Sample::Constant(100.0),
            polar_angle: Sample::Constant(360.0),
            polar_distance: Sample::Constant(1.0),
            ray_angle: Sample::Constant(0.0),
            wavelength: Sample::Constant(460.0),
        };

        let r = Ray::new(&l, &mut rng);
        assert_eq!(r.origin.x.round(), 101.0);
        assert_eq!(r.origin.y.round(), 100.0);
        assert_eq!(r.direction.x.round(), 1.0);
        assert_eq!(r.direction.y.round(), 0.0);
        assert_eq!(r.wavelength.round(), 460.0);
        assert_eq!(r.bounces, 1000);
    }

    #[test]
    fn furthest_aabb_hits_horziontal() {
        let mut rng = Pcg64Fast::from_entropy();

        let x_plus_light = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(0.0),
            y: Sample::Constant(0.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            // x = cos(0) = 1; y = sin(0) = 0
            ray_angle: Sample::Constant(0.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        //Firing a ray in x+, 0 from origin
        let ray = Ray::new(&x_plus_light, &mut rng);

        // wall from 1,-10 to 11, +10 should be in the way
        let p1 = Point { x: 1.0, y: -10.0 };
        let p2 = Point { x: 11.0, y: 10.0 };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);

        // check hit!
        let result = result.expect("Result should have been Some()");
        assert_eq!(result.x, 11.0);
        assert_eq!(result.y, 0.0);
    }

    #[test]
    fn furthest_aabb_hits_vertical() {
        let mut rng = Pcg64Fast::from_entropy();

        let x_plus_light = Light {
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
        let p1 = Point { x: -10.0, y: 1.0 };
        let p2 = Point { x: 10.0, y: 11.0 };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);

        // check hit!
        let result = result.expect("That shouldn't be None!");
        println!("result: ({},{})", result.x, result.y);
        assert_eq!(result.x.round(), 0.0);
        assert_eq!(result.y.round(), 11.0);
    }

    #[test]
    fn furthest_aabb_hits_almost_vertical() {
        let mut rng = Pcg64Fast::from_entropy();

        let x_plus_light = Light {
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
        let p1 = Point { x: -10.0, y: 1.0 };
        let p2 = Point { x: 20.0, y: 11.0 };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);

        // check hit!
        assert!(result.is_some());
        let result = result.expect("Something was meant to be there!");
        println!("result: ({},{})", result.x, result.y);
        assert_eq!(result.x.round(), 11.0);
        assert_eq!(result.y.round(), 11.0);
    }

    #[test]
    fn furthest_aabb_special_case() {
        let mut rng = Pcg64Fast::from_entropy();

        let x_plus_light = Light {
            power: Sample::Constant(1.0),
            x: Sample::Constant(100.0),
            y: Sample::Constant(700.0),
            polar_angle: Sample::Constant(0.0),
            polar_distance: Sample::Constant(0.0),
            // x = cos(45) = rt(2); y = sin(45) = rt(2)
            ray_angle: Sample::Constant(-45.0),
            wavelength: Sample::Blackbody(5800.0),
        };

        //Firing a diagonal ray +x, -y from origin
        let ray = Ray::new(&x_plus_light, &mut rng);

        // wall from 1,-10 to 11, +10 should be in the way
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point {
            x: 200.0,
            y: 1000.0,
        };
        let aabb = Rect::from_points(&p1, &p2);

        let result = ray.furthest_aabb(aabb);

        // check hit!
        let result = result.expect("None is not what we wanted");
        println!("result: {:?}", result);
        assert_eq!(result.x.round(), 200.0);
        assert_eq!(result.y.round(), 600.0);
    }
}

use aabb_quadtree::geom::{Point, Rect};
use scene::Material;
use sampler::Sample;
use prng::PRNG;
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Object<'a> {
    Line{
        material: &'a Material,
        x0: Sample,
        y0: Sample,
        dx: Sample,
        dy: Sample,
    },
    Curve{
        material: &'a Material,
        x0: Sample,
        y0: Sample,
        a0: Sample,
        dx: Sample,
        dy: Sample,
        da: Sample,
    },
}

impl<'a> Object<'a> {
    /// Returns a rectangle enclosing The Object.
    /// Currently does not support curved lines, cos math is hard.
    pub fn bounds(&self) -> Rect {
        match self {
            Object::Curve{..} => {
                // TODO: implement this, 
                // its the same algorithm as for the line,
                // the curve will always fit in the box.
                unimplemented!();
            },
            Object::Line{x0, y0, dx, dy, ..} => {
                let mut x_s = [
                    x0.bounds().0, x0.bounds().1,
                    x0.bounds().0 + dx.bounds().0,
                    x0.bounds().0 + dx.bounds().1,
                    x0.bounds().1 + dx.bounds().0,
                    x0.bounds().1 + dx.bounds().1,
                ];
                x_s.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut y_s = [
                    y0.bounds().0, y0.bounds().1,
                    y0.bounds().0 + dy.bounds().0,
                    y0.bounds().0 + dy.bounds().1,
                    y0.bounds().1 + dy.bounds().0,
                    y0.bounds().1 + dy.bounds().1,
                ];
                y_s.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p0 = Point {
                    x: x_s[0] as f32,
                    y: y_s[0] as f32,
                };
                
                let p1 = Point {
                    x: x_s[x_s.len()] as f32,
                    y: y_s[y_s.len()] as f32,
                };

                return Rect::from_points(&p0, &p1);
            }
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Object::Curve{material, ..} => material,
            Object::Line {material, ..} => material,
        }
    }

    /**
     * Tests if the inbound ray actually hit the object,
     * if so it returns the coords of the hit, followed by the normal
     * to the hit surface.
     *
     * If miss it returns None
     * 
     * This test assumes you have done a box test to 
     * check the bounds of the line first
     */
    pub fn get_hit(&self, origin: &Point, dir: &Point, rng: &mut PRNG) -> Option<(Point, Point)> {
        // Get s1 and sD from samples
        let (s1, sd) = match self {
            Object::Curve{x0, y0, dx, dy, ..} => (
                Point{
                    x: x0.val(rng) as f32,
                    y: y0.val(rng) as f32,
                },
                Point{
                    x: dx.val(rng) as f32,
                    y: dy.val(rng) as f32,
                }),

            Object::Line{x0, y0, dx, dy, ..} => (
                Point{
                    x: x0.val(rng) as f32,
                    y: y0.val(rng) as f32,
                },
                Point{
                    x: dx.val(rng) as f32,
                    y: dy.val(rng) as f32,
                }),
        };

        let slope = dir.y / dir.x;
        let alpha = ((s1.x - origin.x) * slope + (origin.y - s1.y)) / (sd.y - sd.x * slope);
        if alpha < 0.0 { return None; }
        if alpha > 1.0 { return None; }

        let distance = (s1.x + sd.x * alpha - origin.x) / dir.x;
        if distance < 0.0 { return None; }

        let (hit, norm) = match self {
            Object::Line{..} => (
                Point {
                    x: origin.x + distance * dir.x,
                    y: origin.y + distance * dir.y,
                },
                Point {
                    x: -sd.y,
                    y: sd.x,
                }
            ),
            Object::Curve{a0, da, ..} => {
                let deg = a0.val(rng) + alpha as f64 * da.val(rng);
                let rad = deg * (PI / 180.0);
                (Point {
                    x: origin.x + distance * dir.x,
                    y: origin.y + distance * dir.y,
                },
                Point {
                    x: f32::cos(rad as f32),
                    y: f32::sin(rad as f32),
                })
            }
        };

        return Some((hit, norm));
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use scene::Material;
    use sampler::Sample;
    use aabb_quadtree::geom::Point;
    use prng::PRNG;

    #[test]
    fn hit_line_1() {
        let mut rng = PRNG::seed(0);

        let m = Material {
            d: 0.3, r: 0.3, t: 0.3, 
        };

        let obj = Object::Line{
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: &m,
        };

        let origin = Point {x: 10.0, y: 0.0};
        let dir    = Point {x: -1.0, y: 1.0};


        let a = obj.get_hit(&origin, &dir, &mut rng);

        assert!(a.is_some());
        let (a, b) = a.unwrap();
        
        //assert hit is at (5,5)
        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 5.0);

        //assert normal is (-10,10)
        assert_eq!(b.x, -10.0);
        assert_eq!(b.y, 10.0);
        //normal is not normalised
    }
}
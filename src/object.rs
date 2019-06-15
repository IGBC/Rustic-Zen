use geom::{Point, Rect, Vector, Matrix};
use pcg_rand::Pcg64Fast;
use sampler::Sample;
use std::f64::consts::PI;

use material::Material;

/// Holds a definition of an object
///
/// Interally contains the associated logic.

pub enum Object {
    /// Straight line varient
    Line {
        /// Material used
        material: Box<Material>,
        /// Starting x Position
        x0: Sample,
        /// Starting y Position
        y0: Sample,
        /// Length in x Axis
        dx: Sample,
        /// Length in y Axis
        dy: Sample,
    },
    /// Curve Varient, completely untested and in places inplemented
    /// as a straight line.
    ///
    /// **Do not Use!.**
    Curve {
        /// Material used
        material: Box<Material>,
        /// Starting x Position
        x0: Sample,
        /// Starting y Position
        y0: Sample,
        /// Starting angle
        a0: Sample,
        /// Length in x Axis
        dx: Sample,
        /// Length in y Axis
        dy: Sample,
        /// Starting ending angle? maybe?
        da: Sample,
    },
}

impl Object {
    /// Returns a rectangle enclosing The Object.
    /// Currently does not support curved lines, cos math is hard.
    #[inline(always)]
    pub fn bounds(&self) -> Rect {
        match self {
            Object::Curve { .. } => {
                // TODO: implement this,
                // its the same algorithm as for the line,
                // the curve will always fit in the box.
                unimplemented!();
            }
            Object::Line { x0, y0, dx, dy, .. } => {
                let mut x_s = [
                    x0.bounds().0,
                    x0.bounds().1,
                    x0.bounds().0 + dx.bounds().0,
                    x0.bounds().0 + dx.bounds().1,
                    x0.bounds().1 + dx.bounds().0,
                    x0.bounds().1 + dx.bounds().1,
                ];
                x_s.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut y_s = [
                    y0.bounds().0,
                    y0.bounds().1,
                    y0.bounds().0 + dy.bounds().0,
                    y0.bounds().0 + dy.bounds().1,
                    y0.bounds().1 + dy.bounds().0,
                    y0.bounds().1 + dy.bounds().1,
                ];
                y_s.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let p0 = Point {
                    x: x_s[0],
                    y: y_s[0],
                };

                let p1 = Point {
                    x: x_s[x_s.len()],
                    y: y_s[y_s.len()],
                };

                return Rect::from_points(&p0, &p1);
            }
        }
    }

    /**
     * Returns a reference to the material used in this object
     */
    pub fn get_material(&self) -> &Box<Material> {
        match self {
            Object::Curve { material, .. } => material,
            Object::Line { material, .. } => material,
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
    #[inline(always)]
    pub fn get_hit(
        &self,
        origin: &Point,
        dir: &Vector,
        rng: &mut Pcg64Fast,
    ) -> Option<(Point, Vector, f64)> {
        // Get s1 and sD from samples
        let (s1, sd) = match self {
            Object::Curve { x0, y0, dx, dy, .. } => (
                Point {
                    x: x0.val(rng),
                    y: y0.val(rng),
                },
                Point {
                    x: dx.val(rng),
                    y: dy.val(rng),
                },
            ),

            Object::Line { x0, y0, dx, dy, .. } => (
                Point {
                    x: x0.val(rng),
                    y: y0.val(rng),
                },
                Point {
                    x: dx.val(rng),
                    y: dy.val(rng),
                },
            ),
        };

        let mat_a = Matrix {
            a1: sd.x, b1: -dir.x,
            a2: sd.y, b2: -dir.y,
        };

        let omega = origin.clone() - s1;
        
        let result = match mat_a.inverse() {
            Some(r) => r * omega,
            None => {return None},
        };
        if (result.x >= 0.0) && (result.x <= 1.0) && (result.y > 0.0) {
        } else {
            return None;
        };

        let alpha = result.x;
        let distance = result.y;

        let (hit, norm) = match self {
            Object::Line { .. } => (
                Point {
                    x: origin.x + distance * dir.x,
                    y: origin.y + distance * dir.y,
                },
                Vector { x: -sd.y, y: sd.x },
            ),
            Object::Curve { a0, da, .. } => {
                let deg = a0.val(rng) + alpha as f64 * da.val(rng);
                let rad = deg * (PI / 180.0);
                (
                    Point {
                        x: origin.x + distance * dir.x,
                        y: origin.y + distance * dir.y,
                    },
                    Vector {
                        x: f64::cos(rad),
                        y: f64::sin(rad),
                    },
                )
            }
        };

        return Some((hit, norm, alpha));
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use geom::{Point, Vector};
    use material::HQZLegacy;
    use sampler::Sample;
    use rand::prelude::*;
    use pcg_rand::Pcg64Fast;

    #[test]
    /// Ray hits object test
    /// Test result should be a hit at (5,5)
    fn hit_line_1() {
        let mut rng = Pcg64Fast::from_entropy();

        let m = Box::new(HQZLegacy::new(0.3, 0.3, 0.3));

        let obj = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: m,
        };

        let origin = Point { x: 10.0, y: 0.0 };
        let dir = Vector { x: -1.0, y: 1.0 };

        let a = obj.get_hit(&origin, &dir, &mut rng);

        let (a, b, _) = a.expect("A was not meant to be `None`");

        //assert hit is at (5,5)
        assert_eq!(a.x, 5.0);
        assert_eq!(a.y, 5.0);

        //assert normal is (-10,10)
        assert_eq!(b.x, -10.0);
        assert_eq!(b.y, 10.0);
        //normal is not normalised
    }

    #[test]
    /// Vector misses the object test
    /// Test result should be None as Ray crosses dy/dx
    /// Past the end of the object.
    fn miss_line_1() {
        let mut rng = Pcg64Fast::from_entropy();

        let m = Box::new(HQZLegacy::new(0.3, 0.3, 0.3));

        let obj = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: m,
        };

        let origin = Point { x: 30.0, y: 0.0 };
        let dir = Vector { x: -1.0, y: 1.0 };

        let a = obj.get_hit(&origin, &dir, &mut rng);

        assert!(a.is_none());
    }

    #[test]
    /// Vector Going the wrong way test
    /// Test result should be None, as Ray is going 180°
    /// in the wrong direction to hit the object.
    fn miss_line_2() {
        let mut rng = Pcg64Fast::from_entropy();

        let m = Box::new(HQZLegacy::new(0.3, 0.3, 0.3));

        let obj = Object::Line {
            x0: Sample::Constant(0.0),
            y0: Sample::Constant(0.0),
            dx: Sample::Constant(10.0),
            dy: Sample::Constant(10.0),
            material: m,
        };

        let origin = Point { x: 10.0, y: 0.0 };
        let dir = Vector { x: 1.0, y: 1.0 };

        let a = obj.get_hit(&origin, &dir, &mut rng);

        assert!(a.is_none());
    }
}

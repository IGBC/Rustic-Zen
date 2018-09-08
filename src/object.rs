use aabb_quadtree::geom::{Point, Rect};
use scene::Material;
use sampler::Sample;

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
                // TODO: learn this.
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
}
use aabb_quadtree::QuadTree;
use aabb_quadtree::geom::{Rect, Point};
use object::Object;

struct Vec2 {
    x: f64, 
    y: f64,
}

struct Ray<'a> {
    origin: &'a Point,
    end: &'a Point,
}

struct ObjectMap<'a> {
    map: QuadTree<&'a Object<'a>>,
}

impl<'a> ObjectMap<'a> {
    pub fn new(viewport: Rect) -> Self {
        ObjectMap {
            map: QuadTree::default(viewport),
        }
    }

    pub fn add_object(&mut self, o: &'a Object) {
        let b_box = o.bounds();
        self.map.insert_with_box(o, b_box);
    }

    fn trace_ray(&self, ray: &mut Ray) {
        // so the 6 month old idea was to do a 2D binary search across the spacial, the problem is I can't remember why. 
        // I'm writing this while I am 10km in the air, and my most inteligent thought at this altitude was, even if
        // I was too lazy to implement a solution I should have at least written some notes into the code file. 

        // another idea is to use the intersection function buried in the Rect implementation shipped with the quadtree.
        // that works considering a ray also has a bounding box. But rn we don't know what it is, because we are still 
        // trying to determine the length of the ray. 

        // Up here you really can feel the low pressure when you try to really push your brain. Everything is a little slower.
        // so lets go back to that binary search...

        // So we know the start and direction of our ray. 
        //   Step 1: Divide the spacial into quadrants centered on the start, select relevent quadrant.
        //   Step 2: test box colision
        //   Step 2 (true) cut the space into two boxes along the line. 
        //            - test the first (closer) box, this occuludes the second box, divide that and repeat, 
        //              else you can't  assume the second box, as the two boxes don't complete the outer box. 
        //              second box failure pop up the stack by one level and behaves like the first box failed.
        //   Step 2 (False, or after false positive): exits the space, jump to rendering.
        //   Step 3: pixel tests are fucking evil. but that's what we have to do. Thankfully we're dealing with arc's not polys,
        //           so we can use similtanious equations, instead of an iterating test. 
        //   Step 4: record hit coordinates and continue to rendering. 

        // OK now I've had a drink too. Flying is a bad place to try to solve algorithmic problems. Hey at least its not architecture.
        
        //TODO, bound ray,
        let result = self.recurse(ray);
        match result {
            None => (),
            Some(p) => {ray.end = &p}, // <- BUG!!!
        };
    }

    fn recurse(&self, ray: &Ray) -> Option<Point> {
        //Step 2
        let ray_bb: Rect = Rect::from_points(&ray.origin, &ray.end);
        let queryset = self.map.query(ray_bb);
        match queryset.len() {
            // nothing in box, nothing to hit
            0 => None,
            // one object in box, check if we actually hit it.
            1 => {
                // Step 3
                // go get the bloody coords
                Some(Point{
                    //TODO: do precise collision
                    x: 0.0,
                    y: 0.0,
                })
            },
            // more than one hit split the ray and try with a smaller search
            _ => {
                let dy = ray.end.y - ray.origin.y;
                let dx = ray.end.x - ray.origin.x;
                let midpoint: Point = Point{
                    y: ray.origin.y + (dy/2.0),
                    x: ray.origin.x + (dx/2.0),
                };

                let box_a = Ray{
                    origin: ray.origin,
                    end: &midpoint,
                };

                let box_b = Ray{
                    origin: &midpoint,
                    end: ray.end,
                };
                
                let mut result = self.recurse(&box_a);
                // if first ray segment has no hit, try second segment.
                if result.is_none() { 
                    result = self.recurse(&box_b)
                };
                // should be Some(1) or None() if not it will propogate.
                result
            }
        }
    }
}
#![allow(dead_code)]
#![warn(missing_docs)]
extern crate aabb_quadtree;

#[cfg(test)]
extern crate rand;


mod sampler;
mod spectrum;
mod prng;
mod image;
mod scene;
mod object;
//mod raytrace;
mod niave_rt;
mod ray;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

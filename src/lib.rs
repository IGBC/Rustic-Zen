#![allow(dead_code)]
extern crate aabb_quadtree;

mod sampler;
mod spectrum;
mod prng;
mod image;
mod scene;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

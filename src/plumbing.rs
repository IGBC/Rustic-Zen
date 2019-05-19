use geom::{Point};

// Collider --> Rasteriser Pipe
#[derive(Copy, Clone)]
pub struct CompleteRay {
    pub start: Point,
    pub end: Point,
    pub wavelength: f64,    
}

// Collider --> Shader Pipe
#[derive(Copy, Clone)]
pub struct HitData {
    pub hit: Point,
    pub wavelength: f64,
    pub distance: f64,
    pub alpha: f64,
}


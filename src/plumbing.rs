use geom::{Point};

// Collider --> Rasteriser Pipe
pub struct CompleteRay {
    pub start: Point,
    pub end: Point,
    pub wavelength: f64,    
}

// Collider --> Shader Pipe
pub struct HitData {
    pub hit: Point,
    pub wavelength: f64,
    pub distance: f64,
    pub alpha: f64,
}


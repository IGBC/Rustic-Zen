use geom::{Point, Vector};

pub enum Message<T> {
    Next(T),
    Terminate,
}

// Collider --> Rasteriser Pipe
#[derive(Copy, Clone)]
pub struct CompleteRay {
    pub start: Point,
    pub end: Point,
    pub wavelength: f64,    
}

// Collider --> Shader Pipe

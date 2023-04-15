use nalgebra::Vector2;

pub mod parser;
pub mod particle;

pub const HOLE_POSITIONS: [Vector2<f64>; 6] = [
    Vector2::new(0.0, 0.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.5, 0.0),
    Vector2::new(0.5, 1.0),
];

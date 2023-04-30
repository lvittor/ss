use nalgebra::Vector2;

pub mod models;
pub mod parser;

#[cfg(feature = "use_f64")]
pub type Float = f64;
#[cfg(feature = "use_f32")]
pub type Float = f32;

pub const HOLE_POSITIONS: [Vector2<Float>; 6] = [
    Vector2::new(0.0, 0.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.5, 0.0),
    Vector2::new(0.5, 1.0),
];

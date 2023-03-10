use cgmath::{MetricSpace, Vector2};

pub type ID = usize;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub id: ID,
    pub position: Vector2<f64>,
    pub radius: f64,
}

#[derive(Debug)]
pub struct ParticlesData {
    pub space_length: f64,
    pub grid_size: usize,
    pub interaction_radius: f64,
    pub particles: Vec<Particle>,
}

impl Particle {
    pub fn is_within_distance_of(&self, other: &Self, radius: f64) -> bool {
        self.position.distance2(other.position) <= (radius + self.radius + other.radius).powi(2)
    }
}

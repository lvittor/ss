use cim::particles::{CircularParticle, ID};
use nalgebra::Vector2;

#[derive(Debug)]
pub struct ParticlesData {
    pub space_length: f64,
    pub grid_size: usize,
    pub interaction_radius: f64,
    pub particles: Vec<Particle>,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub id: ID,
    pub position: Vector2<f64>,
    pub radius: f64,
}

impl CircularParticle for Particle {
    fn get_id(&self) -> cim::particles::ID {
        self.id
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_position(&self) -> Vector2<f64> {
        self.position
    }
}

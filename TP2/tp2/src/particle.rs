use cgmath::Vector2;
use cim::particles::{ID, CircularParticle};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub id: ID,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

impl CircularParticle for Particle {
    fn get_id(&self) -> ID {
        self.id
    }

    fn get_radius(&self) -> f64 {
        0.0
    }

    fn get_position(&self) -> Vector2<f64> {
        self.position
    }
}


#[derive(Debug)]
pub struct InputData {
    pub rng_seed: Option<usize>,
    pub space_length: f64,
    pub interaction_radius: f64,
    pub noise: f64,
    pub particles: Vec<Particle>,
}


#[derive(Debug, Clone)]
pub struct Frame {
    pub time: f64,
    pub particles: Vec<Particle>,
}


use std::fmt::{Display};

use cim::particles::{CircularParticle, ID};
use nalgebra::Vector2;

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
    pub rng_seed: Option<u64>,
    pub space_length: f64,
    pub interaction_radius: f64,
    pub noise: f64,
    pub speed: f64,
    pub particles: Vec<Particle>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub time: f64,
    pub particles: Vec<Particle>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n", self.time))?;
        for particle in &self.particles {
            f.write_fmt(format_args!(
                "{} {} {} {} {}\n",
                particle.id,
                particle.position.x,
                particle.position.y,
                particle.velocity.x,
                particle.velocity.y
            ))?;
        }

        Ok(())
    }
}

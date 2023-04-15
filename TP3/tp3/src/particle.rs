use std::fmt::Display;

use cim::particles::ID;
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub id: ID,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

#[derive(Debug)]
pub struct InputData {
    pub table_width: f64,
    pub table_height: f64,
    pub hole_radius: f64,
    pub ball_radius: f64,
    pub ball_mass: f64,
    pub balls: Vec<Ball>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub time: f64,
    pub balls: Vec<Ball>,
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}\n", self.balls.len(), self.time))?;
        for particle in &self.balls {
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

use std::fmt::Display;

use cim::particles::ID;
use nalgebra::Vector2;

use crate::Float;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub id: ID,
    pub position: Vector2<Float>,
    pub velocity: Vector2<Float>,
}

#[derive(Debug)]
pub struct InputData {
    pub table_width: Float,
    pub table_height: Float,
    pub hole_radius: Float,
    pub ball_radius: Float,
    pub ball_mass: Float,
    pub balls: Vec<Ball>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub time: Float,
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

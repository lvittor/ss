use std::{fmt::Display, io::Write};

use cim::particles::{CircularParticle, ID};
use nalgebra::Vector2;

use crate::Float;

#[derive(Debug, Clone, Copy)]
pub struct Ball {
    pub id: ID,
    pub position: Vector2<Float>,
    pub velocity: Vector2<Float>,
    pub radius: Float,
}

impl CircularParticle for Ball {
    fn get_id(&self) -> ID {
        self.id
    }

    fn get_position(&self) -> Vector2<f64> {
        self.position
    }

    fn get_radius(&self) -> f64 {
        self.radius as f64
    }
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
        IterableFrame::from(self).write_fmt(f)
    }
}

pub struct IterableFrame<I> {
    pub time: Float,
    pub balls: I,
}

impl<'a> From<&'a Frame> for IterableFrame<std::slice::Iter<'a, Ball>> {
    fn from(frame: &'a Frame) -> Self {
        Self {
            time: frame.time,
            balls: frame.balls.iter(),
        }
    }
}

impl<'a, I: ExactSizeIterator<Item = &'a Ball>> IterableFrame<I> {
    pub fn write_fmt<W: std::fmt::Write>(self, f: &mut W) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}\n", self.balls.len(), self.time))?;
        for particle in self.balls {
            f.write_fmt(format_args!(
                "{} {} {} {} {}\n",
                particle.id,
                particle.position.x,
                particle.position.y,
                particle.velocity.x,
                particle.velocity.y,
            ))?;
        }

        Ok(())
    }
}

impl<'a, I: ExactSizeIterator<Item = &'a Ball>> IterableFrame<I> {
    pub fn write_to<W: Write>(self, f: &mut W) -> std::io::Result<()> {
        f.write_fmt(format_args!("{}\n{}\n", self.balls.len(), self.time))?;
        for particle in self.balls {
            f.write_fmt(format_args!(
                "{} {} {} {} {}\n",
                particle.id,
                particle.position.x,
                particle.position.y,
                particle.velocity.x,
                particle.velocity.y,
            ))?;
        }

        Ok(())
    }
}

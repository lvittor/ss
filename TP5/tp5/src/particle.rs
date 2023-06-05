use std::{fmt::Display, io::Write};

use cim::particles::{CircularParticle, ID};
use nalgebra::Vector2;

#[derive(Debug, Clone, Copy)]
pub enum ParticleTarget {
    Exit,
    FarExit,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub id: ID,
    pub position: Vector2<f64>,
    pub radius: f64,
    pub target: ParticleTarget,
}

impl CircularParticle for Particle {
    fn get_id(&self) -> ID {
        self.id
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn get_position(&self) -> Vector2<f64> {
        self.position
    }
}

#[derive(Debug)]
pub struct InputData {
    pub rng_seed: Option<u64>,
    pub room_side: f64,
    pub max_speed: f64,
    pub min_radius: f64,
    pub max_radius: f64,
    pub exit_size: f64,
    pub far_exit_distance: f64,
    pub far_exit_size: f64,
    pub particles: Vec<Particle>,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub time: f64,
    pub particles: Vec<Particle>,
}

pub struct IterableFrame<I> {
    pub time: f64,
    pub particles: I,
}

impl<'a> From<&'a Frame> for IterableFrame<std::slice::Iter<'a, Particle>> {
    fn from(frame: &'a Frame) -> Self {
        Self {
            time: frame.time,
            particles: frame.particles.iter(),
        }
    }
}

impl<'a, I: ExactSizeIterator<Item = &'a Particle>> IterableFrame<I> {
    pub fn write_fmt<W: std::fmt::Write>(self, f: &mut W) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}\n", self.particles.len(), self.time))?;
        for particle in self.particles {
            f.write_fmt(format_args!(
                "{} {} {} {}\n",
                particle.id, particle.position.x, particle.position.y, particle.radius,
            ))?;
        }

        Ok(())
    }
}

impl<'a, I: ExactSizeIterator<Item = &'a Particle>> IterableFrame<I> {
    pub fn write_to<W: Write>(self, f: &mut W) -> std::io::Result<()> {
        f.write_fmt(format_args!("{}\n{}\n", self.particles.len(), self.time))?;
        for particle in self.particles {
            f.write_fmt(format_args!(
                "{} {} {} {}\n",
                particle.id, particle.position.x, particle.position.y, particle.radius,
            ))?;
        }

        Ok(())
    }
}

use cgmath::{num_traits::Float, InnerSpace, Vector2};

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
    pub fn is_within_distance_of(
        &self,
        other: &Self,
        radius: f64,
        space_length: f64,
        cyclic: bool,
    ) -> bool {
        let mut delta = (self.position - other.position).map(Float::abs);
        if cyclic {
            if delta.x > 0.5 * space_length {
                delta.x = space_length - delta.x;
            }
            if delta.y > 0.5 * space_length {
                delta.y = space_length - delta.y;
            }
        }
        delta.magnitude2() <= (radius + self.radius + other.radius).powi(2)
    }
}

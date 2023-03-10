use cgmath::Vector2;


pub type ID = usize;

#[derive(Debug)]
pub struct Particle {
    pub id: ID,
    pub position: Vector2<f64>,
    pub radius: f64,
}

#[derive(Debug)]
pub struct ParticlesData {
    pub space_side: f64,
    pub grid_size: usize,
    pub interaction_radius: f64,
    pub particles: Vec<Particle>,
}

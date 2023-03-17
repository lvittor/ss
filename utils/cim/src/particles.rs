use cgmath::{num_traits::Float, InnerSpace, Vector2};

pub type ID = usize;

pub trait CircularParticle: Clone + Copy {
    fn get_id(&self) -> ID;
    fn get_position(&self) -> Vector2<f64>;
    fn get_radius(&self) -> f64;
    fn is_within_distance_of(
        &self,
        other: &Self,
        radius: f64,
        space_length: f64,
        cyclic: bool,
    ) -> bool {
        let mut delta = (self.get_position() - other.get_position()).map(Float::abs);
        if cyclic {
            if delta.x > 0.5 * space_length {
                delta.x = space_length - delta.x;
            }
            if delta.y > 0.5 * space_length {
                delta.y = space_length - delta.y;
            }
        }
        delta.magnitude2() <= (radius + self.get_radius() + other.get_radius()).powi(2)
    }
}

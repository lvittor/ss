use nalgebra::Vector2;

pub type ID = usize;

pub trait CircularParticle: Clone + Copy {
    fn get_id(&self) -> ID;
    fn get_position(&self) -> Vector2<f64>;
    fn get_radius(&self) -> f64;
    fn is_within_distance_of(
        &self,
        other: &Self,
        radius: f64,
        space_width: f64,
        space_height: f64,
        cyclic: bool,
    ) -> bool {
        let mut delta = self.get_position() - other.get_position();
        if cyclic {
            delta = delta.abs();
            if delta.x > 0.5 * space_width {
                delta.x = space_width - delta.x;
            }
            if delta.y > 0.5 * space_height {
                delta.y = space_height - delta.y;
            }
        }
        delta.magnitude_squared() <= (radius + self.get_radius() + other.get_radius()).powi(2)
    }
}

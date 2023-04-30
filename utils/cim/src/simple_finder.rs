use itertools::Itertools;

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{CircularParticle, ID},
};

pub struct SimpleNeighborFinder;

pub struct SystemInfo {
    pub cyclic: bool,
    pub interaction_radius: f64,
    pub space_width: f64,
    pub space_height: f64,
}

impl<P: CircularParticle> NeighborFinder<P, SystemInfo> for SimpleNeighborFinder {
    fn find_neighbors(particles: &[P], system: SystemInfo) -> NeighborMap<ID> {
        let mut map = NeighborMap::default();
        for (p1, p2) in particles.iter().tuple_combinations() {
            if p1.is_within_distance_of(
                p2,
                system.interaction_radius,
                system.space_width,
                system.space_height,
                system.cyclic,
            ) {
                map.add_pair(p1.get_id(), p2.get_id());
            }
        }

        map
    }
}

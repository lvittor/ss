use itertools::Itertools;

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{ID, Particle},
};

pub struct SimpleNeighborFinder;

pub struct SystemInfo {
    pub cyclic: bool,
    pub interaction_radius: f64,
    pub space_length: f64,
}

impl NeighborFinder<Particle, SystemInfo> for SimpleNeighborFinder {
    fn find_neighbors(particles: &[Particle], system: SystemInfo) -> NeighborMap<ID> {
        let mut map = NeighborMap::default();
        for (p1, p2) in particles.iter().tuple_combinations() {
            if p1.is_within_distance_of(p2, system.interaction_radius, system.space_length, system.cyclic) {
                map.add_pair(p1.id, p2.id);
            }
        }

        map
    }
}

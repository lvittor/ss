use itertools::Itertools;

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{ParticlesData, ID},
};

pub struct SimpleNeighborFinder;

impl NeighborFinder<ParticlesData, ID> for SimpleNeighborFinder {
    fn find_neighbors(particles: &ParticlesData) -> NeighborMap<ID> {
        let mut map = NeighborMap::default();
        for (p1, p2) in particles.particles.iter().tuple_combinations() {
            if p1.is_within_distance_of(p2, particles.interaction_radius) {
                map.add_pair(p1.id, p2.id);
            }
        }

        map
    }
}

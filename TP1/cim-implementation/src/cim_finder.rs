use cgmath::{num_traits::Euclid, vec2, Vector2};
use ndarray::{Array2, Dim};

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{Particle, ParticlesData, ID},
};

pub struct CimNeighborFinder;

impl NeighborFinder<ParticlesData, ID> for CimNeighborFinder {
    fn find_neighbors(particles: &ParticlesData, cyclic: bool) -> NeighborMap<ID> {
        let mut cells: Array2<Vec<Particle>> =
            Array2::default(Dim([particles.grid_size, particles.grid_size]));
        let cell_length = particles.space_length / particles.grid_size as f64;

        let get_cell_index = |particle: &Particle| {
            particle
                .position
                .map(|v| (v / cell_length).floor() as usize)
        };
        let get_cells_to_check = |cell_index: Vector2<usize>| {
            [vec2(0, 0), vec2(1, 0), vec2(1, 1), vec2(0, 1), vec2(-1, 1)]
                .into_iter()
                .filter_map(move |v| {
                    let v = v + cell_index.cast().unwrap();
                    if cyclic {
                        Some(v.map(|v| v.rem_euclid(&(particles.grid_size as i32)) as usize))
                    } else {
                        (v.x >= 0
                            && v.y >= 0
                            && (v.x as usize) < particles.grid_size
                            && (v.y as usize) < particles.grid_size)
                            .then(|| v.cast().unwrap())
                    }
                })
        };

        for particle in &particles.particles {
            let cell_index = get_cell_index(particle);
            cells[(cell_index.y, cell_index.x)].push(*particle);
        }

        let mut map = NeighborMap::default();

        for particle in &particles.particles {
            let cell_index = get_cell_index(particle);
            for cell in get_cells_to_check(cell_index) {
                for other in &cells[(cell.y, cell.x)] {
                    if other.id > particle.id {
                        if particle.is_within_distance_of(
                            other,
                            particles.interaction_radius,
                            particles.space_length,
                            cyclic,
                        ) {
                            map.add_pair(particle.id, other.id);
                        }
                    }
                }
            }
        }

        map
    }
}

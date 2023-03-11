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
                    let new_index = v + cell_index.cast().unwrap();
                    if cyclic {
                        Some(
                            new_index.map(|v| v.rem_euclid(&(particles.grid_size as i32)) as usize),
                        )
                    } else {
                        (new_index.x >= 0
                            && new_index.y >= 0
                            && (new_index.x as usize) < particles.grid_size
                            && (new_index.y as usize) < particles.grid_size)
                            .then(|| new_index.cast().unwrap())
                    }
                })
        };

        // Fill the cell matrix with particles.
        for particle in &particles.particles {
            let cell_index = get_cell_index(particle);
            cells[(cell_index.y, cell_index.x)].push(*particle);
        }

        let mut map = NeighborMap::default();

        for particle in &particles.particles {
            let cell_index = get_cell_index(particle);
            for other_cell in get_cells_to_check(cell_index) {
                for other in &cells[(other_cell.y, other_cell.x)] {
                    // If we are in the same cell, we only check the same pair once.
                    if (other_cell != cell_index || other.id > particle.id)
                        && particle.is_within_distance_of(
                            other,
                            particles.interaction_radius,
                            particles.space_length,
                            cyclic,
                        )
                    {
                        map.add_pair(particle.id, other.id);
                    }
                }
            }
        }

        map
    }
}

use std::ops::Div;

use nalgebra::Vector2;
use ndarray::{Array2, Dim};

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{CircularParticle, ID},
};

pub struct CimNeighborFinder;

pub struct SystemInfo {
    pub cyclic: bool,
    pub interaction_radius: f64,
    pub space_length: f64,
    pub grid_size: usize,
}

impl<P: CircularParticle> NeighborFinder<P, SystemInfo> for CimNeighborFinder {
    fn find_neighbors(particles: &[P], system: SystemInfo) -> NeighborMap<ID> {
        let mut cells: Array2<Vec<P>> = Array2::default(Dim([system.grid_size, system.grid_size]));
        let cell_length = system.space_length / system.grid_size as f64;

        let get_cell_index = |particle: &P| -> Vector2<usize> {
            particle
                .get_position()
                .div(cell_length)
                .apply_into(|v| *v = v.floor())
                .try_cast()
                .unwrap()
        };
        let get_cells_to_check = |cell_index: Vector2<usize>| {
            [
                Vector2::new(0i32, 0),
                Vector2::new(1, 0),
                Vector2::new(1, 1),
                Vector2::new(0, 1),
                Vector2::new(-1, 1),
            ]
            .into_iter()
            .filter_map(move |v| {
                let new_index = v + cell_index.cast();
                if system.cyclic {
                    Some(new_index.map(|v| v.rem_euclid(system.grid_size as i32) as usize))
                } else {
                    (new_index.x >= 0
                        && new_index.y >= 0
                        && (new_index.x as usize) < system.grid_size
                        && (new_index.y as usize) < system.grid_size)
                        .then(|| new_index.try_cast().unwrap())
                }
            })
        };

        // Fill the cell matrix with particles.
        for particle in particles {
            let cell_index = get_cell_index(particle);
            cells[(cell_index.y, cell_index.x)].push(*particle);
        }

        let mut map = NeighborMap::default();

        for particle in particles {
            let cell_index = get_cell_index(particle);
            for other_cell in get_cells_to_check(cell_index) {
                for other in &cells[(other_cell.y, other_cell.x)] {
                    // If we are in the same cell, we only check the same pair once.
                    if (other_cell != cell_index || other.get_id() > particle.get_id())
                        && particle.is_within_distance_of(
                            other,
                            system.interaction_radius,
                            system.space_length,
                            system.cyclic,
                        )
                    {
                        map.add_pair(particle.get_id(), other.get_id());
                    }
                }
            }
        }

        map
    }
}

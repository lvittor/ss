use std::collections::BTreeMap;

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    neighbor_finder::{NeighborFinder, NeighborMap},
    particles::{CircularParticle, ID},
};

pub struct CimNeighborFinder;

pub struct SystemInfo {
    pub cyclic: bool,
    pub interaction_radius: f64,
    pub space_width: f64,
    pub space_height: f64,
    pub columns: usize,
    pub rows: usize,
}

impl<P: CircularParticle> NeighborFinder<P, SystemInfo> for CimNeighborFinder {
    fn find_neighbors(particles: &[P], system: SystemInfo) -> NeighborMap<ID> {
        let mut cells: BTreeMap<(_, _), Vec<P>> = BTreeMap::new();

        let cell_width = system.space_width / system.columns as f64;
        let cell_height = system.space_height / system.rows as f64;
        let get_cell_index = |particle: &P| -> Vector2<usize> {
            particle
                .get_position()
                .component_div(&Vector2::new(cell_width, cell_height))
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
                    Some(Vector2::new(
                        new_index.x.rem_euclid(system.columns as i32) as usize,
                        new_index.y.rem_euclid(system.rows as i32) as usize,
                    ))
                } else {
                    (new_index.x >= 0
                        && new_index.y >= 0
                        && (new_index.x as usize) < system.columns
                        && (new_index.y as usize) < system.rows)
                        .then(|| new_index.try_cast().unwrap())
                }
            })
        };

        // Fill the cell matrix with particles.
        for particle in particles {
            let cell_index = get_cell_index(particle);
            cells
                .entry((cell_index.y, cell_index.x))
                .or_insert_with(|| Vec::with_capacity(2))
                .push(*particle);
        }

        let mut map = NeighborMap::default();

        for (cell_index, cell) in &cells {
            let cell_index = Vector2::new(cell_index.1, cell_index.0);
            for other_cell_index in get_cells_to_check(cell_index) {
                if let Some(other_cell) = cells.get(&(other_cell_index.y, other_cell_index.x)) {
                    for (particle, other) in cell.iter().cartesian_product(other_cell.iter()) {
                        // If we are in the same cell, we only check the same pair once.
                        if (other_cell_index != cell_index || other.get_id() > particle.get_id())
                            && particle.is_within_distance_of(
                                other,
                                system.interaction_radius,
                                system.space_width,
                                system.space_height,
                                system.cyclic,
                            )
                        {
                            map.add_pair(particle.get_id(), other.get_id());
                        }
                    }
                }
            }
        }

        map
    }
}

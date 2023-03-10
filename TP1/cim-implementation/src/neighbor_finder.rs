use std::{
    collections::HashMap,
    fmt::{Display, Write},
    hash::Hash,
    iter,
};

pub trait NeighborFinder<Particles, ID> {
    fn find_neighbors(particles: &Particles) -> NeighborMap<ID>;
}

#[derive(Debug, Default)]
pub struct NeighborMap<ID> {
    map: HashMap<ID, Vec<ID>>,
}

impl<ID: Hash + Eq + Copy> NeighborMap<ID> {
    pub fn add_pair(&mut self, p1: ID, p2: ID) {
        self.map.entry(p1).or_default().push(p2);
        self.map.entry(p2).or_default().push(p1);
    }
}

impl<ID: ToString> Display for NeighborMap<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (particle, neighbors) in &self.map {
            f.write_str(
                &iter::once(particle)
                    .chain(neighbors)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            )?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

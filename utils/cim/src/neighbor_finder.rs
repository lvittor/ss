use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    hash::Hash,
    iter,
};

pub trait NeighborFinder<Particle, SystemInfo> {
    fn find_neighbors(particles: &[Particle], system: SystemInfo) -> NeighborMap<usize>;
}

#[derive(Debug, Default)]
pub struct NeighborMap<ID> {
    map: HashMap<ID, HashSet<ID>>,
}

impl<ID: Hash + Eq + Copy> NeighborMap<ID> {
    pub fn new(map: HashMap<ID, HashSet<ID>>) -> Self {
        Self { map }
    }

    pub fn add_pair(&mut self, p1: ID, p2: ID) {
        self.map.entry(p1).or_default().insert(p2);
        self.map.entry(p2).or_default().insert(p1);
    }

    pub fn has_pair(&self, p1: ID, p2: ID) -> bool {
        self.map.get(&p1).is_some_and(|s| s.contains(&p2))
    }

    pub fn get_neighbors(&self, p1: ID) -> &HashSet<ID> {
        &self.map[&p1]
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

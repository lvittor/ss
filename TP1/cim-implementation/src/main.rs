use cgmath::Vector2;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Write},
    io::{stdin, Read},
    iter,
    str::FromStr,
};

type ID = usize;

struct Particle {
    id: ID,
    position: Vector2<f64>,
    radius: f64,
}

struct ParticlesData {
    n: usize,
    l: f64,
    m: usize,
    r_c: f64,
    particles: Vec<Particle>,
}

impl FromStr for ParticlesData {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        todo!("Implement parser");
    }
}

struct NeighborMap {
    map: HashMap<ID, Vec<ID>>,
}

impl Display for NeighborMap {
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

impl ParticlesData {
    fn generate_neighbor_map(&self) -> NeighborMap {
        todo!("Implement neighbor search");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input: ParticlesData = input.parse().expect("Error parsing input data.");

    let output = input.generate_neighbor_map();

    print!("{output}");
}

use cgmath::{vec2, Vector2};
use chumsky::{prelude::*, text::newline};
use std::{
    collections::HashMap,
    fmt::{Display, Write},
    io::{stdin, Read},
    iter,
};

type ID = usize;

#[derive(Debug)]
struct Particle {
    id: ID,
    position: Vector2<f64>,
    radius: f64,
}

#[derive(Debug)]
struct ParticlesData {
    n: usize,
    l: f64,
    m: usize,
    r_c: f64,
    particles: Vec<Particle>,
}

fn parser<'a>() -> impl Parser<'a, &'a str, ParticlesData, extra::Err<Rich<'a, char>>> {
    let digits = text::digits(10);
    let unsigned = digits.map_slice(|s: &str| s.parse::<usize>().unwrap());

    let num = just('-')
        .or_not()
        .then(text::int(10))
        .then(just('.').then(digits).or_not())
        .map_slice(|s: &str| s.parse().unwrap());

    let particle_data = unsigned
        .then_ignore(just(' '))
        .then(num.separated_by_exactly::<_, _, 3>(just(' ')))
        .map(|(id, [x, y, r])| Particle {
            id,
            position: vec2(x, y),
            radius: r,
        });

    let particles = particle_data
        .separated_by(newline())
        .at_least(1)
        .allow_trailing()
        .collect();

    unsigned
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(unsigned)
        .then_ignore(newline())
        .then(num)
        .map(|(((n, l), m), r_c)| (n, l, m, r_c))
        .then_ignore(newline())
        .then(particles)
        .map(|((n, l, m, r_c), particles)| ParticlesData {
            n,
            l,
            m,
            r_c,
            particles,
        })
        .then_ignore(end())
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
    let input: ParticlesData = parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    dbg!(&input);

    let output = input.generate_neighbor_map();

    print!("{output}");
}

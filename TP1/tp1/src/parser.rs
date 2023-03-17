use std::collections::{HashMap, HashSet};

use cgmath::vec2;
use chumsky::{prelude::*, text::newline};

use cim::{neighbor_finder::NeighborMap, particles::ID};

use crate::particle::{Particle, ParticlesData};

pub fn input_parser<'a>() -> impl Parser<'a, &'a str, ParticlesData, extra::Err<Rich<'a, char>>> {
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
        .map(
            |((_, space_length, grid_size, interaction_radius), particles)| ParticlesData {
                space_length,
                grid_size,
                interaction_radius,
                particles,
            },
        )
        .then_ignore(end())
}

pub fn output_parser<'a>() -> impl Parser<'a, &'a str, NeighborMap<ID>, extra::Err<Rich<'a, char>>>
{
    let digits = text::digits(10);
    let unsigned = digits.map_slice(|s: &str| s.parse::<usize>().unwrap());

    let line = unsigned.then_ignore(just(' ')).then(
        unsigned
            .separated_by(just(' '))
            .at_least(0)
            .collect::<HashSet<_>>(),
    );

    line.separated_by(newline())
        .allow_trailing()
        .collect::<Vec<_>>()
        .map(|lines| NeighborMap::new(HashMap::from_iter(lines)))
        .then_ignore(end())
}

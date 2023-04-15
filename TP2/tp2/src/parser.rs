use std::io::{BufRead, Lines};

use crate::particle::{Frame, InputData, Particle};
use chumsky::{prelude::*, text::newline};
use cim::particles::ID;
use itertools::Itertools;
use nalgebra::{Rotation2, Vector2};

pub fn input_parser<'a>() -> impl Parser<'a, &'a str, InputData, extra::Err<Rich<'a, char>>> {
    let digits = text::digits(10);
    let unsigned = digits.map_slice(|s: &str| s.parse::<usize>().unwrap());
    let unsigned64 = digits.map_slice(|s: &str| s.parse::<u64>().unwrap());
    let seed = just("any").to(None).or(unsigned64.map(Some));

    let num = just('-')
        .or_not()
        .then(text::int(10))
        .then(just('.').then(digits).or_not())
        .map_slice(|s: &str| s.parse().unwrap());

    let particle_data = unsigned
        .then_ignore(just(' '))
        .then(num.separated_by_exactly::<_, _, 3>(just(' ')))
        .map(|(id, [x, y, a])| Particle {
            id,
            position: Vector2::new(x, y),
            velocity_direction: Rotation2::new(a).transform_vector(&Vector2::x()),
        });

    let particles = particle_data
        .separated_by(newline())
        .at_least(1)
        .allow_trailing()
        .collect();

    seed.then_ignore(newline())
        .then(unsigned)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .map(|(((((seed, n), l), r_c), noise), speed)| (seed, n, l, r_c, noise, speed))
        .then_ignore(newline())
        .then(particles)
        .map(
            |((rng_seed, _, space_length, interaction_radius, noise, speed), particles): (
                _,
                Vec<Particle>,
            )| {
                InputData {
                    rng_seed,
                    space_length,
                    interaction_radius,
                    noise,
                    speed,
                    particles,
                }
            },
        )
        .then_ignore(end())
}

//pub fn output_parser<'a, I: Iterator<Item = char>>(
//particle_count: usize,
//) -> impl IterParser<'a, Stream<I>, Frame> {
//let digits = text::digits(10);
//let unsigned = digits.map(|s: &str| s.parse::<usize>().unwrap());
//let num = just('-')
//.or_not()
//.then(text::digits(10))
//.then(just('.').then(digits).or_not())
//.map_slice(|s: &str| s.parse().unwrap());

//let particle_data = unsigned
//.then_ignore(just(' '))
//.then(num.separated_by_exactly::<_, _, 4>(just(' ')))
//.map(|(id, [x, y, vx, vy])| Particle {
//id,
//position: vec2(x, y),
//velocity: vec2(vx, vy),
//});

//let frame = num
//.then(
//particle_data
//.separated_by(newline())
//.exactly(particle_count)
//.collect(),
//)
//.map(|(time, particles)| Frame { time, particles });

//frame.separated_by(newline())
//}

struct Chunks<I> {
    inner: I,
    size: usize,
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = vec![];
        for _ in 0..self.size {
            if let Some(value) = self.inner.next() {
                values.push(value);
            } else {
                return None;
            }
        }
        Some(values)
    }
}

trait CollectedChunks: Iterator + Sized {
    fn collected_chunks(self, size: usize) -> Chunks<Self> {
        Chunks { inner: self, size }
    }
}

impl<I: Iterator> CollectedChunks for I {}

pub fn output_parser<B: BufRead>(
    particle_count: usize,
    file: Lines<B>,
) -> impl Iterator<Item = Frame> {
    file.map(Result::unwrap)
        .collected_chunks(particle_count + 1)
        .map(|frame| {
            let mut frame = frame.into_iter();
            let time: f64 = frame.next().unwrap().parse().unwrap();
            let particles = frame
                .map(|line| {
                    let mut values = line.split_whitespace();
                    let id: ID = values.next().unwrap().parse().unwrap();
                    let [x, y, vx, vy]: [f64; 4] = values
                        .map(|v| v.parse().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap();
                    Particle {
                        id,
                        position: Vector2::new(x, y),
                        velocity_direction: Vector2::new(vx, vy),
                    }
                })
                .collect_vec();
            Frame { time, particles }
        })
}

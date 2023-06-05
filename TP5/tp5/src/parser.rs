use std::io::{BufRead, Lines};

use crate::particle::{Frame, InputData, Particle, ParticleTarget};
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
        .then(num.separated_by_exactly::<_, _, 2>(just(' ')))
        .map(|(id, [x, y])| Particle {
            id,
            position: Vector2::new(x, y),
            radius: 0.0,
            target: ParticleTarget::Exit,
        });

    let particles = particle_data
        .separated_by(newline())
        .at_least(1)
        .allow_trailing()
        .collect();

    seed.then_ignore(newline())
        .then_ignore(unsigned)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(particles)
        .map(
            |(
                (
                    (
                        (((((rng_seed, room_side), max_speed), min_radius), max_radius), exit_size),
                        far_exit_distance,
                    ),
                    far_exit_size,
                ),
                mut particles,
            ): (_, Vec<Particle>)| {
                particles.iter_mut().for_each(|p| p.radius = min_radius);
                InputData {
                    rng_seed,
                    room_side,
                    max_speed,
                    min_radius,
                    max_radius,
                    exit_size,
                    far_exit_distance,
                    far_exit_size,
                    particles,
                }
            },
        )
        .then_ignore(end())
}

struct Chunks<I> {
    inner: I,
}

impl<I: Iterator<Item = String>> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = vec![];
        if let Some(count) = self.inner.next().map(|v| v.parse::<usize>().unwrap()) {
            for _ in 0..count + 1 {
                if let Some(value) = self.inner.next() {
                    values.push(value);
                } else {
                    return None;
                }
            }
            Some(values)
        } else {
            None
        }
    }
}

trait CollectedChunks: Iterator + Sized {
    fn collected_chunks(self) -> Chunks<Self> {
        Chunks { inner: self }
    }
}

impl<I: Iterator> CollectedChunks for I {}

pub fn output_parser<B: BufRead>(file: Lines<B>) -> impl Iterator<Item = Frame> {
    file.map(Result::unwrap).collected_chunks().map(|frame| {
        let mut frame = frame.into_iter();
        let time: f64 = frame.next().unwrap().parse().unwrap();
        let particles = frame
            .map(|line| {
                let mut values = line.split_whitespace();
                let id: ID = values.next().unwrap().parse().unwrap();
                let [x, y, radius]: [f64; 3] = values
                    .map(|v| v.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap();
                Particle {
                    id,
                    position: Vector2::new(x, y),
                    radius,
                    target: ParticleTarget::Exit,
                }
            })
            .collect_vec();
        Frame { time, particles }
    })
}

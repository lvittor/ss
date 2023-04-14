use std::io::{BufRead, Lines};

use crate::particle::{Ball, Frame, InputData};
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

    let ball_data = unsigned
        .then_ignore(just(' '))
        .then(num.separated_by_exactly::<_, _, 4>(just(' ')))
        .map(|(id, [x, y, vx, vy])| Ball {
            id,
            position: Vector2::new(x, y),
            velocity: Vector2::new(vx, vy),
        });

    let balls = ball_data
        .separated_by(newline())
        .at_least(1)
        .allow_trailing()
        .collect();

    num.then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(num.map(|v| v / 2.0))
        .then_ignore(newline())
        .then(num)
        .then_ignore(newline())
        .then(unsigned)
        .map(|((((w, h), r), m), n)| (w, h, d, m, n))
        .then_ignore(newline())
        .then(balls)
        .map(
            |((table_width, table_height, ball_radius, ball_mass, _), balls): (_, Vec<Ball>)| {
                InputData {
                    table_width,
                    table_height,
                    ball_radius,
                    ball_mass,
                    balls,
                }
            },
        )
        .then_ignore(end())
}

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
        .into_iter()
        .collected_chunks(particle_count + 1)
        .map(|frame| {
            let mut frame = frame.into_iter();
            let time: f64 = frame.next().unwrap().parse().unwrap();
            let count: f64 = frame.next().unwrap().parse().unwrap();
            let balls = frame
                .map(|line| {
                    let mut values = line.split_whitespace();
                    let id: ID = values.next().unwrap().parse().unwrap();
                    let [x, y, vx, vy]: [f64; 4] = values
                        .map(|v| v.parse().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap();
                    Ball {
                        id,
                        position: Vector2::new(x, y),
                        velocity: Vector2::new(vx, vy),
                    }
                })
                .collect_vec();
            Frame { time, balls }
        })
}

use crate::particle::{Frame, InputData, Particle};
use cgmath::vec2;
use chumsky::{prelude::*, text::newline};

pub fn input_parser<'a>() -> impl Parser<'a, &'a str, InputData, extra::Err<Rich<'a, char>>> {
    let digits = text::digits(10);
    let unsigned = digits.map_slice(|s: &str| s.parse::<usize>().unwrap());
    let seed = just("any").to(None).or(unsigned.map(Some));

    let num = just('-')
        .or_not()
        .then(text::int(10))
        .then(just('.').then(digits).or_not())
        .map_slice(|s: &str| s.parse().unwrap());

    let particle_data = unsigned
        .then_ignore(just(' '))
        .then(num.separated_by_exactly::<_, _, 4>(just(' ')))
        .map(|(id, [x, y, vx, vy])| Particle {
            id,
            position: vec2(x, y),
            velocity: vec2(vx, vy),
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
        .map(|((((seed, n), l), r_c), noise)| (seed, n, l, r_c, noise))
        .then_ignore(newline())
        .then(particles)
        .map(
            |((rng_seed, _, space_length, interaction_radius, noise), particles)| InputData {
                rng_seed,
                space_length,
                interaction_radius,
                particles,
                noise,
            },
        )
        .then_ignore(end())
}

pub fn output_parser<'a>(particle_count: usize) -> impl IterParser<'a, &'a str, Frame> {
    let digits = text::digits(10);
    let unsigned = digits.map_slice(|s: &str| s.parse::<usize>().unwrap());
    let num = just('-')
        .or_not()
        .then(text::int(10))
        .then(just('.').then(digits).or_not())
        .map_slice(|s: &str| s.parse().unwrap());

    let particle_data = unsigned
        .then_ignore(just(' '))
        .then(num.separated_by_exactly::<_, _, 4>(just(' ')))
        .map(|(id, [x, y, vx, vy])| Particle {
            id,
            position: vec2(x, y),
            velocity: vec2(vx, vy),
        });

    let frame = num
        .then(
            particle_data
                .separated_by(newline())
                .exactly(particle_count)
                .collect(),
        )
        .map(|(time, particles)| Frame { time, particles });

    frame.separated_by(newline())
}

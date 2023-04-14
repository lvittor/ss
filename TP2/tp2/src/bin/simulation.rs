use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{stdout, Write},
    iter,
};

use chumsky::Parser;
use cim::{cim_finder::CimNeighborFinder, neighbor_finder::NeighborFinder, particles::ID};
use itertools::Itertools;
use nalgebra::{Rotation2, Vector2};
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use tp2::{
    parser::input_parser,
    particle::{Frame, InputData, Particle},
};

use clap::Parser as _parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    max_duration: Option<f64>,
}

fn run<W: Write, F: FnMut(&BTreeMap<ID, Particle>, f64) -> bool>(
    config: InputData,
    mut output_writer: W,
    mut stop_condition: F,
) {
    let dt = 1.0;
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = config.particles.into_iter().map(|p| (p.id, p)).collect();
    let mut rng = if let Some(seed) = config.rng_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    };

    while !stop_condition(&state, time) {
        let m = (config.space_length / config.interaction_radius).floor() as usize;
        let neighbors = CimNeighborFinder::find_neighbors(
            &state.values().cloned().collect_vec(),
            cim::cim_finder::SystemInfo {
                cyclic: true,
                interaction_radius: config.interaction_radius,
                space_length: config.space_length,
                grid_size: m,
            },
        );

        let mut new_state = BTreeMap::new();
        for (&id, particle) in &state {
            let sums = neighbors
                .get_neighbors(id)
                .chain(iter::once(&id))
                .map(|i| state[i])
                .map(|n| n.velocity_direction)
                .sum::<Vector2<_>>();

            let angle = f64::atan2(sums.y, sums.x)
                + rng.sample(Uniform::new_inclusive(
                    -config.noise / 2.0,
                    config.noise / 2.0,
                ));

            let new_velocity = Rotation2::new(angle).transform_vector(&Vector2::x());

            new_state.insert(
                id,
                Particle {
                    id,
                    position: (particle.position + particle.velocity_direction * config.speed * dt)
                        .apply_into(|f| *f = f.rem_euclid(config.space_length)),
                    velocity_direction: new_velocity,
                },
            );
        }
        let frame = Frame {
            time,
            particles: state.values().cloned().collect_vec(),
        };
        output_writer.write_fmt(format_args!("{frame}")).unwrap();
        state = new_state;
        time += dt;
    }
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let writer = if let Some(output) = args.output {
        Box::new(File::create(output).unwrap()) as Box<dyn Write>
    } else {
        Box::new(stdout())
    };

    run(input, writer, |_state, t| {
        args.max_duration
            .is_some_and(|max_duration| t > max_duration)
    });
}

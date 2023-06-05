use std::fs::File;
use std::{collections::BTreeMap, io::Write, path::PathBuf};
use std::{fs, iter};

use chumsky::Parser;
use cim::{cim_finder::CimNeighborFinder, particles::ID};
use clap::{Args, Parser as _parser, Subcommand};
use nalgebra::Vector2;
use rand::SeedableRng;
use rand::{distributions::Uniform, rngs::StdRng};
use tp5::parser::input_parser;
use tp5::particle::{InputData as SimpleInputData, IterableFrame, Particle};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output_particles: PathBuf,

    #[arg(short, long)]
    output_exit_times: PathBuf,

    #[arg(short, long)]
    steps_per_second: u16,

    #[arg(long)]
    output_every: u64,
    #[arg(long)]
    output_last: bool,
}

struct InputData {
    simple_input_data: SimpleInputData,
    steps_per_second: u16,
    output_every: u64,
    output_last: bool,
}

fn run<W: Write, W2: Write, F: FnMut(&BTreeMap<ID, Particle>, f64) -> bool>(
    config: InputData,
    mut output_particles: W,
    mut output_exit_times: W2,
    mut stop_condition: F,
) {
    let dt = 1.0;
    let mut iteration = 0;
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = config
        .simple_input_data
        .particles
        .into_iter()
        .map(|p| (p.id, p))
        .collect();
    let mut rng = if let Some(seed) = config.simple_input_data.rng_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    };

    // Write to output
    IterableFrame {
        time,
        particles: state.values(),
    }
    .write_to(&mut output_particles)
    .unwrap();

    while !stop_condition(&state, time) {
        iteration += 1;
        if iteration % config.output_every == 0 {
            // Write to output
            IterableFrame {
                time,
                particles: state.values(),
            }
            .write_to(&mut output_particles)
            .unwrap();
        }
    }

    // Write last frame in case it wasnt
    if config.output_last && iteration % config.output_every != 0 {
        IterableFrame {
            time,
            particles: state.values(),
        }
        .write_to(&mut output_particles)
        .unwrap();
    }
}

fn main() {
    let args = Arguments::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input = InputData {
        simple_input_data: input_parser()
            .parse(&input)
            .into_result()
            .expect("Error parsing input data."),
        steps_per_second: args.steps_per_second,
        output_every: args.output_every,
        output_last: args.output_last,
    };

    run(
        input,
        File::create(args.output_particles).unwrap(),
        File::create(args.output_exit_times).unwrap(),
        |_state, t| false,
    );
}

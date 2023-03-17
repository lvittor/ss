use std::{collections::HashMap, fs, iter, ops::RemAssign};

use chumsky::Parser;
use cim::{
    cim_finder::CimNeighborFinder,
    neighbor_finder::{self, NeighborFinder},
};
use itertools::Itertools;
use nalgebra::{AbstractRotation, Rotation2, Vector2};
use rand::{distributions::Uniform, Rng};
use tp2::{
    parser::input_parser,
    particle::{InputData, Particle, Frame},
};

use clap::Parser as _parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn run(config: InputData) {
    let dt = 1.0;
    let mut time = 0.0;
    let mut state: HashMap<_, _> = config.particles.into_iter().map(|p| (p.id, p)).collect();
    let mut rng = rand::thread_rng();
    loop {
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

        let mut new_state = HashMap::new();
        for (&id, particle) in &state {
            let mut cos_sum = 0.0;
            let mut sin_sum = 0.0;
            let particle_neighbors = neighbors.get_neighbors(id);
            for neighbor in particle_neighbors
                .iter()
                .chain(iter::once(&id))
                .map(|i| state[i])
            {
                cos_sum += neighbor.position.x / 0.03;
                sin_sum += neighbor.position.y / 0.03;
            }
            let angle = f64::atan2(sin_sum, cos_sum)
                + rng.sample(Uniform::new_inclusive(
                    -config.noise / 2.0,
                    config.noise / 2.0,
                ));

            let new_velocity = Rotation2::new(angle).transform_vector(&Vector2::new(0.03, 0.0));

            new_state.insert(
                id,
                Particle {
                    id,
                    position: (particle.position + particle.velocity * dt)
                        .apply_into(|f| *f = f.rem_euclid(config.space_length)),
                    velocity: new_velocity,
                },
            );
        }
        print!("{}", Frame {
            time,
            particles: state.values().cloned().collect_vec()
        });
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

    run(input);
}

#![feature(btree_drain_filter)]

use std::collections::HashMap;
use std::fs::File;
use std::{collections::BTreeMap, io::Write, path::PathBuf};
use std::{fs, iter};

use chumsky::Parser;
use cim::{cim_finder::CimNeighborFinder, particles::ID};
use clap::{Args, Parser as _parser, Subcommand};
use itertools::Itertools;
use nalgebra::Vector2;
use nannou::prelude::Pow;
use rand::{distributions::Uniform, rngs::StdRng};
use rand::{Rng, SeedableRng};
use tp5::parser::input_parser;
use tp5::particle::{InputData as SimpleInputData, IterableFrame, Particle, ParticleTarget};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output_particles: PathBuf,

    #[arg(short, long)]
    output_exit_times: PathBuf,

    #[arg(long)]
    outputs_per_second: u16,
    #[arg(long)]
    output_last: bool,
}

struct InputData {
    simple_input_data: SimpleInputData,
    outputs_per_second: u16,
    output_last: bool,
}

const TAU: f64 = 0.5;
const BETA: f64 = 1.0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

fn did_particle_go_outside(particle: &Particle, config: &SimpleInputData) -> Vec<Wall> {
    let mut collisions = Vec::with_capacity(2);
    let radius = particle.radius;

    if particle.position.x - radius < 0.0 {
        collisions.push(Wall::Left)
    } else if particle.position.x + radius > config.room_side {
        collisions.push(Wall::Right)
    }

    if particle.position.y - radius < 0.0 {
        collisions.push(Wall::Bottom)
    } else if particle.position.y + radius > config.room_side {
        collisions.push(Wall::Top)
    }

    collisions
}

fn find_target_direction<R: Rng>(
    position: Vector2<f64>,
    target_y: f64,
    left: f64,
    right: f64,
    rng: &mut R,
) -> Vector2<f64> {
    let target_x = if (left..right).contains(&position.x) {
        position.x
    } else {
        rng.sample(Uniform::new_inclusive(left, right))
    };
    Vector2::new(target_x, target_y) - position
}

fn run<W: Write, W2: Write, F: FnMut(&BTreeMap<ID, Particle>, f64) -> bool>(
    config: InputData,
    mut output_particles: W,
    mut output_exit_times: W2,
    mut stop_condition: F,
) {
    let input_data = config.simple_input_data;
    let output_dt = 1.0 / config.outputs_per_second as f64;
    let mut next_output_time = 0.0;
    let dt = input_data.min_radius / (2.0 * input_data.max_speed);
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = input_data
        .particles
        .iter()
        .cloned()
        .map(|p| (p.id, p))
        .collect();
    let mut rng = if let Some(seed) = input_data.rng_seed {
        StdRng::seed_from_u64(seed)
    } else {
        StdRng::from_entropy()
    };

    struct IterationParticleData {
        velocity: Vector2<f64>,
        in_contact: bool,
        to_delete: bool,
    }

    let mut iteration_particle_data: HashMap<ID, IterationParticleData> = HashMap::new();

    while !stop_condition(&state, time) {
        if time >= next_output_time {
            next_output_time += output_dt;
            // Write to output
            IterableFrame {
                time,
                particles: state.values(),
            }
            .write_to(&mut output_particles)
            .unwrap();
        }

        iteration_particle_data.clear();
        iteration_particle_data.extend(state.iter().map(|(&id, _)| {
            (
                id,
                IterationParticleData {
                    velocity: Vector2::zeros(),
                    in_contact: false,
                    to_delete: false,
                },
            )
        }));

        for (p1, p2) in state.values().tuple_combinations() {
            let delta = p2.position - p1.position;
            if delta.magnitude_squared() < (p1.radius + p2.radius).powi(2) {
                let v = -delta.normalize();
                let p1d = iteration_particle_data.get_mut(&p1.id).unwrap();
                p1d.velocity += v;
                p1d.in_contact = true;
                let p2d = iteration_particle_data.get_mut(&p2.id).unwrap();
                p2d.velocity -= v;
                p2d.in_contact = true;
            }
        }

        let exit_left = (input_data.room_side - input_data.exit_size) / 2.0;
        let exit_right = (input_data.room_side + input_data.exit_size) / 2.0;
        for (id, particle) in &state {
            for closest_point in [
                Vector2::new(particle.position.x, input_data.room_side),
                Vector2::new(input_data.room_side, particle.position.y),
                Vector2::new(0.0, particle.position.y),
                Vector2::new(particle.position.x.min(exit_left), 0.0),
                Vector2::new(particle.position.x.max(exit_right), 0.0),
            ] {
                let delta = particle.position - closest_point;
                if delta.magnitude_squared() < particle.radius.powi(2) {
                    let pd = iteration_particle_data.get_mut(id).unwrap();
                    pd.in_contact = true;
                    pd.velocity += delta.normalize();
                }
            }
        }

        let mut reached_count = 0;

        for (id, particle) in state.iter_mut() {
            let particle_data = iteration_particle_data.get_mut(id).unwrap();
            if particle_data.in_contact {
                particle.radius = input_data.min_radius;
                particle_data.velocity = particle_data.velocity.normalize() * input_data.max_speed;
            } else {
                if particle.radius < input_data.max_radius {
                    particle.radius += input_data.max_radius / (TAU / dt);
                }
                let (target_y, left, right) = match particle.target {
                    ParticleTarget::Exit => {
                        let left_exit_target =
                            (input_data.room_side - (input_data.exit_size - 0.2)) / 2.0;
                        let right_exit_target =
                            (input_data.room_side + (input_data.exit_size - 0.2)) / 2.0;
                        (0.0, left_exit_target, right_exit_target)
                    }
                    ParticleTarget::FarExit => {
                        let left_exit_target =
                            (input_data.room_side - input_data.far_exit_size) / 2.0;
                        let right_exit_target =
                            (input_data.room_side + input_data.far_exit_size) / 2.0;
                        (
                            -input_data.far_exit_distance,
                            left_exit_target,
                            right_exit_target,
                        )
                    }
                };
                let delta =
                    find_target_direction(particle.position, target_y, left, right, &mut rng);

                if particle.position.y < 0.0 && particle.target == ParticleTarget::Exit {
                    particle.target = ParticleTarget::FarExit;
                    reached_count += 1;
                } else if particle.position.y < -input_data.far_exit_distance
                    && particle.target == ParticleTarget::FarExit
                {
                    particle_data.to_delete = true;
                } else {
                    particle_data.velocity = input_data.max_speed
                        * ((particle.radius - input_data.min_radius)
                            / (input_data.max_radius - input_data.min_radius))
                            .pow(BETA)
                        * delta.normalize();
                }
            }

            particle.position += particle_data.velocity * dt;
        }

        state.drain_filter(|id, _| iteration_particle_data[id].to_delete);

        time += dt;

        for _ in 0..reached_count {
            output_exit_times
                .write_fmt(format_args!("{}\n", time))
                .unwrap();
        }
    }

    // Write last frame in case it wasnt
    if config.output_last {
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
        outputs_per_second: args.outputs_per_second,
        output_last: args.output_last,
    };

    run(
        input,
        File::create(args.output_particles).unwrap(),
        File::create(args.output_exit_times).unwrap(),
        |_state, _t| false,
    );
}

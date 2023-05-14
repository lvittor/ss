#![feature(let_chains)]

use chumsky::Parser;
use cim::{
    /*cim_finder::CimNeighborFinder, */ neighbor_finder::NeighborFinder, particles::ID,
    simple_finder::SimpleNeighborFinder,
};
use tp4b::gear::GearPredictor;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, File},
    io::{stdout, Write},
};

use itertools::Itertools;
use nalgebra::Vector2;
use pool::{
    models::{Ball, Frame, InputData as SimpleInputData},
    parser::input_parser,
    Float, HOLE_POSITIONS,
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
    delta_time_n: u16,

    #[arg(long)]
    output_every: u64,

    #[arg(short, long)]
    with_holes: bool,

    #[arg(short, long)]
    max_duration: Option<f64>,
}

struct InputData {
    simple_input_data: SimpleInputData,
    output_every: u64,
    delta_time_n: u16,
    with_holes: bool,
}

#[derive(Debug, Copy, Clone)]
enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

const K: Float = 10e4 * 100.0;

fn did_ball_go_outside(ball: &Ball, config: &InputData) -> Vec<Wall> {
    let mut collisions = Vec::with_capacity(2);
    let radius = config.simple_input_data.ball_radius;

    if ball.position.x - radius < 0.0 {
        collisions.push(Wall::Left)
    } else if ball.position.x + radius > config.simple_input_data.table_width {
        collisions.push(Wall::Right)
    }

    if ball.position.y - radius < 0.0 {
        collisions.push(Wall::Bottom)
    } else if ball.position.y + radius > config.simple_input_data.table_height {
        collisions.push(Wall::Top)
    }

    collisions
}

fn calculate_force(b: &Ball, other: &Ball, radius_sum: Float) -> Vector2<Float> {
    let r_hat = (other.position - b.position).normalize();
    K * ((b.position - other.position).magnitude() - radius_sum) * r_hat
}

/*
fn find_earliest_collision(
    state: &[&Ball],
    holes: &[Vector2<Float>],
    config: &InputData,
) -> Option<Collision> {
    let mut earliest: Option<Collision> = None;
    for (ball_1, ball_2) in state.iter().tuple_combinations() {
        if let Some(time) = find_collision_between_balls(ball_1, ball_2, config.ball_radius * 2.0)
            && earliest.map(|e| time < e.time).unwrap_or(true)
        {
            earliest = Some(Collision {
                time,
                info: CollisionAgainst::Ball(ball_1.id, ball_2.id)
            });
        }
    }

    for (ball, hole) in state.iter().cartesian_product(holes.iter()) {
        if let Some(time) = find_collision_between_balls(ball, &Ball {
            id: 0,
            position: *hole,
            velocity: Vector2::zeros()
        }, config.ball_radius + config.hole_radius)
            && earliest.map(|e| time < e.time).unwrap_or(true)
        {
            earliest = Some(Collision {
                time,
                info: CollisionAgainst::Hole(ball.id)
            });
        }
    }

    for ball in state.iter() {
        if let Some((time, wall_type)) = find_collision_against_wall(ball, config)
            && earliest.map(|e| time < e.time).unwrap_or(true)
        {
            earliest = Some(Collision {
                time,
                info: CollisionAgainst::Wall(ball.id, wall_type)
            });
        }
    }

    earliest
}
*/

/*
fn apply_collision(state: &mut BTreeMap<ID, Ball>, config: &InputData, collision: Collision) {
    match collision.info {
        CollisionAgainst::Ball(id1, id2) => {
            let delta_v = state[&id2].velocity - state[&id1].velocity;
            let delta_r = state[&id2].position - state[&id1].position;
            let sigma = config.ball_radius * 2.0;

            let j = (2.0 * config.ball_mass.powi(2) * (delta_v.dot(&delta_r)))
                / (sigma * (config.ball_mass * 2.0));
            let j_vec = delta_r * j / sigma;

            let ball_1 = state.get_mut(&id1).unwrap();
            ball_1.velocity += j_vec / config.ball_mass;
            let ball_2 = state.get_mut(&id2).unwrap();
            ball_2.velocity -= j_vec / config.ball_mass;
        }
        CollisionAgainst::Wall(id, wall_type) => match wall_type {
            WallType::Horizontal => state.get_mut(&id).unwrap().velocity.y *= -1.0,
            WallType::Vertical => state.get_mut(&id).unwrap().velocity.x *= -1.0,
        },
        CollisionAgainst::Hole(id) => {
            state.remove(&id);
        }
    }
}
*/

fn run<W: Write, F: FnMut(&BTreeMap<ID, (Ball, [Vector2<f64>; 4])>, Float) -> bool>(
    config: InputData,
    mut output_writer: W,
    mut stop_condition: F,
) {
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = config
        .simple_input_data
        .balls
        .iter()
        .copied()
        .map(|p| (p.id, (p, [Vector2::zeros(); 4])))
        .collect();

    if config.with_holes {
        let holes = HOLE_POSITIONS.map(|v| {
            v.component_mul(&Vector2::new(
                config.simple_input_data.table_width,
                config.simple_input_data.table_height,
            ))
        });
    }

    {
        // Write to output
        let frame = Frame {
            time,
            balls: state.values().map(|(b, _)| b).copied().collect_vec(),
        };
        output_writer.write_fmt(format_args!("{frame}")).unwrap();
    }

    let delta_time = (10.0 as Float).powi(-(config.delta_time_n as i32));
    let mut iteration = 0;

    while !stop_condition(&state, time) {
        let radius_sum = config.simple_input_data.ball_radius * 2.0;

        let predictions: BTreeMap<_, _> = state
            .iter()
            .map(|(&id, (b, [r2, r3, r4, r5]))| {
                (
                    id,
                    GearPredictor::from_ball(&b, *r2, *r3, *r4, *r5).predict(delta_time),
                )
            })
            .collect();

        let neighbors = SimpleNeighborFinder::find_neighbors(
            &predictions
                .iter()
                .map(|(&id, pred)| Ball {
                    id,
                    radius: state[&id].0.radius,
                    position: pred.predictions[0],
                    velocity: pred.predictions[1],
                })
                .collect_vec(),
            cim::simple_finder::SystemInfo {
                cyclic: false,
                interaction_radius: 0.0,
                space_width: config.simple_input_data.table_width,
                space_height: config.simple_input_data.table_height,
            },
        );

        /*
         * It's slower for normal ball count but faster for more balls
        let neighbors = CimNeighborFinder::find_neighbors(
            &state.values().cloned().collect_vec(),
            cim::cim_finder::SystemInfo {
                cyclic: false,
                interaction_radius: 0.0,
                space_width: config.simple_input_data.table_width,
                space_height: config.simple_input_data.table_height,
                columns: (config.simple_input_data.table_width / radius_sum).floor() as usize,
                rows: (config.simple_input_data.table_height / radius_sum).floor() as usize,
            },
        );
        */

        let mut forces: HashMap<_, _> = state.iter().map(|(&k, _)| (k, Vector2::zeros())).collect();

        for (&id, (ball, _)) in &state {
            let neighs = neighbors.get_neighbors(id);

            for other in neighs.map(|id| state[id].0) {
                if id > other.id {
                    let force = calculate_force(ball, &other, radius_sum);
                    *forces.get_mut(&ball.id).unwrap() += force;
                    *forces.get_mut(&other.id).unwrap() -= force;
                }
            }

            let walls = did_ball_go_outside(ball, &config);
            for wall in walls {
                match wall {
                    Wall::Left => {
                        let depth = -(ball.position.x - ball.radius);
                        forces.get_mut(&ball.id).unwrap().x += K * depth;
                    }
                    Wall::Right => {
                        let depth =
                            ball.position.x - config.simple_input_data.table_width + ball.radius;
                        forces.get_mut(&ball.id).unwrap().x -= K * depth;
                    }
                    Wall::Bottom => {
                        let depth = -(ball.position.y - ball.radius);
                        forces.get_mut(&ball.id).unwrap().y += K * depth;
                    }
                    Wall::Top => {
                        let depth =
                            ball.position.y - config.simple_input_data.table_height + ball.radius;
                        forces.get_mut(&ball.id).unwrap().y -= K * depth;
                    }
                }
            }
        }

        for (id, (ball, higher_order)) in state.iter_mut() {
            let force = forces.get(id).cloned().unwrap_or_else(Vector2::zeros);
            let acceleration = force / config.simple_input_data.ball_mass;
            let [p, v, r2, r3, r4, r5] = predictions[&id].correct(acceleration, delta_time);
            ball.position = p;
            ball.velocity = v;
            *higher_order = [r2, r3, r4, r5];
        }

        time += delta_time;
        iteration += 1;

        if iteration % config.output_every == 0 {
            // Write to output
            let frame = Frame {
                time,
                balls: state.values().map(|(b, _)| b).copied().collect_vec(),
            };
            output_writer.write_fmt(format_args!("{frame}")).unwrap();
        }
    }
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input).unwrap();
    let input = InputData {
        simple_input_data: input_parser()
            .parse(&input)
            .into_result()
            .expect("Error parsing input data."),
        delta_time_n: args.delta_time_n,
        with_holes: args.with_holes,
        output_every: args.output_every,
    };

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

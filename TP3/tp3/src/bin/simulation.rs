#![feature(let_chains)]

use chumsky::Parser;
use cim::particles::ID;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{stdout, Write},
};

use itertools::Itertools;
use nalgebra::Vector2;
use tp3::{
    parser::input_parser,
    particle::{Ball, Frame, InputData},
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
    max_duration: Option<Float>,
}

#[derive(Debug, Copy, Clone)]
struct Collision {
    time: Float,
    info: CollisionAgainst,
}

#[derive(Debug, Copy, Clone)]
enum WallType {
    Horizontal,
    Vertical,
}

#[derive(Debug, Copy, Clone)]
enum CollisionAgainst {
    Ball(ID, ID),
    Wall(ID, WallType),
    Hole(ID),
}

fn find_collision_between_balls(b1: &Ball, b2: &Ball, radius_sum: Float) -> Option<Float> {
    let delta_v = b2.velocity - b1.velocity;
    let delta_r = b2.position - b1.position;
    let sigma = radius_sum;
    let d = (delta_v.dot(&delta_r).powi(2))
        - delta_v.dot(&delta_v) * (delta_r.dot(&delta_r) - sigma.powi(2));

    (delta_v.dot(&delta_r) < 0.0 && d >= 0.0)
        .then(|| -(delta_v.dot(&delta_r) + d.sqrt()) / (delta_v.dot(&delta_v)))
}

fn find_collision_against_wall(ball: &Ball, config: &InputData) -> Option<(Float, WallType)> {
    let radius = config.ball_radius;

    let time_x = if ball.velocity.x > 0.0 {
        Some((config.table_width - radius - ball.position.x) / ball.velocity.x)
    } else if ball.velocity.x < 0.0 {
        Some((radius - ball.position.x) / ball.velocity.x)
    } else {
        None
    }
    .map(|t| (t, WallType::Vertical));

    let time_y = if ball.velocity.y > 0.0 {
        Some((config.table_height - radius - ball.position.y) / ball.velocity.y)
    } else if ball.velocity.y < 0.0 {
        Some((radius - ball.position.y) / ball.velocity.y)
    } else {
        None
    }
    .map(|t| (t, WallType::Horizontal));

    time_x
        .into_iter()
        .chain(time_y.into_iter())
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
}

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

fn run<W: Write, F: FnMut(&BTreeMap<ID, Ball>, Float) -> bool>(
    config: InputData,
    mut output_writer: W,
    mut stop_condition: F,
) {
    let mut time = 0.0;
    let mut state: BTreeMap<_, _> = config.balls.iter().copied().map(|p| (p.id, p)).collect();

    let holes = HOLE_POSITIONS
        .map(|v| v.component_mul(&Vector2::new(config.table_width, config.table_height)));

    {
        // Write to output
        let frame = Frame {
            time,
            balls: state.values().copied().collect_vec(),
        };
        output_writer.write_fmt(format_args!("{frame}")).unwrap();
    }

    while let Some(collision) = find_earliest_collision(&state.values().collect_vec(), &holes, &config) && !stop_condition(&state, time) {
        // Forward until earliest collision
        for ball in state.values_mut() {
            ball.position += ball.velocity * collision.time;
        }

        time += collision.time;

        apply_collision(&mut state, &config, collision);

        // Write to output
        let frame = Frame {
            time,
            balls: state.values().copied().collect_vec(),
        };
        output_writer.write_fmt(format_args!("{frame}")).unwrap();
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

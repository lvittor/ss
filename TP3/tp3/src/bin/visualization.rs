#![feature(let_chains)]

use capturable_visualization::VisualizationBuilder;
use chumsky::Parser;
use clap::Parser as _parser;
use itertools::Either;
use nalgebra::Vector2;
use nannou::{
    color::{rgb_u32, Saturate, Shade},
    prelude::{Rgb, *},
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
};
use tp3::{
    parser::{input_parser, output_parser},
    particle::{Ball, Frame, InputData},
    Float, HOLE_POSITIONS,
};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long)]
    capture_directory: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let capture_directory = args.capture_directory.clone();
    let mut visualization = VisualizationBuilder::new(|app| model(app, args))
        .update(update)
        .draw(draw)
        .with_aspect_ratio(2.0);

    if let Some(capture_directory) = capture_directory {
        visualization = visualization.with_capture(capture_directory.to_owned(), (2160, 1080));
    }

    visualization.run();
}

struct Model {
    system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Frame,
    last_frame: Option<Frame>,
    holes: Vec<Vector2<Float>>,
    time: Float,
}

fn model(_app: &App, args: Args) -> Model {
    let input = read_to_string(args.input).unwrap();
    let output_file = File::open(args.output).unwrap();
    let system_info = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let frame_iter = Box::new(output_parser(BufReader::new(output_file).lines()));

    let holes = Vec::from(HOLE_POSITIONS.map(|v| {
        v.component_mul(&Vector2::new(
            system_info.table_width,
            system_info.table_height,
        ))
    }));

    Model {
        last_frame: None,
        frame: Frame {
            time: 0.0,
            balls: system_info.balls.clone(),
        },
        time: 0.0,
        frame_iter,
        holes,
        system_info,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //model.time += update.since_last.as_secs_f64();
    model.time += 0.016666;

    while model.time >= model.frame.time {
        model.last_frame = Some(model.frame.clone());
        model.frame = model.frame_iter.next().unwrap_or_else(|| Frame {
            time: Float::INFINITY,
            balls: model.last_frame.as_ref().unwrap().balls.clone(),
        });
    }
}

fn draw(_app: &App, model: &Model, draw: &Draw) {
    let draw = draw.scale(1.0 / model.system_info.table_height as f32);
    draw.background().color(parse_hex_color("305A4A").unwrap());
    let interpolated_balls = if let Some(last_frame) = &model.last_frame {
        Either::Left(last_frame.balls.iter().map(
            |&Ball {
                 id,
                 position,
                 velocity,
             }| Ball {
                id,
                position: position + velocity * (model.time - last_frame.time),
                velocity,
            },
        ))
    } else {
        Either::Right(model.frame.balls.iter().cloned())
    };

    for particle in interpolated_balls {
        let circle_border = draw
            .ellipse()
            .radius(model.system_info.ball_radius as f32)
            .x(particle.position.x as f32)
            .y(particle.position.y as f32);
        let circle = draw
            .ellipse()
            .radius(model.system_info.ball_radius as f32 - 0.5)
            .x(particle.position.x as f32)
            .y(particle.position.y as f32);

        if particle.id == 0 {
            circle_border.color(WHITE).finish();
            circle.color(WHITE).finish();
        } else {
            let base = hsv((particle.id as f32 - 1.0) / 15.0, 1.0, 1.0).desaturate(0.1);
            circle_border.color(base.darken(0.5)).finish();
            circle.color(base).finish();
        }
    }

    for hole in &model.holes {
        draw.ellipse()
            .radius(model.system_info.hole_radius as f32)
            .x(hole.x as f32)
            .y(hole.y as f32)
            //.no_fill()
            //.stroke_weight(1.0)
            //.stroke(GRAY)
            .color(parse_hex_color("182d25").unwrap())
            .finish();
    }
}

fn parse_hex_color(s: &str) -> Result<Rgb<u8>, ParseIntError> {
    u32::from_str_radix(s, 16).map(rgb_u32)
}

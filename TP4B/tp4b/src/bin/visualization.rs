#![feature(let_chains)]

use capturable_visualization::VisualizationBuilder;
use chumsky::Parser;
use clap::Parser as _parser;
use nalgebra::Vector2;
use nannou::prelude::*;
use pool::{
    draw::draw as draw_pool,
    models::{Frame, InputData},
    parser::{input_parser, output_parser},
    Float, HOLE_POSITIONS,
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    path::PathBuf,
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
        visualization = visualization.with_capture(capture_directory, (2160, 1080));
    }

    visualization.run();
}

struct Model {
    system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Frame,
    holes: Vec<Vector2<Float>>,
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
        frame: Frame {
            time: 0.0,
            balls: system_info.balls.clone(),
        },
        frame_iter,
        holes,
        system_info,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(frame) = model.frame_iter.next() {
        model.frame = frame;
    }
}

fn draw(_app: &App, model: &Model, draw: &Draw) {
    draw_pool(&model.system_info, model.frame.balls.iter().cloned(), &model.holes, draw);
}

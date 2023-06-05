use capturable_visualization::VisualizationBuilder;
use chumsky::Parser;
use clap::Parser as _parser;
use nalgebra::{Rotation2, Vector2};
use nannou::{
    color::rgb_u32,
    prelude::{Rgb, *},
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
};
use tp5::{
    parser::{input_parser, output_parser},
    particle::{Frame, InputData},
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
        .draw(draw);

    if let Some(capture_directory) = capture_directory {
        visualization = visualization.with_capture(capture_directory, (1080, 1080));
    }

    visualization.run();
}

struct Model {
    system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Frame,
}

fn model(_app: &App, args: Args) -> Model {
    let input = read_to_string(args.input).unwrap();
    let output_file = File::open(args.output).unwrap();
    let system_info = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let frame_iter = Box::new(output_parser(BufReader::new(output_file).lines()));

    Model {
        frame: Frame {
            time: -1.0,
            particles: vec![],
        },
        frame_iter,
        system_info,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(frame) = model.frame_iter.next() {
        model.frame = frame;

        //let colors = ["d03e2d", "e97c54", "ee9262", "e6bca5", "f4e0d8"]
        //.map(parse_hex_color)
        //.map(|c| Rgba::<f32>::from(c.unwrap().into_format()))
        //.map(|mut c| {
        //c.alpha = 0.3;
        //c.into_linear()
        //});
        //let gradient = Gradient::new(colors);
    }
}

fn parse_hex_color(s: &str) -> Result<Rgb<u8>, ParseIntError> {
    u32::from_str_radix(s, 16).map(rgb_u32)
}

fn draw(_app: &App, model: &Model, draw: &Draw) {
    let draw = draw
        .scale(1.0 / (model.system_info.room_side + model.system_info.far_exit_distance) as f32)
        .y(model.system_info.far_exit_distance as f32)
        .x((model.system_info.far_exit_distance / 2.0) as f32);
    draw.background().color(parse_hex_color("213437").unwrap());

    let room_side = model.system_info.room_side as f32;
    let half_room = room_side / 2.0;
    let half_exit = (model.system_info.exit_size / 2.0) as f32;

    let points = [
        pt2(half_room - half_exit, 0.0),
        pt2(0.0, 0.0),
        pt2(0.0, room_side),
        pt2(room_side, room_side),
        pt2(room_side, 0.0),
        pt2(half_room + half_exit, 0.0),
    ];

    draw.polyline().weight(0.1).points(points).color(WHITE);

    for (_i, particle) in model.frame.particles.iter().enumerate() {
        draw.ellipse()
            .radius(particle.radius as f32)
            .x(particle.position.x as f32)
            .y(particle.position.y as f32)
            //.color(WHITE);
            //.color(gradient.get(angle.rem_euclid(TAU_F64) as f32 / TAU));
            .color(hsva((particle.radius % 1.0) as f32, 1.0, 1.0, 0.4));
    }
}

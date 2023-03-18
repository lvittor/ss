#![feature(is_some_and)]

use chumsky::Parser;
use clap::Parser as _parser;
use frame_capturer::FrameCapturer;
use nalgebra::{Rotation2, Vector2};
use nannou::{
    color::rgb_u32,
    prelude::{Rgb, *},
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::Path,
};
use tp2::{
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
}

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    _system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Frame,
    space_to_texture: Mat4,
    frame_capturer: FrameCapturer,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let texture_size = [1000, 1000];

    let args = Args::parse();
    let input = read_to_string(args.input).unwrap();
    let output_file = File::open(args.output).unwrap();
    let system_info = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let frame_iter = Box::new(output_parser(
        system_info.particles.len(),
        BufReader::new(output_file).lines(),
    ));

    let space_to_texture = Mat4::from_scale(vec3(1.0, -1.0, 1.0))
        * Mat4::from_translation(vec3(
            -(texture_size[0] as f32) / 2.0,
            -(texture_size[1] as f32) / 2.0,
            0.0,
        ))
        * Mat4::from_scale({
            let scale = texture_size[0] as f32 / system_info.space_length as f32;
            Vec3::new(scale, scale, scale)
        });

    let window = app.new_window().view(view).build().unwrap();
    Model {
        _window: window,
        frame: Frame {
            time: -1.0,
            particles: vec![],
        },
        frame_iter,
        _system_info: system_info,
        space_to_texture,
        frame_capturer: FrameCapturer::new(
            &app.window(window).unwrap(),
            texture_size,
            frame_capturer::CaptureMode::Capture {
                directory: Path::new("./captures").to_owned(),
            },
        ),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(frame) = model.frame_iter.next() {
        model.frame = frame;

        let draw = model.frame_capturer.get_draw();
        draw.reset();

        let colors = ["d03e2d", "e97c54", "ee9262", "e6bca5", "f4e0d8"]
            .map(parse_hex_color)
            .map(|c| Rgba::<f32>::from(c.unwrap().into_format()))
            .map(|mut c| {
                c.alpha = 0.3;
                c
            });
        let draw = &draw.transform(model.space_to_texture);
        draw.background().color(parse_hex_color("213437").unwrap());
        for (i, particle) in model.frame.particles.iter().enumerate() {
            let angle =
                Rotation2::rotation_between(&Vector2::x(), &particle.velocity_direction).angle();
            draw.polygon()
                .points([vec2(-0.1, -0.1), vec2(-0.1, 0.1), vec2(0.15, 0.0)])
                .x(particle.position.x as f32)
                .y(particle.position.y as f32)
                .rotate(angle as f32)
                .color(colors[i % colors.len()]);
        }

        model.frame_capturer.render_to_texture(&app.main_window());
    }
}

fn parse_hex_color(s: &str) -> Result<Rgb<u8>, ParseIntError> {
    u32::from_str_radix(s, 16).map(rgb_u32)
}

fn view(_app: &App, model: &Model, frame: nannou::Frame) {
    model.frame_capturer.draw_to_frame(frame);
}

fn exit(app: &App, model: Model) {
    model
        .frame_capturer
        .wait_for_image_writing(&app.main_window());
}

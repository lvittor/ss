#![feature(let_chains)]

use chumsky::Parser;
use clap::Parser as _parser;
use frame_capturer::{CaptureMode, FrameCapturer};
use itertools::Either;
use nalgebra::Vector2;
use nannou::{
    color::{rgb_u32, Shade, Saturate},
    prelude::{Rgb, *},
    wgpu::ToTextureView,
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf, f64::INFINITY,
};
use tp3::{
    parser::{input_parser, output_parser},
    particle::{Ball, Frame, InputData},
    HOLE_POSITIONS,
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
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Option<Frame>,
    last_frame: Option<Frame>,
    holes: Vec<Vector2<f64>>,
    space_to_texture: Mat4,
    frame_capturer: FrameCapturer,
    window: window::Id,
    texture_copy: wgpu::Texture,
    texture_copy_view: wgpu::TextureView,
    time: f64,
}

fn model(app: &App) -> Model {
    let texture_size = [2160, 1080];

    let args = Args::parse();
    let input = read_to_string(args.input).unwrap();
    let output_file = File::open(args.output).unwrap();
    let system_info = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let frame_iter = Box::new(output_parser(BufReader::new(output_file).lines()));

    let space_to_texture = Mat4::from_scale(vec3(1.0, -1.0, 1.0))
        * Mat4::from_translation(vec3(
            -(texture_size[0] as f32) / 2.0,
            -(texture_size[1] as f32) / 2.0,
            0.0,
        ))
        * Mat4::from_scale({
            let scale = texture_size[1] as f32 / system_info.table_height as f32;
            Vec3::new(scale, scale, scale)
        });

    let window = app.new_window().view(view).build().unwrap();

    let texture_copy = nannou::wgpu::TextureBuilder::new()
        .size(texture_size)
        .format(nannou::Frame::TEXTURE_FORMAT)
        .sample_count(1)
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.window(window).unwrap().device());

    let texture_copy_view = texture_copy.to_texture_view();

    let holes = Vec::from(HOLE_POSITIONS.map(|v| {
        v.component_mul(&Vector2::new(
            system_info.table_width,
            system_info.table_height,
        ))
    }));

    Model {
        window,
        last_frame: None,
        frame: Some(Frame {
            time: 0.0,
            balls: system_info.balls.clone()
        }),
        time: 0.0,
        frame_iter,
        holes,
        system_info,
        space_to_texture,
        frame_capturer: FrameCapturer::new(
            &app.window(window).unwrap(),
            texture_size,
            match args.capture_directory {
                Some(directory) => CaptureMode::Capture { directory },
                None => CaptureMode::NoCapture,
            },
        ),
        texture_copy,
        texture_copy_view,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if let Some(frame) = &model.frame && model.time < frame.time {
        let draw = model.frame_capturer.get_draw();
        draw.reset();

        let draw = &draw.transform(model.space_to_texture);
        draw.background().color(parse_hex_color("305A4A").unwrap());
        let interpolated_balls = if let Some(last_frame) = &model.last_frame {
            Either::Left(last_frame.balls.iter().map(|&Ball{id, position, velocity}| Ball{
                id, 
                position: position + velocity * (model.time - last_frame.time), 
                velocity
            }))
        } else {
            Either::Right(frame.balls.iter().cloned())
        };
        for particle in interpolated_balls {
            let circle = draw.ellipse()
                .radius(model.system_info.ball_radius as f32)
                .x(particle.position.x as f32)
                .y(particle.position.y as f32);

            if particle.id == 0 {
                circle
                    .color(WHITE);
            } else {
                let base = hsv((particle.id as f32 - 1.0) / 15.0, 1.0, 1.0).desaturate(0.1);
                circle
                    .color(base)
                    .stroke_color(base.darken(0.5))
                    .stroke_weight(0.5);
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

        model
            .frame_capturer
            .render_to_texture(&app.window(model.window).unwrap());

        //model.time += update.since_last.as_secs_f64();
        model.time += 0.016666;
    } else if let Some(frame) = model.frame_iter.next() {
        model.last_frame = model.frame.clone();
        model.frame = Some(frame);
    } else {
        model.last_frame = model.frame.clone();
        model.frame = Some(Frame { time: INFINITY, balls: model.last_frame.as_ref().unwrap().balls.clone() })
    }
}

fn parse_hex_color(s: &str) -> Result<Rgb<u8>, ParseIntError> {
    u32::from_str_radix(s, 16).map(rgb_u32)
}

fn view(app: &App, model: &Model, frame: nannou::Frame) {
    {
        let mut encoder = frame.command_encoder();
        model
            .frame_capturer
            .draw_to_texture(&mut encoder, &model.texture_copy_view);
    }
    let scale = {
        let [w, h] = model.texture_copy.size();
        let w = w as f32;
        let h = h as f32;
        let [win_w, win_h] = frame.texture_size();
        let win_w = win_w as f32;
        let win_h = win_h as f32;
        f32::min(win_w / w, win_h / h)
    };
    let draw = app
        .draw()
        .scale(scale / app.window(model.window).unwrap().scale_factor());
    draw.texture(&model.texture_copy);
    draw.to_frame(app, &frame).unwrap();
}

fn exit(app: &App, model: Model) {
    model
        .frame_capturer
        .wait_for_image_writing(&app.main_window());
}

use chumsky::Parser;
use clap::Parser as _parser;
use frame_capturer::{CaptureMode, FrameCapturer};
use nalgebra::{Rotation2, Vector2};
use nannou::{
    color::rgb_u32,
    prelude::{Rgb, *},
    wgpu::ToTextureView,
};
use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
};
use tp3::{
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
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    _system_info: InputData,
    frame_iter: Box<dyn Iterator<Item = Frame>>,
    frame: Frame,
    space_to_texture: Mat4,
    frame_capturer: FrameCapturer,
    window: window::Id,
    texture_copy: wgpu::Texture,
    texture_copy_view: wgpu::TextureView,
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

    Model {
        window,
        frame: Frame {
            time: -1.0,
            balls: vec![],
        },
        frame_iter,
        _system_info: system_info,
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
    if let Some(frame) = model.frame_iter.next() {
        model.frame = frame;

        let draw = model.frame_capturer.get_draw();
        draw.reset();

        //let colors = ["d03e2d", "e97c54", "ee9262", "e6bca5", "f4e0d8"]
        //.map(parse_hex_color)
        //.map(|c| Rgba::<f32>::from(c.unwrap().into_format()))
        //.map(|mut c| {
        //c.alpha = 0.3;
        //c.into_linear()
        //});
        //let gradient = Gradient::new(colors);
        let draw = &draw.transform(model.space_to_texture);
        draw.background().color(parse_hex_color("213437").unwrap());
        for (_i, particle) in model.frame.balls.iter().enumerate() {
            let angle =
                Rotation2::rotation_between(&Vector2::x(), &particle.velocity).angle();
            let tgt = particle.position + particle.velocity * 0.25;
            draw.arrow()
                .weight(0.025)
                .points(
                    vec2(particle.position.x as f32, particle.position.y as f32),
                    vec2(tgt.x as f32, tgt.y as f32),
                )
                //.color(colors[_i % colors.len()]);
                //.color(gradient.get(angle.rem_euclid(TAU_F64) as f32 / TAU));
                .color(hsva(angle.rem_euclid(TAU_F64) as f32 / TAU, 1.0, 1.0, 0.4));
        }

        model
            .frame_capturer
            .render_to_texture(&app.window(model.window).unwrap());
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

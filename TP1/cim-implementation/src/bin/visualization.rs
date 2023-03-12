#![feature(is_some_and)]

use cgmath::MetricSpace;
use chumsky::Parser;
use cim_implementation::{
    neighbor_finder::NeighborMap,
    parser::{input_parser, output_parser},
    particles::{ParticlesData, ID},
};
use nannou::{glam::Vec3Swizzles, prelude::*};
use std::fs::read_to_string;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    particles: ParticlesData,
    neighbor_map: NeighborMap<ID>,
    space_to_window: Mat4,
    selected_particle: Option<ID>,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let input = read_to_string("../data/example_input.txt").unwrap();
    let output = read_to_string("../data/output.txt").unwrap();
    let particles: ParticlesData = input_parser()
        .parse(&input)
        .into_result()
        .expect("Error parsing input data.");

    let neighbor_map: NeighborMap<ID> = output_parser()
        .parse(&output)
        .into_result()
        .expect("Error parsing output data.");

    let _window = app.new_window().view(view).event(event).build().unwrap();
    Model {
        _window,
        particles,
        neighbor_map,
        selected_particle: None,
        space_to_window: Mat4::IDENTITY,
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    if event == MousePressed(MouseButton::Left) {
        let x = app.mouse.x;
        let y = app.mouse.y;
        let pos = model
            .space_to_window
            .inverse()
            .transform_point3(vec3(x, y, 0.0))
            .xy();
        println!("{pos}");
        for particle in &model.particles.particles {
            if particle
                .position
                .distance(cgmath::vec2(pos.x, pos.y).cast().unwrap())
                < particle.radius
            {
                model.selected_particle = Some(particle.id);
                break;
            }
        }
    } else if let Resized(new_size) = event {
        let min_size = new_size.min_element();
        model.space_to_window = Mat4::from_scale(vec3(1.0, -1.0, 1.0))
            * Mat4::from_translation(vec3(-new_size.x / 2.0, -new_size.y / 2.0, 0.0))
            * Mat4::from_scale({
                let scale = min_size / model.particles.space_length as f32;
                Vec3::new(scale, scale, scale)
            });
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().transform(model.space_to_window);
    draw.background().color(BLACK);
    for particle in &model.particles.particles {
        let selected = model.selected_particle.is_some_and(|id| particle.id == id);
        let in_range = model
            .selected_particle
            .is_some_and(|id| model.neighbor_map.has_pair(id, particle.id));
        draw.ellipse()
            .x(particle.position.x as f32)
            .y(particle.position.y as f32)
            .radius(particle.radius as f32)
            .color(if selected {
                srgba(1.0, 1.0, 0.0, 0.7)
            } else if in_range {
                srgba(0.0, 0.0, 1.0, 0.7)
            } else {
                srgba(1.0, 1.0, 1.0, 0.7)
            })
            .stroke(WHITE)
            .stroke_weight(0.25);
        if selected {
            draw.ellipse()
                .x(particle.position.x as f32)
                .y(particle.position.y as f32)
                .no_fill()
                .radius((model.particles.interaction_radius + particle.radius) as f32)
                .stroke_weight(0.25)
                .stroke(RED);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

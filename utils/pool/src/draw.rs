use std::num::ParseIntError;

use nalgebra::Vector2;
use nannou::{
    color::{rgb_u32, Saturate, Shade},
    prelude::{Rgb, *},
};

use crate::{
    models::{Ball, InputData},
    Float,
};

fn parse_hex_color(s: &str) -> Result<Rgb<u8>, ParseIntError> {
    u32::from_str_radix(s, 16).map(rgb_u32)
}

pub fn draw<BI: IntoIterator<Item = Ball>>(
    system_info: &InputData,
    balls: BI,
    holes: &[Vector2<Float>],
    draw: &Draw,
) {
    let draw = draw.scale(1.0 / system_info.table_height as f32);
    draw.background().color(parse_hex_color("305A4A").unwrap());

    for particle in balls {
        let circle_border = draw
            .ellipse()
            .radius(system_info.ball_radius as f32)
            .x(particle.position.x as f32)
            .y(particle.position.y as f32);
        let circle = draw
            .ellipse()
            .radius(system_info.ball_radius as f32 - 0.5)
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

    let hole_color = parse_hex_color("182d25").unwrap();

    for hole in holes {
        draw.ellipse()
            .radius(system_info.hole_radius as f32)
            .x(hole.x as f32)
            .y(hole.y as f32)
            //.no_fill()
            //.stroke_weight(1.0)
            //.stroke(GRAY)
            .color(hole_color)
            .finish();
    }

    let pool_rect = Rect::from_corners(
        Vec2::ZERO,
        vec2(
            system_info.table_width as f32,
            system_info.table_height as f32,
        ),
    )
    .pad(-system_info.hole_radius as f32 / 2.0);

    draw.rect()
        .xy(pool_rect.xy())
        .wh(pool_rect.wh())
        .no_fill()
        .stroke_weight(system_info.hole_radius as f32)
        .stroke(hole_color);
}

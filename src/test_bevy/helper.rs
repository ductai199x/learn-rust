#![allow(unused)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;

use crate::test_bevy::*;

#[allow(non_snake_case)]
pub fn STR(s: &str) -> String {
    s.to_string()
}

pub fn mid_to_topleft_origin(
    from: &collision::Point,
    win_size: &Res<GameWindowSize>,
) -> collision::Point {
    collision::Point::new((win_size.w / 2. + from.x(), win_size.h / 2. - from.y()))
}

pub fn topleft_to_mid_origin(
    from: &collision::Point,
    win_size: &Res<GameWindowSize>,
) -> collision::Point {
    collision::Point::new((from.x() - win_size.w / 2., win_size.h / 2. - from.y()))
}

pub fn botleft_to_toplef_origin(
    from: &collision::Point,
    win_size: &Res<GameWindowSize>,
) -> collision::Point {
    collision::Point::new((from.x(), win_size.h - from.y()))
}

pub fn set_bevy_color_rgba(color: &mut Color, r: f32, g: f32, b: f32, a: f32) {
    color.set_r(r);
    color.set_g(g);
    color.set_b(b);
    color.set_a(a);
}

pub fn get_sprite_rect(size: &Vec2, pos: &Vec2) -> collision::Rect {
    let upper_left = collision::Point::new((pos.x - size.x / 2., pos.y + size.y / 2.));
    collision::Rect::new((upper_left, size.x, size.y))
}

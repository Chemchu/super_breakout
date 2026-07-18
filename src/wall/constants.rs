use bevy::{color::Color, math::primitives::Rectangle};

pub const WALL_HEIGHT: f32 = 15.;
pub const WALL_WIDTH: f32 = 150.;
pub const WALL_SHAPE: Rectangle = Rectangle::new(WALL_WIDTH, WALL_HEIGHT);
pub const WALL_COLOR: Color = Color::srgb(0., 1., 0.);

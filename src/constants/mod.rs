use bevy::{
    color::Color,
    math::primitives::{Circle, Rectangle},
};

pub const PADDLE_Y_POS: f32 = -300.;
pub const PADDLE_WIDTH: f32 = 150.;
pub const PADDLE_HEIGHT: f32 = 20.;

pub const BALL_RADIUS: f32 = 15.;
pub const BALL_SHAPE: Circle = Circle::new(BALL_RADIUS);
pub const BALL_COLOR: Color = Color::srgb(0., 0., 1.);
pub const BALL_SPEED: f32 = 15_000_000.;
pub const BALL_MAX_SPEED: f32 = BALL_SPEED * 5.;

pub const WALL_HEIGHT: f32 = 15.;
pub const WALL_WIDTH: f32 = 150.;
pub const WALL_SHAPE: Rectangle = Rectangle::new(WALL_WIDTH, WALL_HEIGHT);
pub const WALL_COLOR: Color = Color::srgb(0., 1., 0.);

pub const BOUNCE_MAX_ANGLE: f32 = 35.; // In degrees
pub const BOUNCE_MIN_ANGLE: f32 = 0.; // In degrees

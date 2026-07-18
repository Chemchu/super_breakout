use bevy::{color::Color, math::primitives::Circle};

pub const BALL_POOL_MAX_CAPACITY: u16 = 300;
pub const BALL_RADIUS: f32 = 15.;
pub const BALL_SHAPE: Circle = Circle::new(BALL_RADIUS);
pub const BALL_COLOR: Color = Color::srgb(0., 0., 1.);
pub const BALL_SPEED: f32 = 15_000_000.;
pub const BALL_MAX_SPEED: f32 = BALL_SPEED * 5.;
pub const BALL_DEFAULT_DAMAGE: f32 = 1.;
pub const BALL_DEFAULT_BOUNCES: u8 = 5;

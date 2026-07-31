use bevy::{color::Color, math::primitives::Circle};

pub const BALL_MASS: f32 = 1.0_f32;
pub const BALL_ONE_UNIT: u16 = 1;
pub const BALL_POOL_MAX_CAPACITY: u16 = 50;
pub const BALL_RADIUS: f32 = 15.;
pub const BALL_SHAPE: Circle = Circle::new(BALL_RADIUS);
pub const BALL_COLOR: Color = Color::srgb(0., 0., 1.);
pub const BALL_IMPULSE: f32 = 20_000.;
pub const BALL_MAX_SPEED: f32 = 500.0;
pub const FAN_ANGLE_RAD: f32 = 10.0_f32.to_radians();

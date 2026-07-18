use bevy::{ecs::component::Component, math::Vec2};
use bevy_enhanced_input::prelude::InputAction;

#[derive(Component, Default, Clone)]
pub struct Paddle;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct PaddleHorizontalMovement;

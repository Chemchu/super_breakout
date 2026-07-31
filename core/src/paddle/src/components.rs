use bevy::{ecs::component::Component, math::Vec2};
use bevy_enhanced_input::prelude::InputAction;

use common::components::BounceDeflector;

#[derive(Component, Default, Clone)]
#[require(BounceDeflector)]
pub struct Paddle;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct PaddleHorizontalMovement;

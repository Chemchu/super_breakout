use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    math::Vec2,
};
use bevy_enhanced_input::prelude::InputAction;

use crate::common::events::Died;

#[derive(Component, Default, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Default, Clone)]
pub struct NeedsImpulse {
    pub impulse: Vec2,
}

#[derive(Component, Default, Clone)]
pub struct Bounceable;

#[derive(Component, Default, Clone)]
pub struct BounceDeflector {
    pub width: f32,
    pub max_angle: f32,
    pub dead_zone: f32,
}

#[derive(Component, Clone)]
pub struct Health {
    value: f32,
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

impl Health {
    pub fn new(health_value: f32) -> Self {
        Health {
            value: health_value,
        }
    }

    pub fn take_damage(&mut self, mut commands: Commands, damage: f32, entity: Entity) {
        self.value -= damage;

        if self.value <= 0.0 {
            commands.trigger(Died { entity });
        }
    }
}

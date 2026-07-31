use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    math::Vec2,
};
use bevy_enhanced_input::prelude::InputAction;

use crate::events::Died;

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

#[derive(Component, Default, Clone)]
pub struct Dashable;

#[derive(Component, Default, Clone)]
pub struct Rejectable;

impl Health {
    pub fn new(health_value: f32) -> Self {
        Health {
            value: health_value,
        }
    }

    pub fn take_damage(
        &mut self,
        mut commands: Commands,
        damage: f32,
        mass: f32,
        linear_velocity: f32,
        entity: Entity,
    ) {
        println!(
            "Damage {:#?}, mass {:#?}, linear_velocity {:#?}",
            damage, mass, linear_velocity
        );
        let actual_damage = damage * mass * linear_velocity;
        println!("Actual damage {:#?}", actual_damage);
        self.value -= actual_damage;

        if self.value <= 0.0 {
            commands.trigger(Died { entity });
        }
    }
}

#[derive(Component, Clone)]
pub struct LaunchPoint {
    pub surface_offset: Vec2,
}

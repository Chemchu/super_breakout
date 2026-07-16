use avian2d::prelude::PhysicsLayer;
use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub struct Wall {
    life: f32,
}

impl Default for Wall {
    fn default() -> Self {
        Wall { life: 5. }
    }
}

#[derive(Component, Default, Clone)]
pub struct NeedsImpulse;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Paddle,
    Ball,
    Wall,
}

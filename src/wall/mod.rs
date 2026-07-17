use avian2d::{collision::collision_events::CollisionStart, prelude::PhysicsLayer};
use bevy::ecs::{
    component::Component,
    entity::{ContainsEntity, Entity},
    event::{EntityEvent, Event},
    observer::On,
    query::With,
    system::{Commands, Query},
};

use crate::ball::Ball;

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Component, Clone)]
pub struct Health {
    value: f32,
}

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

#[derive(EntityEvent)]
pub struct Died {
    pub entity: Entity,
}

#[derive(Component, Default, Clone)]
pub struct Damage(pub f32);

#[derive(Component, Default, Clone)]
pub struct NeedsImpulse;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Paddle,
    Ball,
    Wall,
}

pub fn on_ball_and_wall_collision(
    event: On<CollisionStart>,
    mut wall_query: Query<&mut Health, With<Wall>>,
    ball_query: Query<&Damage, With<Ball>>,
    commands: Commands,
) {
    let wall_entity = event.collider1;
    let ball_entity = event.collider2;

    if let (Ok(mut wall_health), Ok(ball_damage)) =
        (wall_query.get_mut(wall_entity), ball_query.get(ball_entity))
    {
        wall_health.take_damage(commands, ball_damage.0, wall_entity);
    }
}

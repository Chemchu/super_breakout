use avian2d::{collision::collision_events::CollisionStart, prelude::PhysicsLayer};
use bevy::ecs::{component::Component, observer::On, query::With, system::Query};

use crate::ball::Ball;

#[derive(Component, Clone)]
pub struct Wall;

#[derive(Component, Default, Clone)]
pub struct Health(f32);

#[derive(Component, Default, Clone)]
pub struct Damage(f32);

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
    mut wall_query: Query<&Health, With<Wall>>,
    ball_query: Query<&Damage, With<Ball>>,
) {
    let wall_entity = event.collider1;
    let ball_entity = event.collider2;

    if let (Ok(wall_health), Ok(ball_damage)) =
        (wall_query.get_mut(wall_entity), ball_query.get(ball_entity))
    {}
}

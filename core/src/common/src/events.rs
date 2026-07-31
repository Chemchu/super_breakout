use bevy::ecs::{entity::Entity, event::Event, event::EntityEvent};

#[derive(EntityEvent)]
pub struct Died {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LaunchBallRequested;

#[derive(Event)]
pub struct DoubleBallRequested;

#[derive(Event)]
pub struct TripleBallRequested;

#[derive(Event)]
pub struct ReverseBallRequested;

#[derive(Event)]
pub struct SlowTimeRequested;

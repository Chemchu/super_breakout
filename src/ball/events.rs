use bevy::ecs::event::Event;

#[derive(Event)]
pub struct LaunchBallRequested;

#[derive(Event)]
pub struct DoubleBallRequested;

#[derive(Event)]
pub struct TripleBallRequested;

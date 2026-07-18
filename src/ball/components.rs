use bevy::{
    ecs::{component::Component, resource::Resource},
    math::Vec2,
};

use crate::ball::constants::BALL_POOL_MAX_CAPACITY;

#[derive(Component, Default, Clone, Debug)]
pub struct Ball;

#[derive(Resource)]
pub struct BallPool {
    pub capacity: u16,
}

impl Default for BallPool {
    fn default() -> Self {
        BallPool {
            capacity: BALL_POOL_MAX_CAPACITY,
        }
    }
}

impl BallPool {
    pub fn increase_pool_size_by_n(&mut self, increment: u16) {
        self.capacity += increment;
    }

    pub fn decrease_pool_size_by_n(&mut self, decrement: u16) {
        self.capacity -= decrement;
    }
}

#[derive(Component, Clone)]
pub struct BallLaunchPoint {
    pub surface_offset: Vec2,
}

use bevy::{
    ecs::{component::Component, resource::Resource},
    math::Vec2,
};

use crate::ball::constants::{BALL_DEFAULT_BOUNCES, BALL_DEFAULT_DAMAGE, BALL_POOL_MAX_CAPACITY};

#[derive(Component, Clone, Debug)]
pub struct Ball {
    damage: f32,
    bounces: u8,
}

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

impl Default for Ball {
    fn default() -> Self {
        Ball {
            damage: BALL_DEFAULT_DAMAGE,
            bounces: BALL_DEFAULT_BOUNCES,
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

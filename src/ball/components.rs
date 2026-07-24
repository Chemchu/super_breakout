use bevy::{
    asset::{Assets, Handle},
    ecs::{component::Component, resource::Resource, world::FromWorld},
    math::Vec2,
    mesh::Mesh,
    sprite_render::ColorMaterial,
};

use crate::ball::constants::{BALL_COLOR, BALL_POOL_MAX_CAPACITY, BALL_SHAPE};

#[derive(Component, Default, Clone, Debug)]
pub struct Ball;

#[derive(Resource, Clone)]
pub struct BallAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

impl FromWorld for BallAssets {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(BALL_SHAPE);
        let material = world
            .resource_mut::<Assets<ColorMaterial>>()
            .add(BALL_COLOR);

        BallAssets { mesh, material }
    }
}

#[derive(Resource)]
pub struct BallPool {
    max_capacity: u16,
    pub current_ball_count: u16,
}

impl Default for BallPool {
    fn default() -> Self {
        BallPool {
            max_capacity: BALL_POOL_MAX_CAPACITY,
            current_ball_count: 0,
        }
    }
}

impl BallPool {
    pub fn has_available_balls(&mut self, balls_needed: u16) -> bool {
        self.max_capacity - (self.current_ball_count + balls_needed) > 0
    }

    pub fn increase_current_ball_count<F>(&mut self, increment: u16, spawn_balls: F)
    where
        F: FnOnce(u16),
    {
        let spawned_balls = if self.current_ball_count + increment > self.max_capacity {
            self.max_capacity - self.current_ball_count
        } else {
            increment
        };

        self.current_ball_count += spawned_balls;
        spawn_balls(spawned_balls);
    }

    pub fn decrease_current_ball_count<F>(&mut self, decrement: u16, despawn_balls: F)
    where
        F: FnOnce(),
    {
        self.current_ball_count = if self.current_ball_count < decrement {
            0
        } else {
            self.current_ball_count - decrement
        };

        despawn_balls();
    }
}

#[derive(Component, Clone)]
pub struct BallLaunchPoint {
    pub surface_offset: Vec2,
}

use bevy::{
    asset::{Assets, Handle},
    ecs::{component::Component, resource::Resource, world::FromWorld},
    mesh::Mesh,
    sprite_render::ColorMaterial,
};

use crate::{
    ball::constants::{BALL_COLOR, BALL_POOL_MAX_CAPACITY, BALL_SHAPE},
    common::components::{Bounceable, Damage},
};

#[derive(Component, Default, Clone, Debug)]
#[require(Damage, Bounceable)]
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
    pub fn allocate_balls(&mut self, increment: u16) -> u16 {
        if self.current_ball_count >= self.max_capacity {
            return 0;
        }

        let allowed_space = self.max_capacity - self.current_ball_count;
        let spawned_balls = increment.min(allowed_space);

        self.current_ball_count += spawned_balls;
        spawned_balls
    }

    pub fn deallocate_balls(&mut self, decrement: u16) -> u16 {
        let actual_removed = self.current_ball_count.min(decrement);
        self.current_ball_count -= actual_removed;
        actual_removed
    }
}

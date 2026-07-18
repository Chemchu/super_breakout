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

#[derive(Resource)]
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

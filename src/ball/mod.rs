use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{Friction, GravityScale, MaxLinearSpeed, Restitution, RigidBody},
    interpolation::TransformInterpolation,
};
use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, resource::Resource},
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};
use bevy_enhanced_input::prelude::InputAction;

use crate::{
    constants::{
        BALL_DEFAULT_BOUNCES, BALL_DEFAULT_DAMAGE, BALL_MAX_SPEED, BALL_POOL_MAX_CAPACITY,
        BALL_RADIUS,
    },
    wall::{CollisionLayer, Damage, NeedsImpulse},
};

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

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

#[derive(InputAction)]
#[action_output(bool)]
pub struct BallShot;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Action1;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Action2;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Action3;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Action4;

pub fn setup_ball(
    translation: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Ball::default(),
        Damage(1.),
        NeedsImpulse,
        Transform::from_translation(translation),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Collider::circle(BALL_RADIUS),
        RigidBody::Dynamic,
        GravityScale(0.),
        TransformInterpolation,
        Restitution::new(1.0),
        Friction::new(0.),
        MaxLinearSpeed(BALL_MAX_SPEED),
        CollisionLayers::new(
            [CollisionLayer::Ball],
            [CollisionLayer::Wall, CollisionLayer::Paddle],
        ),
    )
}

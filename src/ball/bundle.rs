use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{Friction, GravityScale, MaxLinearSpeed, Restitution, RigidBody},
    interpolation::TransformInterpolation,
};
use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    math::{Vec2, Vec3},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    ball::{
        components::Ball,
        constants::{BALL_MAX_SPEED, BALL_RADIUS, BALL_SPEED},
    },
    common::{
        components::{Damage, NeedsImpulse},
        physical_layers::CollisionLayer,
    },
};

pub fn get_ball_bundle(
    translation: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Ball::default(),
        Damage(1.),
        NeedsImpulse {
            impulse: Vec2::new(0., BALL_SPEED),
        },
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

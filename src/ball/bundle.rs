use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{Friction, GravityScale, MaxLinearSpeed, Restitution, RigidBody},
    interpolation::TransformInterpolation,
};
use bevy::{
    ecs::bundle::{Bundle, NoBundleEffect},
    math::{Vec2, Vec3},
    mesh::Mesh2d,
    sprite_render::MeshMaterial2d,
    transform::components::Transform,
};

use crate::{
    ball::{
        components::{Ball, BallAssets},
        constants::{BALL_MAX_SPEED, BALL_RADIUS, BALL_SPEED},
    },
    common::{
        components::{Damage, NeedsImpulse},
        physical_layers::CollisionLayer,
    },
};

pub fn get_ball_bundle(
    translation: Vec2,
    launch_direction: Vec2,
    assets: &BallAssets,
) -> impl Bundle<Effect: NoBundleEffect> {
    (
        Ball::default(),
        Damage(1.),
        NeedsImpulse {
            impulse: launch_direction.normalize_or_zero() * BALL_SPEED,
        },
        Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
        Mesh2d(assets.mesh.clone()),
        MeshMaterial2d(assets.material.clone()),
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

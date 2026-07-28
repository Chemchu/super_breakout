use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{
        Friction, GravityScale, MaxLinearSpeed, Restitution, RigidBody,
        mass_properties::components::Mass,
    },
    interpolation::TransformInterpolation,
};
use bevy::{
    ecs::bundle::Bundle,
    math::{Vec2, Vec3},
    mesh::Mesh2d,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    components::{Ball, BallAssets},
    constants::{BALL_MAX_SPEED, BALL_RADIUS, BALL_SPEED},
};
use common::{
    components::{Bounceable, Damage, NeedsImpulse},
    physical_layers::CollisionLayer,
};

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub damage: Damage,
    pub needs_impulse: NeedsImpulse,
    pub bounceable: Bounceable,
    pub mass: Mass,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
    pub interpolation: TransformInterpolation,
    pub restitution: Restitution,
    pub friction: Friction,
    pub max_speed: MaxLinearSpeed,
    pub collision_layers: CollisionLayers,
}

pub fn get_ball_bundle(
    translation: Vec2,
    launch_direction: Vec2,
    assets: BallAssets,
) -> BallBundle {
    BallBundle {
        ball: Ball::default(),
        damage: Damage(1.5_f32),
        needs_impulse: NeedsImpulse {
            impulse: launch_direction.normalize_or_zero() * BALL_SPEED,
        },
        bounceable: Bounceable,
        mass: Mass(1.0_f32),
        transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
        mesh: Mesh2d(assets.mesh),
        material: MeshMaterial2d(assets.material),
        collider: Collider::circle(BALL_RADIUS),
        rigid_body: RigidBody::Dynamic,
        gravity_scale: GravityScale(0.),
        interpolation: TransformInterpolation,
        restitution: Restitution::new(1.0),
        friction: Friction::new(0.),
        max_speed: MaxLinearSpeed(BALL_MAX_SPEED),
        collision_layers: CollisionLayers::new(
            [CollisionLayer::Ball],
            [CollisionLayer::Wall, CollisionLayer::Paddle],
        ),
    }
}

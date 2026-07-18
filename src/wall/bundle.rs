use avian2d::{
    collision::{
        collider::{Collider, CollisionLayers},
        collision_events::CollisionEventsEnabled,
    },
    dynamics::rigid_body::{Friction, GravityScale, Restitution, RigidBody},
    interpolation::TransformInterpolation,
};
use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    math::Vec3,
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    common::{components::Health, physical_layers::CollisionLayer},
    wall::{
        components::Wall,
        constants::{WALL_HEIGHT, WALL_WIDTH},
    },
};

pub fn get_wall_bundle(
    translation: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Wall,
        Health::new(5.),
        Transform::from_translation(translation),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Collider::rectangle(WALL_WIDTH, WALL_HEIGHT),
        RigidBody::Static,
        GravityScale(0.),
        TransformInterpolation,
        Restitution::new(1.0),
        Friction::new(0.),
        CollisionLayers::new([CollisionLayer::Wall], [CollisionLayer::Ball]),
        CollisionEventsEnabled,
    )
}

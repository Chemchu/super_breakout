use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{
        Friction, GravityScale, Restitution, RigidBody,
        forces::{Forces, WriteRigidBodyForces},
    },
    interpolation::TransformInterpolation,
};
use bevy::{
    asset::{Assets, Handle},
    camera::Camera2d,
    ecs::{
        bundle::Bundle,
        entity::Entity,
        query::With,
        system::{Commands, Query, ResMut},
    },
    math::{Vec2, Vec3},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    constants::{BALL_SPEED, WALL_COLOR, WALL_HEIGHT, WALL_SHAPE, WALL_WIDTH},
    paddle::{on_ball_and_paddle_collision, setup_paddle},
    wall::{CollisionLayer, NeedsImpulse, Wall},
};

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands
        .spawn(setup_paddle())
        .observe(on_ball_and_paddle_collision);

    let wall_pos = Vec3::new(0., 400., 0.);
    let mesh = meshes.add(WALL_SHAPE);
    let material = materials.add(WALL_COLOR);

    commands.spawn(setup_wall(wall_pos, mesh.clone(), material.clone()));
}

pub fn setup_wall(
    translation: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Wall,
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
    )
}

pub fn apply_linear_impulse(
    mut query: Query<(Entity, Forces), With<NeedsImpulse>>,
    mut commands: Commands,
) {
    for (entity, mut forces) in &mut query {
        forces.apply_force(Vec2::new(0., BALL_SPEED));
        commands.entity(entity).remove::<NeedsImpulse>();
    }
}

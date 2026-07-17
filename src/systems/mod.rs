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
    constants::{
        BALL_RADIUS, BALL_SPEED, PADDLE_HEIGHT, WALL_COLOR, WALL_HEIGHT, WALL_SHAPE, WALL_WIDTH,
    },
    paddle::setup_paddle,
    wall::{CollisionLayer, NeedsImpulse, Wall},
};

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn(setup_paddle());

    let wall_pos = Vec3::new(0., (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_2 = Vec3::new(WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_3 = Vec3::new(2. * WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_4 = Vec3::new(3. * WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_5 = Vec3::new(-2. * WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_6 = Vec3::new(-3. * WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let wall_pos_7 = Vec3::new(-1. * WALL_WIDTH, (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let mesh = meshes.add(WALL_SHAPE);
    let material = materials.add(WALL_COLOR);

    commands.spawn(setup_wall(wall_pos, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_2, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_3, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_4, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_5, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_6, mesh.clone(), material.clone()));
    commands.spawn(setup_wall(wall_pos_7, mesh.clone(), material.clone()));
}

pub fn setup_wall(
    translation: Vec3,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) -> impl Bundle {
    (
        Wall::default(),
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

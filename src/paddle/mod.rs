use avian2d::{
    collision::{
        collider::{Collider, CollisionLayers},
        collision_events::{CollisionEventsEnabled, CollisionStart},
        contact_types::Collisions,
    },
    dynamics::rigid_body::{Friction, GravityScale, Restitution, RigidBody, forces::Forces},
};
use bevy::{
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        observer::On,
        query::With,
        spawn::SpawnRelated,
        system::{Commands, Query, ResMut, Single},
    },
    math::{Vec2, Vec3, primitives::Rectangle},
    mesh::Mesh,
    sprite::Sprite,
    sprite_render::ColorMaterial,
    transform::components::Transform,
    utils::default,
};
use bevy_enhanced_input::{
    action::{
        Action,
        events::{Complete, Fire},
    },
    actions,
    binding::relationship::Bindings,
    bindings,
    prelude::InputAction,
    preset::{axial::Axial, cardinal::Cardinal},
};
use bevy_input::keyboard::KeyCode;

use crate::{
    ball::{Ball, BallPool, BallShot, setup_ball},
    constants::{
        BALL_COLOR, BALL_RADIUS, BALL_SHAPE, BALL_SPEED, BOUNCE_MAX_ANGLE, PADDLE_HEIGHT,
        PADDLE_WIDTH, PADDLE_Y_POS,
    },
    wall::CollisionLayer,
};

#[derive(Component, Default, Clone)]
pub struct Paddle;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct PaddleHorizontalMovement;

pub fn setup_paddle() -> impl Bundle {
    let sprite = Sprite {
        color: bevy::color::Color::srgba(1., 0., 0., 1.),
        custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        ..default()
    };
    let transform = Transform {
        translation: Vec3::new(0., PADDLE_Y_POS, 0.),
        ..default()
    };
    (
        Paddle,
        sprite,
        transform,
        RigidBody::Kinematic,
        Collider::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        GravityScale(0.),
        actions!(Paddle[
            (Action::<BallShot>::new(), bindings![KeyCode::Space]),
            (Action::<PaddleHorizontalMovement>::new(), Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick(), Cardinal::arrows()))),
        ]),
        Restitution::new(1.0),
        Friction::new(0.),
        CollisionLayers::new([CollisionLayer::Paddle], [CollisionLayer::Ball]),
        CollisionEventsEnabled,
    )
}

pub fn shoot_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ball_pool: ResMut<BallPool>,
    paddle_transform: Single<&mut Transform, With<Paddle>>,
) {
    let ball_pos =
        paddle_transform.translation + Vec3::new(0., (PADDLE_HEIGHT / 2.) + BALL_RADIUS, 0.);
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    let _b = commands.spawn(setup_ball(ball_pos, mesh, material));
    ball_pool.decrease_pool_size_by_n(1);
}

pub fn on_shoot_ball(
    _on: On<Complete<BallShot>>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    ball_pool: ResMut<BallPool>,
    paddle_transform: Single<&mut Transform, With<Paddle>>,
) {
    if ball_pool.capacity <= 0 {
        return;
    }

    shoot_ball(commands, meshes, materials, ball_pool, paddle_transform);
}

pub fn on_paddle_move(
    on: On<Fire<PaddleHorizontalMovement>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    timer: bevy::ecs::system::Res<bevy::time::Time>,
) {
    paddle_transform.translation += Vec3::new(on.value.x, 0., 0.) * 500. * timer.delta_secs();
}

pub fn on_ball_and_paddle_collision(
    event: On<CollisionStart>,
    paddle_query: Query<&Paddle>,
    mut forces_query: Query<(Entity, Forces), With<Ball>>,
    collisions: Collisions,
) {
    let paddle_entity = event.collider1;
    let ball_entity = event.collider2;

    if !paddle_query.contains(paddle_entity) {
        return;
    }

    let contact_pair = collisions.get(paddle_entity, ball_entity);
    if contact_pair.is_none() {
        return;
    }

    let manifold = contact_pair.unwrap().manifolds.first();
    if manifold.is_none() {
        return;
    }

    let contact_point = manifold.unwrap().points.first();
    if contact_point.is_none() {
        return;
    }

    let collision_contact_point = contact_point.unwrap();

    let ball_contact_point_x = collision_contact_point.anchor1.x;
    if ball_contact_point_x != 0.00000 {
        // This is the case when the ball hits the edge of the paddle
        // In this case we do not need to modify the bounce angle, so we return
        return;
    }

    let paddle_contact_point_x = collision_contact_point.anchor2.x;
    let paddle_hit_percentage = paddle_contact_point_x / (PADDLE_WIDTH / 2.);

    let bounce_angle = BOUNCE_MAX_ANGLE * paddle_hit_percentage;

    for (entity, mut forces) in &mut forces_query {
        if entity == ball_entity {
            let bounce_vec = Vec2::from_angle(bounce_angle);
            println!("{:#?}", bounce_vec);

            // TODO: modificar el angulo de rebote y la velocidad
        }
    }
}

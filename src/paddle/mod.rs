use avian2d::{
    collision::{
        collider::{Collider, CollisionLayers},
        collision_events::{CollisionEventsEnabled, CollisionStart},
    },
    dynamics::rigid_body::{
        Friction, GravityScale, Restitution, RigidBody,
        forces::{Forces, ReadRigidBodyForces, WriteRigidBodyForces},
        mass_properties::components::ComputedMass,
    },
};
use bevy::{
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        observer::On,
        query::With,
        spawn::SpawnRelated,
        system::{Commands, Query, ResMut, Single},
    },
    log::debug,
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
        BALL_COLOR, BALL_RADIUS, BALL_SHAPE, BOUNCE_MAX_ANGLE, PADDLE_HEIGHT, PADDLE_OFFSET_MARGIN,
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
    paddle_query: Query<&Transform, With<Paddle>>,
    mut ball_query: Query<(Forces, &Transform, &ComputedMass), With<Ball>>,
) {
    let paddle_entity = event.collider1;
    let ball_entity = event.collider2;

    let Ok(paddle_transform) = paddle_query.get(paddle_entity) else {
        return;
    };
    let Ok((mut forces, ball_transform, ball_mass)) = ball_query.get_mut(ball_entity) else {
        return;
    };

    let ball_vel = forces.linear_velocity();
    let offset_x = ball_transform.translation.x - paddle_transform.translation.x;
    if offset_x <= PADDLE_OFFSET_MARGIN && offset_x >= -PADDLE_OFFSET_MARGIN {
        return;
    }
    if offset_x >= PADDLE_WIDTH / 2. || offset_x <= -PADDLE_WIDTH / 2. {
        return;
    }

    let normalized = (offset_x / (PADDLE_WIDTH / 2.)).clamp(-1.0, 1.0);
    let bounce_angle = normalized * BOUNCE_MAX_ANGLE;

    let speed: f32 = ball_vel.length();
    // Using (sin(), cos()) to shift the reference point from the cartesian system from (1, 0) to (0, 1)
    let desired_dir = Vec2::new(bounce_angle.sin(), bounce_angle.cos());
    let desired_vel = desired_dir * speed;

    let delta_v: Vec2 = desired_vel - ball_vel;
    let impulse = delta_v * ball_mass.value();
    forces.apply_linear_impulse(impulse);
}

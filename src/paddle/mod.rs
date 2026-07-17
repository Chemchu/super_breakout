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
        system::{Commands, Query, Res, ResMut, Single},
    },
    math::{Vec2, Vec3, primitives::Rectangle},
    mesh::Mesh,
    sprite::Sprite,
    sprite_render::ColorMaterial,
    time::Time,
    transform::components::Transform,
    utils::default,
};
use bevy_enhanced_input::{
    action::{
        Action,
        events::{Complete, Fire, Start},
    },
    actions,
    binding::relationship::Bindings,
    bindings,
    prelude::InputAction,
    preset::{axial::Axial, cardinal::Cardinal},
};
use bevy_input::keyboard::KeyCode;

use crate::{
    ball::{Action1, Action2, Action3, Action4, Ball, BallPool, BallShot, Pause, setup_ball},
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
            (Action::<PaddleHorizontalMovement>::new(), Bindings::spawn((Axial::left_stick(), Cardinal::arrows()))),
            (Action::<Action1>::new(), bindings![KeyCode::KeyQ]),
            (Action::<Action2>::new(), bindings![KeyCode::KeyW]),
            (Action::<Action3>::new(), bindings![KeyCode::KeyE]),
            (Action::<Action4>::new(), bindings![KeyCode::KeyR]),
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

    commands.spawn(setup_ball(ball_pos, mesh, material));
    ball_pool.decrease_pool_size_by_n(1);
}

pub fn on_shoot_ball(
    _: On<Complete<BallShot>>,
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
    timer: Res<Time>,
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

    if let (Ok(paddle_tf), Ok((mut forces, ball_tf, mass))) = (
        paddle_query.get(paddle_entity),
        ball_query.get_mut(ball_entity),
    ) {
        let offset_x = ball_tf.translation.x - paddle_tf.translation.x;
        let half_width = PADDLE_WIDTH / 2.0;

        if offset_x.abs() <= PADDLE_OFFSET_MARGIN || offset_x.abs() >= half_width {
            return;
        }

        let normalized = (offset_x / half_width).clamp(-1.0, 1.0);
        let bounce_angle = normalized * BOUNCE_MAX_ANGLE;
        let speed = forces.linear_velocity().length();

        // Using (sin(), cos()) to shift the cartesian system reference point from (1, 0) to (0, 1)
        let desired_vel = Vec2::new(bounce_angle.sin(), bounce_angle.cos()) * speed;

        let impulse = (desired_vel - forces.linear_velocity()) * mass.value();
        forces.apply_linear_impulse(impulse);
    }
}

pub fn on_pause_toggle(on: On<Start<Pause>>, commands: Commands) {
    println!("Pause: {:#?}", on.value);

    // TODO: toggle app state
}

pub fn on_action1(on: On<Start<Action1>>, commands: Commands) {
    println!("Action 1: {:#?}", on.value);
}
pub fn on_action2(on: On<Start<Action2>>, commands: Commands) {
    println!("Action 2: {:#?}", on.value);
}
pub fn on_action3(on: On<Start<Action3>>, commands: Commands) {
    println!("Action 3: {:#?}", on.value);
}
pub fn on_action4(on: On<Start<Action4>>, commands: Commands) {
    println!("Action 4: {:#?}", on.value);
}

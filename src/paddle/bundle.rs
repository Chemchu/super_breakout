use avian2d::{
    collision::{
        collider::{Collider, CollisionLayers},
        collision_events::CollisionEventsEnabled,
    },
    dynamics::rigid_body::{Friction, GravityScale, Restitution, RigidBody},
};
use bevy::{
    ecs::{bundle::Bundle, spawn::SpawnRelated},
    math::{Vec2, Vec3, primitives::Rectangle},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use bevy_enhanced_input::{
    action::Action,
    actions,
    binding::relationship::Bindings,
    bindings,
    preset::{axial::Axial, cardinal::Cardinal},
};
use bevy_input::keyboard::KeyCode;

use crate::{
    ball::components::BallLaunchPoint,
    common::{
        components::{BounceDeflector, Pause},
        physical_layers::CollisionLayer,
    },
    input::slots::{Slot1, Slot2, Slot3, Slot4},
    paddle::{
        components::{Paddle, PaddleHorizontalMovement},
        constants::{
            BOUNCE_MAX_ANGLE, PADDLE_HEIGHT, PADDLE_OFFSET_MARGIN, PADDLE_WIDTH, PADDLE_Y_POS,
        },
    },
};

pub fn get_paddle_bundle() -> impl Bundle {
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
        BounceDeflector {
            width: PADDLE_WIDTH,
            max_angle: BOUNCE_MAX_ANGLE,
            dead_zone: PADDLE_OFFSET_MARGIN,
        },
        BallLaunchPoint {
            surface_offset: Vec2::new(0., PADDLE_HEIGHT / 2.0),
        },
        sprite,
        transform,
        RigidBody::Kinematic,
        Collider::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
        GravityScale(0.),
        actions!(Paddle[
            (Action::<Pause>::new(), bindings![KeyCode::Escape]),
            (Action::<Slot1>::new(), bindings![KeyCode::Space]),
            (Action::<Slot2>::new(), bindings![KeyCode::KeyW]),
            (Action::<Slot3>::new(), bindings![KeyCode::KeyE]),
            (Action::<Slot4>::new(), bindings![KeyCode::KeyR]),
            (Action::<PaddleHorizontalMovement>::new(), Bindings::spawn((Axial::left_stick(), Cardinal::arrows()))),
        ]),
        Restitution::new(1.0),
        Friction::new(0.),
        CollisionLayers::new([CollisionLayer::Paddle], [CollisionLayer::Ball]),
        CollisionEventsEnabled,
    )
}

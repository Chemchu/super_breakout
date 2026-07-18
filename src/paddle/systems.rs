use bevy::{
    ecs::{
        observer::On,
        query::With,
        system::{Res, Single},
    },
    math::Vec3,
    time::Time,
    transform::components::Transform,
};
use bevy_enhanced_input::action::events::Fire;

use crate::paddle::components::{Paddle, PaddleHorizontalMovement};

pub fn on_paddle_move(
    on: On<Fire<PaddleHorizontalMovement>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    timer: Res<Time>,
) {
    paddle_transform.translation += Vec3::new(on.value.x, 0., 0.) * 500. * timer.delta_secs();
}

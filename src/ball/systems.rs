use crate::ball::{
    bundle::get_ball_bundle,
    components::{Ball, BallAssets, BallLaunchPoint, BallPool},
    constants::{BALL_ONE_UNIT, BALL_RADIUS, FAN_ANGLE_RAD},
    events::{DoubleBallRequested, LaunchBallRequested, ReverseBallRequested, TripleBallRequested},
};
use avian2d::dynamics::rigid_body::{
    LinearVelocity,
    forces::{ReadRigidBodyForces, WriteRigidBodyForces},
    mass_properties::components::ComputedMass,
};

use avian2d::dynamics::rigid_body::forces::Forces;
use bevy::{
    ecs::{
        entity::Entity,
        observer::On,
        query::{With, Without},
        system::{Commands, Query, Res, ResMut, Single},
    },
    math::{Rot2, Vec2, Vec2Swizzles, Vec3Swizzles},
    transform::components::Transform,
};

pub fn on_launch_ball_requested(
    _: On<LaunchBallRequested>,
    mut commands: Commands,
    mut ball_pool: ResMut<BallPool>,
    ball_assets: Res<BallAssets>,
    launch_point: Single<(&Transform, &BallLaunchPoint), Without<Ball>>,
) {
    let (transform, launch) = *launch_point;
    let ball_pos = transform.translation.xy() + launch.surface_offset + Vec2::new(0., BALL_RADIUS);
    let owned_assets = ball_assets.clone();
    let increase_amount = ball_pool.allocate_balls(BALL_ONE_UNIT);

    if increase_amount > 0 {
        commands.spawn_batch(
            (0..increase_amount)
                .map(move |_| get_ball_bundle(ball_pos, Vec2::Y, owned_assets.clone())),
        );
    }
}

pub fn on_double_ball_requested(
    _: On<DoubleBallRequested>,
    mut commands: Commands,
    mut ball_pool: ResMut<BallPool>,
    ball_assets: Res<BallAssets>,
    ball_query: Query<(Entity, &Transform, &LinearVelocity), With<Ball>>,
) {
    let left_rot = Rot2::radians(FAN_ANGLE_RAD);
    let right_rot = Rot2::radians(-FAN_ANGLE_RAD);

    let existing_ball_count = ball_query.iter().len() as u16;
    let allowed_increase = ball_pool.allocate_balls(existing_ball_count);

    if allowed_increase == 0 {
        return;
    }

    let mut bundles_to_spawn = Vec::with_capacity(allowed_increase as usize * 2);
    for (entity, tf, vel) in ball_query.iter().take(allowed_increase as usize) {
        let base_dir = vel.xy().normalize_or(Vec2::Y);
        let pos = tf.translation.xy();

        bundles_to_spawn.push(get_ball_bundle(
            pos,
            left_rot * base_dir,
            ball_assets.clone(),
        ));
        bundles_to_spawn.push(get_ball_bundle(
            pos,
            right_rot * base_dir,
            ball_assets.clone(),
        ));

        commands.entity(entity).despawn();
    }

    commands.spawn_batch(bundles_to_spawn);
}

pub fn on_triple_ball_requested(
    _: On<TripleBallRequested>,
    mut commands: Commands,
    mut ball_pool: ResMut<BallPool>,
    ball_assets: Res<BallAssets>,
    ball_query: Query<(&Transform, &LinearVelocity), With<Ball>>,
) {
    let left_rot = Rot2::radians(FAN_ANGLE_RAD);
    let right_rot = Rot2::radians(-FAN_ANGLE_RAD);

    let existing_count = ball_query.iter().len() as u16;
    let requested_increase = existing_count * 2;
    let allowed_increase = ball_pool.allocate_balls(requested_increase);
    if allowed_increase == 0 {
        return;
    }

    let source_ball_limit = (allowed_increase / 2) as usize;
    let mut bundles_to_spawn = Vec::with_capacity(allowed_increase as usize);
    for (tf, vel) in ball_query.iter().take(source_ball_limit) {
        let base_dir = vel.xy().normalize_or(Vec2::Y);
        let pos = tf.translation.xy();

        bundles_to_spawn.push(get_ball_bundle(
            pos,
            left_rot * base_dir,
            ball_assets.clone(),
        ));
        bundles_to_spawn.push(get_ball_bundle(
            pos,
            right_rot * base_dir,
            ball_assets.clone(),
        ));
    }

    commands.spawn_batch(bundles_to_spawn);
}

pub fn on_reverse_ball_requested(
    _: On<ReverseBallRequested>,
    mut forces_query: Query<(Forces, &ComputedMass), With<Ball>>,
) {
    for (mut forces, mass) in &mut forces_query {
        let impulse = -2.0 * forces.linear_velocity() * mass.value();
        forces.apply_linear_impulse(impulse);
    }
}

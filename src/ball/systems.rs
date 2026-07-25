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

    ball_pool.increase_current_ball_count(BALL_ONE_UNIT, move |increase_ammount: u16| {
        commands.spawn_batch(
            (0..increase_ammount)
                .map(move |_| get_ball_bundle(ball_pos.clone(), Vec2::Y, owned_assets.clone())),
        );
    });
}

// BUG: this function is not spawning-despawning correctly. Investigate why
pub fn on_double_ball_requested(
    _: On<DoubleBallRequested>,
    mut commands: Commands,
    mut ball_pool: ResMut<BallPool>,
    ball_assets: Res<BallAssets>,
    ball_query: Query<(Entity, &Transform, &LinearVelocity), With<Ball>>,
) {
    let left_rot = Rot2::radians(FAN_ANGLE_RAD);
    let right_rot = Rot2::radians(-FAN_ANGLE_RAD);

    ball_pool.increase_current_ball_count(
        ball_query.iter().len() as u16 * 2,
        move |spawned_balls: u16| {
            commands.spawn_batch(
                ball_query
                    .iter()
                    .flat_map(|(_, tf, vel)| {
                        let base_dir = vel.xy().normalize_or(Vec2::Y);

                        [
                            get_ball_bundle(
                                tf.translation.xy(),
                                left_rot * base_dir,
                                ball_assets.clone(),
                            ),
                            get_ball_bundle(
                                tf.translation.xy(),
                                right_rot * base_dir,
                                ball_assets.clone(),
                            ),
                        ]
                    })
                    .take(spawned_balls as usize)
                    .collect::<Vec<_>>(),
            );

            ball_query
                .iter()
                .take(spawned_balls as usize)
                .for_each(|(entity, _, _)| {
                    commands.entity(entity).despawn();
                });
        },
    );
}

pub fn on_triple_ball_requested(
    _: On<TripleBallRequested>,
    mut commands: Commands,
    ball_assets: Res<BallAssets>,
    ball_query: Query<(&Transform, &LinearVelocity), With<Ball>>,
) {
    let left_rot = Rot2::radians(FAN_ANGLE_RAD);
    let right_rot = Rot2::radians(-FAN_ANGLE_RAD);

    let ball_bundles = ball_query
        .iter()
        .flat_map(|(tf, vel)| {
            let base_dir = vel.xy().normalize_or(Vec2::Y);

            [
                get_ball_bundle(
                    tf.translation.xy(),
                    left_rot * base_dir,
                    ball_assets.clone(),
                ),
                get_ball_bundle(
                    tf.translation.xy(),
                    right_rot * base_dir,
                    ball_assets.clone(),
                ),
            ]
        })
        .collect::<Vec<_>>();

    commands.spawn_batch(ball_bundles);
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

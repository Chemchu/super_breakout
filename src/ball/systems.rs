use crate::ball::{
    bundle::get_ball_bundle,
    components::{Ball, BallAssets, BallLaunchPoint, BallPool},
    constants::{BALL_RADIUS, FAN_ANGLE_RAD},
    events::{DoubleBallRequested, LaunchBallRequested},
};
use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::{
    ecs::{
        entity::Entity,
        observer::On,
        query::With,
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
    launch_point: Single<(&Transform, Option<&BallLaunchPoint>)>,
) {
    println!("Eyyyy");
    if ball_pool.capacity == 0 {
        return;
    }

    let (transform, launch) = *launch_point;
    let ball_pos = transform.translation.xy()
        + launch
            .unwrap_or(&BallLaunchPoint {
                surface_offset: Vec2::new(0.0, 0.0),
            })
            .surface_offset
        + Vec2::new(0., BALL_RADIUS);

    commands.spawn(get_ball_bundle(ball_pos, None, &ball_assets));
    ball_pool.decrease_pool_size_by_n(1);
}

pub fn on_double_ball_requested(
    _: On<DoubleBallRequested>,
    mut commands: Commands,
    ball_assets: Res<BallAssets>,
    ball_query: Query<(Entity, &Transform, &LinearVelocity), With<Ball>>,
) {
    let left_rot = Rot2::radians(FAN_ANGLE_RAD);
    let right_rot = Rot2::radians(-FAN_ANGLE_RAD);

    let ball_bundles = ball_query
        .iter()
        .flat_map(|(_, tf, vel)| {
            let base_dir = vel.xy().normalize_or(Vec2::Y);

            [
                get_ball_bundle(tf.translation.xy(), Some(left_rot * base_dir), &ball_assets),
                get_ball_bundle(
                    tf.translation.xy(),
                    Some(right_rot * base_dir),
                    &ball_assets,
                ),
            ]
        })
        .collect::<Vec<_>>();

    commands.spawn_batch(ball_bundles);

    for (entity, _, _) in &ball_query {
        commands.entity(entity).despawn();
    }
}

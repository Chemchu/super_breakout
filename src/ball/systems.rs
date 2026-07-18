use bevy::{
    asset::Assets,
    ecs::{
        observer::On,
        system::{Commands, ResMut, Single},
    },
    math::Vec2,
    mesh::Mesh,
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

use crate::ball::{
    bundle::get_ball_bundle,
    components::{BallLaunchPoint, BallPool},
    constants::{BALL_COLOR, BALL_RADIUS, BALL_SHAPE},
    events::LaunchBallRequested,
};

pub fn launch_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ball_pool: ResMut<BallPool>,
    launch_point: Single<(&Transform, &BallLaunchPoint)>,
) {
    let (transform, launch) = *launch_point;
    let ball_pos = Vec2::new(transform.translation.x, transform.translation.y)
        + launch.surface_offset
        + Vec2::new(0., BALL_RADIUS);

    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);

    commands.spawn(get_ball_bundle(ball_pos, mesh, material));
    ball_pool.decrease_pool_size_by_n(1);
}

pub fn on_launch_ball_requested(
    _: On<LaunchBallRequested>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    ball_pool: ResMut<BallPool>,
    launch_point: Single<(&Transform, &BallLaunchPoint)>,
) {
    if ball_pool.capacity == 0 {
        return;
    }

    launch_ball(commands, meshes, materials, ball_pool, launch_point);
}

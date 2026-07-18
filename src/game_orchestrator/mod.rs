use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    camera::Camera2d,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    mesh::Mesh,
    sprite_render::ColorMaterial,
};

use crate::{
    common::effects::{ActionLoadout, ActionSlot, GameEffect},
    paddle::bundle::get_paddle_bundle,
    wall::{
        bundle::get_wall_bundle,
        constants::{WALL_COLOR, WALL_SHAPE},
    },
};

pub struct GameOrchestratorPlugin;

impl Plugin for GameOrchestratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_game, setup_default_loadout));
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn(get_paddle_bundle());

    let wall_pos = Vec3::new(0., 400., 0.);
    let mesh = meshes.add(WALL_SHAPE);
    let material = materials.add(WALL_COLOR);
    commands.spawn(get_wall_bundle(wall_pos, mesh, material));
}

fn setup_default_loadout(mut loadout: ResMut<ActionLoadout>) {
    loadout.bind(ActionSlot::Slot1, GameEffect::LaunchBall);
}

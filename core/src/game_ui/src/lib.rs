use bevy::{
    app::{App, Plugin, PreStartup},
    ecs::{resource::Resource, system::Single},
    math::Vec2,
    window::{Window, WindowMode},
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_systems(PreStartup, |mut window: Single<&mut Window>| {
            window.mode = WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current);
        });
    }
}

#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

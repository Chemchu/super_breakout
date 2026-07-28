use bevy::{DefaultPlugins, app::App};

use core::CorePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian2d::PhysicsPlugins::default(),
            bevy_enhanced_input::EnhancedInputPlugin,
        ))
        .add_plugins(CorePlugin)
        .run();
}

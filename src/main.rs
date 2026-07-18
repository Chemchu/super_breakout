use bevy::{DefaultPlugins, app::App, state::app::AppExtStates};

mod ball;
mod common;
mod game_orchestrator;
mod input;
mod paddle;
mod wall;

use crate::common::game_states::AppState;
use ball::BallPlugin;
use common::CommonPlugin;
use game_orchestrator::GamePlugin;
use input::InputPlugin;
use paddle::PaddlePlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            avian2d::PhysicsPlugins::default(),
            bevy_enhanced_input::EnhancedInputPlugin,
        ))
        .init_state::<AppState>()
        .add_plugins((
            CommonPlugin,
            InputPlugin,
            BallPlugin,
            PaddlePlugin,
            WallPlugin,
            GamePlugin,
        ))
        .run();
}

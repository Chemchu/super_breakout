use bevy::{DefaultPlugins, app::App, state::app::AppExtStates};

use common::{game_states::AppState, CommonPlugin};
use ball::BallPlugin;
use game_orchestrator::GameOrchestratorPlugin;
use game_ui::GameUiPlugin;
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
            GameOrchestratorPlugin,
            GameUiPlugin,
        ))
        .run();
}

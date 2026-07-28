use ball::BallPlugin;
use bevy::{
    app::{App, Plugin},
    state::app::AppExtStates,
};
use common::{CommonPlugin, game_states::AppState};
use game_orchestrator::GameOrchestratorPlugin;
use game_ui::GameUiPlugin;
use input::InputPlugin;
use paddle::PaddlePlugin;
use wall::WallPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((
            CommonPlugin,
            InputPlugin,
            BallPlugin,
            WallPlugin,
            PaddlePlugin,
            GameUiPlugin,
            GameOrchestratorPlugin,
        ));
    }
}

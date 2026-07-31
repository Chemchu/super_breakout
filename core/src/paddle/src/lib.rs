use crate::systems::on_paddle_move;
use bevy::{
    app::{App, Plugin},
    ecs::{observer::ObserverSystemExt, schedule::SystemCondition},
    state::condition::in_state,
};
use common::{game_states::AppState, systems::on_pause_toggle};

pub mod components;
pub mod constants;
pub mod systems;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_paddle_move.run_if(in_state(AppState::InGame)))
            .add_observer(
                on_pause_toggle
                    .run_if(in_state(AppState::InGame).or_else(in_state(AppState::Paused))),
            );
    }
}

use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
    Paused,
}

impl AppState {
    fn next(&self) -> Self {
        match *self {
            AppState::Menu => AppState::InGame,
            AppState::InGame => AppState::Paused,
            AppState::Paused => AppState::InGame,
        }
    }
}

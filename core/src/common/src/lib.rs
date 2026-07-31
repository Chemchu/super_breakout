pub mod components;
pub mod constants;
pub mod effects;
pub mod events;
pub mod game_states;
pub mod physical_layers;
pub mod systems;

use bevy::app::{App, FixedUpdate, Plugin};

use crate::{
    effects::ActionLoadout,
    systems::{
        apply_linear_impulse, on_bounce_collision, on_damageable_collision, on_died_event,
        on_slow_time_requested,
    },
};

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionLoadout>()
            .add_observer(on_bounce_collision)
            .add_observer(on_damageable_collision)
            .add_observer(on_died_event)
            .add_observer(on_slow_time_requested)
            .add_systems(FixedUpdate, apply_linear_impulse);
    }
}

use bevy::app::{App, Plugin};
use bevy_enhanced_input::prelude::InputAction;

use crate::input::slots::{dispatch_slot1, dispatch_slot2, dispatch_slot3, dispatch_slot4};

pub mod slots;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(dispatch_slot1)
            .add_observer(dispatch_slot2)
            .add_observer(dispatch_slot3)
            .add_observer(dispatch_slot4);
    }
}

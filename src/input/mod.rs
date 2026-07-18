use bevy::app::{App, Plugin};

use crate::input::slots::{
    dispatch_slot1, dispatch_slot2, dispatch_slot3, dispatch_slot4, dispatch_slot5, dispatch_slot6,
    dispatch_slot7, dispatch_slot8,
};

pub mod slots;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(dispatch_slot1)
            .add_observer(dispatch_slot2)
            .add_observer(dispatch_slot3)
            .add_observer(dispatch_slot4)
            .add_observer(dispatch_slot5)
            .add_observer(dispatch_slot6)
            .add_observer(dispatch_slot7)
            .add_observer(dispatch_slot8);
    }
}

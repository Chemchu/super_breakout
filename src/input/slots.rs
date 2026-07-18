use bevy::ecs::{
    observer::On,
    system::{Commands, Res},
};
use bevy_enhanced_input::{action::events::Start, prelude::InputAction};

use crate::common::effects::{ActionLoadout, ActionSlot, dispatch};

#[derive(InputAction)]
#[action_output(bool)]
pub struct Slot1;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Slot2;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Slot3;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Slot4;

macro_rules! dispatch_slot {
    ($fn_name:ident, $slot_ty:ty, $slot:expr) => {
        pub fn $fn_name(
            _: On<Start<$slot_ty>>,
            loadout: Res<ActionLoadout>,
            mut commands: Commands,
        ) {
            if let Some(effect) = loadout.effect_for($slot) {
                dispatch(effect, &mut commands);
            }
        }
    };
}

dispatch_slot!(dispatch_slot1, Slot1, ActionSlot::Slot1);
dispatch_slot!(dispatch_slot2, Slot2, ActionSlot::Slot2);
dispatch_slot!(dispatch_slot3, Slot3, ActionSlot::Slot3);
dispatch_slot!(dispatch_slot4, Slot4, ActionSlot::Slot4);

use bevy::{
    ecs::{bundle::Bundle, component::Component, spawn::SpawnRelated},
    input::keyboard::KeyCode,
    math::Vec2,
};
use bevy_enhanced_input::{
    action::Action,
    actions,
    binding::relationship::Bindings,
    bindings,
    prelude::InputAction,
    preset::{axial::Axial, cardinal::Cardinal},
};
use common::components::Pause;

use crate::slots::{Slot1, Slot2, Slot3, Slot4, Slot5, Slot6, Slot7, Slot8};

#[derive(Component)]
pub struct InputManager;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct LeftStickAxis;

pub fn get_input_manager() -> impl Bundle {
    (
        InputManager,
        actions!(InputManager[
            (Action::<Pause>::new(), bindings![KeyCode::Escape]),
            (Action::<Slot1>::new(), bindings![KeyCode::Space]),
            (Action::<Slot2>::new(), bindings![KeyCode::KeyQ]),
            (Action::<Slot3>::new(), bindings![KeyCode::KeyW]),
            (Action::<Slot4>::new(), bindings![KeyCode::KeyE]),
            (Action::<Slot5>::new(), bindings![KeyCode::KeyR]),
            (Action::<Slot6>::new(), bindings![KeyCode::KeyA]),
            (Action::<Slot7>::new(), bindings![KeyCode::KeyS]),
            (Action::<Slot8>::new(), bindings![KeyCode::KeyD]),
            (Action::<LeftStickAxis>::new(), Bindings::spawn((Axial::left_stick(), Cardinal::arrows()))),
        ]),
    )
}

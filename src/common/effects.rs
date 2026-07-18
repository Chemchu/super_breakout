use bevy::ecs::{resource::Resource, system::Commands};
use std::collections::HashMap;

use crate::ball::events::{DoubleBallRequested, LaunchBallRequested, TripleBallRequested};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEffect {
    LaunchBall,
    Double,
    Triple,
    // future: SlowTime, MultiBall, Shield...
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionSlot {
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
}

#[derive(Resource, Default)]
pub struct ActionLoadout {
    bindings: HashMap<ActionSlot, GameEffect>,
}

impl ActionLoadout {
    pub fn bind(&mut self, slot: ActionSlot, effect: GameEffect) {
        self.bindings.insert(slot, effect);
    }

    pub fn effect_for(&self, slot: ActionSlot) -> Option<GameEffect> {
        self.bindings.get(&slot).copied()
    }
}

pub fn dispatch(effect: GameEffect, commands: &mut Commands) {
    match effect {
        GameEffect::LaunchBall => commands.trigger(LaunchBallRequested),
        GameEffect::Double => commands.trigger(DoubleBallRequested),
        GameEffect::Triple => commands.trigger(TripleBallRequested),
    }
}

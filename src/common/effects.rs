use bevy::ecs::{resource::Resource, system::Commands};
use std::collections::HashMap;

use crate::ball::events::{
    DoubleBallRequested, LaunchBallRequested, ReverseBallRequested, TripleBallRequested,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameEffect {
    LaunchBall,
    DoubleBall,
    TripleBall,
    ReverseBall,
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
    pub fn binds(&mut self, slots: HashMap<ActionSlot, GameEffect>) {
        slots.iter().for_each(|(slot, effect)| {
            self.bindings.insert(*slot, *effect);
        });
    }

    pub fn effect_for(&self, slot: ActionSlot) -> Option<GameEffect> {
        self.bindings.get(&slot).copied()
    }
}

pub fn dispatch(effect: GameEffect, commands: &mut Commands) {
    match effect {
        GameEffect::LaunchBall => commands.trigger(LaunchBallRequested),
        GameEffect::DoubleBall => commands.trigger(DoubleBallRequested),
        GameEffect::TripleBall => commands.trigger(TripleBallRequested),
        GameEffect::ReverseBall => commands.trigger(ReverseBallRequested),
    }
}

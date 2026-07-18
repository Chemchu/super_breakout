use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(EntityEvent)]
pub struct Died {
    pub entity: Entity,
}

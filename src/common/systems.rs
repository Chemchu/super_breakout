use avian2d::{
    collision::collision_events::CollisionStart,
    dynamics::rigid_body::{
        forces::{Forces, ReadRigidBodyForces, WriteRigidBodyForces},
        mass_properties::components::ComputedMass,
    },
};
use bevy::{
    ecs::{
        entity::Entity,
        observer::On,
        query::With,
        system::{Commands, Query},
    },
    math::Vec2,
    transform::components::Transform,
};
use bevy_enhanced_input::action::events::Start;

use crate::{
    common::{
        components::{BounceDeflector, Bounceable, Damage, Health, NeedsImpulse},
        events::Died,
    },
    input::Pause,
};

pub fn on_damageable_collision(
    event: On<CollisionStart>,
    mut health_query: Query<&mut Health>,
    damage_query: Query<&Damage>,
    commands: Commands,
) {
    let (entity_a, entity_b) = (event.collider1, event.collider2);

    if let (Ok(mut health), Ok(damage)) =
        (health_query.get_mut(entity_a), damage_query.get(entity_b))
    {
        health.take_damage(commands, damage.0, entity_a);
    } else if let (Ok(mut health), Ok(damage)) =
        (health_query.get_mut(entity_b), damage_query.get(entity_a))
    {
        health.take_damage(commands, damage.0, entity_b);
    }
}

pub fn apply_linear_impulse(
    mut query: Query<(Entity, Forces, &NeedsImpulse)>,
    mut commands: Commands,
) {
    for (entity, mut forces, needs_impulse) in &mut query {
        forces.apply_force(needs_impulse.impulse);
        commands.entity(entity).remove::<NeedsImpulse>();
    }
}

pub fn on_died_event(event: On<Died>, mut commands: Commands) {
    commands.entity(event.entity).despawn();
}

pub fn on_bounce_collision(
    event: On<CollisionStart>,
    deflector_query: Query<(&Transform, &BounceDeflector)>,
    mut bounceable_query: Query<(Forces, &Transform, &ComputedMass), With<Bounceable>>,
) {
    let (entity_a, entity_b) = (event.collider1, event.collider2);

    let (deflector, bounced) = if let Ok(d) = deflector_query.get(entity_a) {
        (d, entity_b)
    } else if let Ok(d) = deflector_query.get(entity_b) {
        (d, entity_a)
    } else {
        return;
    };

    let (deflector_tf, deflector_data) = deflector;

    if let Ok((mut forces, ball_tf, mass)) = bounceable_query.get_mut(bounced) {
        let offset_x = ball_tf.translation.x - deflector_tf.translation.x;
        let half_width = deflector_data.width / 2.0;

        if offset_x.abs() <= deflector_data.dead_zone || offset_x.abs() >= half_width {
            return;
        }

        let normalized = (offset_x / half_width).clamp(-1.0, 1.0);
        let bounce_angle = normalized * deflector_data.max_angle;
        let speed = forces.linear_velocity().length();

        // Using (sin(), cos()) to shift the cartesian system reference point from (1, 0) to (0, 1)
        let desired_vel = Vec2::new(bounce_angle.sin(), bounce_angle.cos()) * speed;
        let impulse = (desired_vel - forces.linear_velocity()) * mass.value();

        forces.apply_linear_impulse(impulse);
    }
}

pub fn on_pause_toggle(on: On<Start<Pause>>) {
    println!("Pause: {:#?}", on.value);
    // TODO: toggle AppState between InGame and Paused via NextState<AppState>
}

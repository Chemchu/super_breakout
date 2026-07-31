use avian2d::{
    collision::collider::{Collider, CollisionLayers},
    dynamics::rigid_body::{
        Friction, GravityScale, MaxLinearSpeed, Restitution, RigidBody,
        mass_properties::components::Mass,
    },
    interpolation::TransformInterpolation,
};
use bevy::{
    asset::{Assets, Handle},
    ecs::{bundle::Bundle, component::Component, resource::Resource, world::FromWorld},
    math::{Vec2, Vec3},
    mesh::{Mesh, Mesh2d},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::constants::{
    BALL_COLOR, BALL_IMPULSE, BALL_MASS, BALL_MAX_SPEED, BALL_POOL_MAX_CAPACITY, BALL_RADIUS,
    BALL_SHAPE,
};
use common::{
    components::{Bounceable, Damage, NeedsImpulse},
    physical_layers::CollisionLayer,
};

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub damage: Damage,
    pub needs_impulse: NeedsImpulse,
    pub bounceable: Bounceable,
    pub mass: Mass,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
    pub interpolation: TransformInterpolation,
    pub restitution: Restitution,
    pub friction: Friction,
    pub max_speed: MaxLinearSpeed,
    pub collision_layers: CollisionLayers,
}

pub fn get_ball_bundle(
    translation: Vec2,
    launch_direction: Vec2,
    assets: BallAssets,
) -> BallBundle {
    BallBundle {
        ball: Ball::default(),
        damage: Damage(1.5_f32),
        needs_impulse: NeedsImpulse {
            impulse: launch_direction.normalize_or_zero() * BALL_IMPULSE,
        },
        bounceable: Bounceable,
        mass: Mass(BALL_MASS),
        transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0.0)),
        mesh: Mesh2d(assets.mesh),
        material: MeshMaterial2d(assets.material),
        collider: Collider::circle(BALL_RADIUS),
        rigid_body: RigidBody::Dynamic,
        gravity_scale: GravityScale(0.),
        interpolation: TransformInterpolation,
        restitution: Restitution::new(1.0),
        friction: Friction::new(0.),
        max_speed: MaxLinearSpeed(BALL_MAX_SPEED),
        collision_layers: CollisionLayers::new(
            [CollisionLayer::Ball],
            [CollisionLayer::Wall, CollisionLayer::Paddle],
        ),
    }
}

#[derive(Component, Default, Clone, Debug)]
#[require(Damage, Bounceable)]
pub struct Ball;

#[derive(Resource, Clone)]
pub struct BallAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

impl FromWorld for BallAssets {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(BALL_SHAPE);
        let material = world
            .resource_mut::<Assets<ColorMaterial>>()
            .add(BALL_COLOR);

        BallAssets { mesh, material }
    }
}

#[derive(Resource)]
pub struct BallPool {
    max_capacity: u16,
    pub current_ball_count: u16,
}

impl Default for BallPool {
    fn default() -> Self {
        BallPool {
            max_capacity: BALL_POOL_MAX_CAPACITY,
            current_ball_count: 0,
        }
    }
}

impl BallPool {
    pub fn allocate_balls(&mut self, increment: u16) -> u16 {
        if self.current_ball_count >= self.max_capacity {
            return 0;
        }

        let allowed_space = self.max_capacity - self.current_ball_count;
        let spawned_balls = increment.min(allowed_space);

        self.current_ball_count += spawned_balls;
        spawned_balls
    }

    pub fn deallocate_balls(&mut self, decrement: u16) -> u16 {
        let actual_removed = self.current_ball_count.min(decrement);
        self.current_ball_count -= actual_removed;
        actual_removed
    }
}

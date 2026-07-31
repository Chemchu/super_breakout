use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Paddle,
    Ball,
    Wall,
}

use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec2,
}

impl Velocity {
    pub fn new(inital_velocity: Vec2) -> Self {
        Velocity {
            direction: inital_velocity,
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.direction.x * time.delta_seconds();
        transform.translation.y += velocity.direction.y * time.delta_seconds();
    }
}

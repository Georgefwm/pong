use crate::ball::*;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Component)]
pub struct Collider;

// Too much math. I don't wanna touch this again.
pub fn check_colliders(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = Vec2::new(crate::BALL_SIZE * 2.0, crate::BALL_SIZE * 2.0);

    // check collision with walls
    for transform in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.direction.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.direction.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.direction.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.direction.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.direction.x = 1.0;
                ball_velocity.direction.y = 0.0;

                // Get distance between ball center and paddle center
                let distance_to_center = ball_transform.translation.y - transform.translation.y;

                // Normalise distance to [-1, 1] based on paddle height
                let normalised_distance_to_center =
                    distance_to_center / (crate::PADDLE_SCALE.y / 2.0);

                // Adjust the angle based on the
                let bounce_angle = normalised_distance_to_center
                    * f32::to_radians(crate::BALL_MAX_BOUNCE_ANGLE_DEGREES);

                let sin = f32::sin(bounce_angle);
                let cos = f32::cos(bounce_angle);

                // Calculate the new direction vector
                ball_velocity.direction.x =
                    ball_velocity.direction.x * cos - ball_velocity.direction.y * sin;

                ball_velocity.direction.y =
                    ball_velocity.direction.x * sin + ball_velocity.direction.y * sin;

                // Flip x value if we are bouncing off left side
                if collision == Collision::Left {
                    ball_velocity.direction.x *= -1.0;
                }

                // Add ball speed to normalied direction vector
                ball_velocity.direction *= crate::BALL_SPEED;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.direction.y = -ball_velocity.direction.y;
            }
        }
    }
}

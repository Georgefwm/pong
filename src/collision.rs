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
                // Get distance between ball center and paddle center
                let distance_to_center = ball_transform.translation.y - transform.translation.y;

                // Normalise distance to [-1, 1] based on paddle height
                let normalised_distance_to_center = f32::clamp(
                    distance_to_center / (crate::PADDLE_SCALE.y / 2.0),
                    -1.0,
                    1.0,
                );

                // Adjust the angle based on the normalised
                let mut bounce_angle =
                    normalised_distance_to_center * crate::BALL_MAX_BOUNCE_ANGLE_DEGREES;

                let mut base_direction: Vec2 = Vec2::new(1.0, 0.0);

                if collision == Collision::Left {
                    bounce_angle *= -1.0;
                    base_direction.x *= -1.0;
                }

                let rotation_matrix = Mat2::from_angle(bounce_angle.to_radians());

                // Calculate the new direction vector
                // ball_velocity.direction = rotation_matrix * base_direction;
                ball_velocity.direction = rotation_matrix.mul_vec2(base_direction);

                // Add ball speed to normalied direction vector
                ball_velocity.direction.x *= crate::BALL_SPEED;
                ball_velocity.direction.y *=
                    crate::BALL_SPEED / f32::cos(bounce_angle.abs().to_radians());
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            // ignore y velocity if hitting corner of paddle
            if reflect_y {
                ball_velocity.direction.y = -ball_velocity.direction.y;
            }
        }
    }
}

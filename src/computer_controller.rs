use crate::ball::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ComputerController {
    pub wish_direction: f32,
}

impl Default for ComputerController {
    fn default() -> Self {
        ComputerController {
            wish_direction: 0.0,
        }
    }
}

pub fn get_input(
    mut controller_query: Query<(&mut ComputerController, &Transform), With<ComputerController>>,
    ball_query: Query<(&Transform, &Velocity), With<Ball>>,
) {
    let (mut controller, transform) = controller_query.single_mut();
    let (ball_transform, ball_velocity) = ball_query.single();

    // Good enough for now
    // TODO: Make slightly more sofisticated
    controller.wish_direction = (ball_transform.translation.y
        - (transform.translation.y - ball_velocity.direction.normalize().y * 100.0))
        / (crate::WINDOW_HEIGHT / 4.0);
}

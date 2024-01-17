use crate::ball::Ball;
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
    ball_query: Query<&Transform, With<Ball>>,
) {
    let (mut controller, transform) = controller_query.single_mut();
    let ball_transform = ball_query.single();

    // Good enough for now
    // TODO: Make slightly more sofisticated
    controller.wish_direction =
        (ball_transform.translation.y - transform.translation.y) / (crate::WINDOW_HEIGHT / 5.0);
}

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerController {
    pub wish_direction: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        PlayerController {
            wish_direction: 0.0,
        }
    }
}

pub fn get_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut PlayerController>) {
    let mut player_controller = query.single_mut();
    player_controller.wish_direction = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        player_controller.wish_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_controller.wish_direction -= 1.0;
    }
}

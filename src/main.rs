use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
    window::WindowResolution,
};

mod ball;
mod collision;
mod computer_controller;
mod player_controller;
mod post_processing;
mod scoreboard;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const BALL_COLOR: Color = Color::WHITE;
const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 600.0;
const BALL_START_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BALL_INITIAL_VELOCITY: Vec2 = Vec2::new(1.0, 1.0);
const BALL_MAX_BOUNCE_ANGLE_DEGREES: f32 = 60.0; // Assumes value between [0, 90]

const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SCALE: Vec3 = Vec3::new(20.0, 150.0, 1.0);
const PADDLE_PADDING: f32 = 25.0;
const PADDLE_SPEED: f32 = 500.0;

const WALL_COLOR: Color = Color::WHITE;
const WALL_SCALE: Vec3 = Vec3::new(WINDOW_WIDTH, 10.0, 1.0);

const SCOREBOARD_FONT_SIZE: f32 = 64.0;
const SCOREBOARD_TEXT_TOP_PADDING: Val = Val::Px(40.0);
const SCOREBOARD_TEXT_COLOR: Color = Color::WHITE;

const COMPUTER_LEAD_AMOUNT: f32 = 10.0;
const BACKGROUND_COLOR: Color = Color::DARK_GRAY;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                title: "pong - by George Mclachlan".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(scoreboard::Scoreboard {
            human: 0,
            computer: 0,
        })
        .add_systems(Startup, setup)
        .add_plugins(post_processing::PostProcessingPlugin)
        .add_event::<scoreboard::ScoreEvent>()
        .add_systems(
            FixedUpdate,
            (
                ball::apply_velocity,
                player_controller::get_input,
                computer_controller::get_input,
                move_paddles,
                collision::check_colliders,
                check_scored,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (scoreboard::update_scoreboard, bevy::window::close_on_esc),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn 2d camera
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Default,
                ..default()
            },
            ..default()
        },
        post_processing::PostProcessingSettings {
            intensity: 0.3,
            color_aberration: Mat3::IDENTITY,
        },
    ));

    // Top wall
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_HEIGHT / 2.0 - (WALL_SCALE.y / 2.0), 0.0),
                scale: WALL_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        collision::Collider,
    ));

    // Bottom wall
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_HEIGHT / 2.0 * -1.0 + (WALL_SCALE.y / 2.0), 0.0),
                scale: WALL_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            ..default()
        },
        collision::Collider,
    ));

    // Spawn ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_START_POSITION),
            ..default()
        },
        ball::Ball,
        collision::Collider,
        ball::Velocity::new(BALL_INITIAL_VELOCITY.normalize() * BALL_SPEED),
    ));

    // Spawn player paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_WIDTH / 2.0 * -1.0 + (PADDLE_SCALE.x / 2.0) + PADDLE_PADDING,
                    0.0,
                    0.0,
                ),
                scale: PADDLE_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        collision::Collider,
        player_controller::PlayerController::default(),
    ));

    // Spawn computer paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_WIDTH / 2.0 - (PADDLE_SCALE.x / 2.0) - PADDLE_PADDING,
                    0.0,
                    0.0,
                ),
                scale: PADDLE_SCALE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        collision::Collider,
        computer_controller::ComputerController::default(),
    ));

    // Spawn the scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "0",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::new(
                " - SCORE - ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_TEXT_COLOR,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            top: SCOREBOARD_TEXT_TOP_PADDING,
            ..default()
        }),
    );
}

// Takes input from controllers and applies movement to the paddles
fn move_paddles(
    mut computer_controller_query: Query<
        (&computer_controller::ComputerController, &mut Transform),
        Without<player_controller::PlayerController>,
    >,
    mut player_controller_query: Query<
        (&player_controller::PlayerController, &mut Transform),
        Without<computer_controller::ComputerController>,
    >,
    time: Res<Time>,
) {
    let (computer_controller, mut computer_transform) = computer_controller_query.single_mut();
    let (player_controller, mut player_transform) = player_controller_query.single_mut();

    // Calculate the new paddle positions based on wish_direction
    let new_player_paddle_position = player_transform.translation.y
        + player_controller.wish_direction * PADDLE_SPEED * time.delta_seconds();

    let new_computer_paddle_position = computer_transform.translation.y
        + computer_controller.wish_direction * PADDLE_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let upper_bound = WINDOW_HEIGHT / 2.0 - (WALL_SCALE.y + PADDLE_SCALE.y / 2.0);
    let lower_bound = (WINDOW_HEIGHT / 2.0 * -1.0) + (WALL_SCALE.y + PADDLE_SCALE.y / 2.0);

    player_transform.translation.y = new_player_paddle_position.clamp(lower_bound, upper_bound);
    computer_transform.translation.y = new_computer_paddle_position.clamp(lower_bound, upper_bound);
}

// Checks if ball has passed a players paddle, if so, awards a point
// TODO: Are the paddles positions supposed to be reset on score?
fn check_scored(
    mut scoreboard: ResMut<scoreboard::Scoreboard>,
    mut query: Query<(&mut Transform, &mut ball::Velocity)>,
    mut score_events: EventWriter<scoreboard::ScoreEvent>,
) {
    let (mut ball_transform, mut ball_velocity) = query.single_mut();

    if ball_transform.translation.x.abs() > WINDOW_WIDTH / 2.0 + 50.0 {
        if ball_transform.translation.x > 0.0 {
            scoreboard.human += 1;
        } else {
            scoreboard.computer += 1;
        }

        score_events.send_default();

        ball_transform.translation = BALL_START_POSITION;
        ball_velocity.direction = BALL_INITIAL_VELOCITY.normalize() * BALL_SPEED;

        if scoreboard.human + scoreboard.computer % 2 == 1 {
            ball_velocity.direction.y *= -1.0;
        }
    }
}

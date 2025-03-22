use bevy::{
    input::keyboard::KeyboardInput, prelude::*, render::camera::ScalingMode
};

mod ball;
mod paddle;
mod pixel_grid;
mod scoreboard;

use pixel_grid::{PIXEL_SIZE, get_half_screen_size};
use scoreboard::ScoreBoard;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        pixel_grid::GRID_WIDTH as f32 * PIXEL_SIZE,
                        pixel_grid::GRID_HEIGHT as f32 * PIXEL_SIZE,
                    ).into(),
                    title: "Pixel Pong".to_string(),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ScoreBoard::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, paddle::spawn_left_paddle)
        .add_systems(Startup, paddle::spawn_right_paddle)
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, ball::spawn_ball)
        .add_systems(Update, close_on_escape)
        .add_systems(Update, paddle::move_left_paddle)
        .add_systems(Update, paddle::move_right_paddle)
        .add_systems(Update, ball::update_ball)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = Camera2d;
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
            min_width: pixel_grid::GRID_WIDTH as f32 * PIXEL_SIZE,
            min_height: pixel_grid::GRID_HEIGHT as f32 * PIXEL_SIZE,
        },
        ..OrthographicProjection::default_2d()
    };
    commands.spawn((camera, projection));
}

fn spawn_background(mut commands: Commands) {
    let (half_width, half_height) = get_half_screen_size();

    // Create grid border
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(half_width * 2.0, half_height * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
    ));

    // Create center line
    let center_pixel_size = PIXEL_SIZE * 0.5;
    let center_offset = 0.0;

    for y in -((pixel_grid::GRID_HEIGHT / 2) as i32)..(pixel_grid::GRID_HEIGHT / 2) as i32 {
        if y % 2 == 0 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(center_pixel_size, center_pixel_size)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    0.0,
                    (y as f32 * PIXEL_SIZE) + center_offset,
                    -0.05,
                )),
            ));
        }
    }
}

fn close_on_escape(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input.read() {
        if event.key_code == KeyCode::Escape {
            exit_events.send(AppExit::Success);
        }
    }
}

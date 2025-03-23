use bevy::prelude::*;

mod ball;
mod paddle;
mod pixel_grid;
mod scoreboard;
mod window;

use pixel_grid::{PIXEL_SIZE, get_half_screen_size};
use scoreboard::ScoreBoard;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(window::get_window_settings()),
        )
        .insert_resource(ScoreBoard::default())
        .add_systems(Startup, window::setup_camera)
        .add_systems(Startup, paddle::spawn_left_paddle)
        .add_systems(Startup, paddle::spawn_right_paddle)
        .add_systems(Startup, spawn_background)
        .add_systems(Startup, ball::spawn_ball)
        .add_systems(Update, window::close_on_escape)
        .add_systems(Update, paddle::move_left_paddle)
        .add_systems(Update, paddle::move_right_paddle)
        .add_systems(Update, ball::update_ball)
        .run();
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

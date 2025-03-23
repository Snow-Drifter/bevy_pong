use bevy::prelude::*;

mod ball;
mod background;
mod paddle;
mod pixel_grid;
mod scoreboard;
mod window;

use pixel_grid::get_half_screen_size;
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
        .add_systems(Startup, background::spawn_background)
        .add_systems(Startup, ball::spawn_ball)
        .add_systems(Update, window::close_on_escape)
        .add_systems(Update, paddle::move_left_paddle)
        .add_systems(Update, paddle::move_right_paddle)
        .add_systems(Update, ball::update_ball)
        .run();
}

pub const MIDDLE_LINE_WIDTH: f32 = 25.0;

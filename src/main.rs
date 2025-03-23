use bevy::prelude::*;

mod ball;
mod background;
mod paddle;
mod pixel_grid;
mod scoreboard;
mod window;

use scoreboard::ScoreBoard;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(window::get_window_settings()),
        )
        .insert_resource(ScoreBoard::default())
        .add_systems(Startup, (
            window::setup_camera,
            paddle::spawn_left_paddle,
            paddle::spawn_right_paddle,
            background::spawn_background,
            ball::spawn_ball,
        ))
        .add_systems(Update, (
            window::close_on_escape,
            paddle::move_left_paddle,
            paddle::move_right_paddle,
            ball::update_ball,
        ))
        .run();
}

pub const MIDDLE_LINE_WIDTH: f32 = 25.0;

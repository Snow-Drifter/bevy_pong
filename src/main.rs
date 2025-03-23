use bevy::prelude::*;

mod ball;
mod background;
mod paddle;
mod pixel_grid;
mod scoreboard;
mod window;

use scoreboard::{ScoreBoard, ScoreEvent};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(window::get_window_settings()),
        )
        .insert_resource(ScoreBoard::default())
        .add_event::<ScoreEvent>()
        .add_systems(Startup, (
            window::setup_camera,
            paddle::spawn_left_paddle,
            paddle::spawn_right_paddle,
            background::spawn_background,
            ball::spawn_ball,
            scoreboard::spawn_scoreboard,
        ))
        .add_systems(Update, (
            window::close_on_escape,
            paddle::move_left_paddle,
            paddle::move_right_paddle,
            ball::update_ball,
            scoreboard::update_scoreboard,
            scoreboard::update_scoreboard_text,
            ball::reset_ball_system,
        ))
        .run();
}

pub const MIDDLE_LINE_WIDTH: f32 = 25.0;

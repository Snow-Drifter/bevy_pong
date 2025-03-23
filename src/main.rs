/// Main entry point for the Pixel Pong game application
use bevy::{prelude::*, render::pipelined_rendering::PipelinedRenderingPlugin};

/// Background module responsible for creating the play area and visual elements
mod background;
/// Ball module with ball physics, movement and collision detection
mod ball;
/// Paddle module handling player input and paddle positioning
mod paddle;
/// Scoreboard module for tracking and displaying player scores
mod scoreboard;
/// Window module for handling window settings and camera configuration
mod window;

use scoreboard::{ScoreBoard, ScoreEvent};

/// Main function that configures and runs the game
///
/// Sets up the Bevy app with all necessary plugins, resources, events, and systems.
/// Organizes systems into appropriate startup and update schedules.
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window::get_window_settings())
                .disable::<PipelinedRenderingPlugin>(),
        )
        .insert_resource(ScoreBoard::default())
        .add_event::<ScoreEvent>()
        .add_systems(
            Startup,
            (
                window::setup_camera,
                paddle::spawn_left_paddle,
                paddle::spawn_right_paddle,
                background::spawn_background,
                ball::spawn_ball,
                scoreboard::spawn_scoreboard,
            ),
        )
        .add_systems(
            Update,
            (
                window::close_on_escape,
                paddle::move_left_paddle,
                paddle::move_right_paddle,
                ball::update_ball,
                scoreboard::update_scoreboard,
                scoreboard::update_scoreboard_text,
                ball::reset_ball_system,
            ),
        )
        .run();
}

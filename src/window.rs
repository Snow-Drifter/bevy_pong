/// Module for handling window configuration, camera setup, and input events
use bevy::{
    input::keyboard::KeyboardInput, 
    prelude::*, 
    render::camera::ScalingMode
};

/// Width of the game window in pixels
pub const WIDTH: usize = 768;
/// Height of the game window in pixels
pub const HEIGHT: usize = 480;

/// Returns window configuration with default settings for Pixel Pong
///
/// Creates a WindowPlugin with specified dimensions and title.
/// Sets up the primary window with appropriate resolution and title.
pub fn get_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: (WIDTH as f32, HEIGHT as f32).into(),
            title: "Pixel Pong".to_string(),
            ..default()
        }),
        ..default()
    }
}

/// System that handles closing the application when Escape key is pressed
///
/// Listens for keyboard input events and sends an AppExit event when
/// the Escape key is detected, which will gracefully close the application.
pub fn close_on_escape(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input.read() {
        if event.key_code == KeyCode::Escape {
            exit_events.send(AppExit::Success);
        }
    }
}

/// Sets up a 2D camera that maintains proper scaling regardless of window size
///
/// Creates a Camera2d entity with an OrthographicProjection configured to
/// automatically scale while maintaining the minimum specified dimensions.
/// This ensures consistent gameplay experience across different window sizes.
pub fn setup_camera(mut commands: Commands) {
    let camera = Camera2d;
    let projection = OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
            min_width: WIDTH as f32,
            min_height: HEIGHT as f32,
        },
        ..OrthographicProjection::default_2d()
    };
    commands.spawn((camera, projection));
} 
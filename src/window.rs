use bevy::{
    input::keyboard::KeyboardInput, 
    prelude::*, 
    render::camera::ScalingMode
};

// Constants for screen dimensions
pub const WIDTH: usize = 768;  // Horizontal pixels
pub const HEIGHT: usize = 480; // Vertical pixels

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
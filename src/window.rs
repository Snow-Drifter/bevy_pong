use bevy::{
    input::keyboard::KeyboardInput, 
    prelude::*, 
    render::camera::ScalingMode
};
use crate::pixel_grid::{get_screen_size, GRID_HEIGHT, GRID_WIDTH};

pub fn get_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            resolution: get_screen_size().into(),
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
            min_width: GRID_WIDTH as f32,
            min_height: GRID_HEIGHT as f32,
        },
        ..OrthographicProjection::default_2d()
    };
    commands.spawn((camera, projection));
} 
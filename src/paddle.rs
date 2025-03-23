use bevy::prelude::*;
use crate::pixel_grid::{GRID_HEIGHT, GRID_WIDTH};

pub const PADDLE_HEIGHT: f32 = 28.0;
pub const PADDLE_WIDTH: f32 = 9.0;
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_OFFSET: f32 = 40.0;

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct RightPaddle;

pub fn spawn_left_paddle(mut commands: Commands) {
    let half_width = GRID_WIDTH as f32 / 2.0;
    
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-half_width + PADDLE_WIDTH + PADDLE_OFFSET, 0.0, 0.0),
            ..default()
        },
        LeftPaddle,
    ));
}

pub fn spawn_right_paddle(mut commands: Commands) {
    let half_width = GRID_WIDTH as f32 / 2.0;
    
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(half_width - PADDLE_WIDTH - PADDLE_OFFSET, 0.0, 0.0),
            ..default()
        },
        RightPaddle,
    ));
}

pub fn move_left_paddle(
    mut query: Query<&mut Transform, With<LeftPaddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_boundary = (GRID_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0;
    
    let mut transform = query.single_mut();
    let move_amount = PADDLE_SPEED * time.delta_secs();
    
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.y = (transform.translation.y + move_amount).min(paddle_boundary);
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.y = (transform.translation.y - move_amount).max(-paddle_boundary);
    }
}

pub fn move_right_paddle(
    mut query: Query<&mut Transform, With<RightPaddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_boundary = (GRID_HEIGHT as f32 - PADDLE_HEIGHT) / 2.0;
    
    let mut transform = query.single_mut();
    let move_amount = PADDLE_SPEED * time.delta_secs();
    
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y = (transform.translation.y + move_amount).min(paddle_boundary);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y = (transform.translation.y - move_amount).max(-paddle_boundary);
    }
} 
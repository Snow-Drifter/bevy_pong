use bevy::prelude::*;
use crate::pixel_grid::{PIXEL_SIZE, get_half_screen_size};

#[derive(Component)]
pub struct LeftPaddle;

#[derive(Component)]
pub struct RightPaddle;

pub fn spawn_left_paddle(mut commands: Commands) {
    let (half_width, _) = get_half_screen_size();
    
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PIXEL_SIZE, PIXEL_SIZE * 4.0)), // 1x3 pixels paddle
            ..default()
        },
        Transform {
            translation: Vec3::new(-half_width + PIXEL_SIZE, 0.0, 0.0),
            ..default()
        },
        LeftPaddle,
    ));
}

pub fn spawn_right_paddle(mut commands: Commands) {
    let (half_width, _) = get_half_screen_size();
    
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PIXEL_SIZE, PIXEL_SIZE * 4.0)), // 1x3 pixels paddle
            ..default()
        },
        Transform {
            translation: Vec3::new(half_width - PIXEL_SIZE, 0.0, 0.0),
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
    let (_, half_height) = get_half_screen_size();
    
    // Paddle is 3 pixels tall, so need to offset by 1.5 pixels
    let paddle_height = PIXEL_SIZE * 4.0;
    let paddle_half_height = paddle_height / 2.0;
    let paddle_boundary = half_height - paddle_half_height;
    
    let mut transform = query.single_mut();
    let base_speed = PIXEL_SIZE * 4.0; // Move 4 pixels per second
    let move_amount = base_speed * time.delta_secs();
    
    // Move based on accumulated time instead of jumping full pixels
    if keyboard.pressed(KeyCode::KeyW) {
        let new_y = (transform.translation.y + move_amount).min(paddle_boundary);
        transform.translation.y = new_y;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        let new_y = (transform.translation.y - move_amount).max(-paddle_boundary);
        transform.translation.y = new_y;
    }
}

pub fn move_right_paddle(
    mut query: Query<&mut Transform, With<RightPaddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (_, half_height) = get_half_screen_size();
    
    // Paddle is 3 pixels tall, so need to offset by 1.5 pixels
    let paddle_height = PIXEL_SIZE * 4.0;
    let paddle_half_height = paddle_height / 2.0;
    let paddle_boundary = half_height - paddle_half_height;
    
    let mut transform = query.single_mut();
    let base_speed = PIXEL_SIZE * 4.0; // Move 4 pixels per second
    let move_amount = base_speed * time.delta_secs();
    
    // Move based on accumulated time instead of jumping full pixels
    if keyboard.pressed(KeyCode::ArrowUp) {
        let new_y = (transform.translation.y + move_amount).min(paddle_boundary);
        transform.translation.y = new_y;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        let new_y = (transform.translation.y - move_amount).max(-paddle_boundary);
        transform.translation.y = new_y;
    }
} 
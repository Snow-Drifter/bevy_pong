/// Module for handling the paddles that players control to hit the ball
use bevy::prelude::*;
use crate::window::{HEIGHT, WIDTH};

/// Height of the paddle sprite in pixels
pub const PADDLE_HEIGHT: f32 = 28.0;
/// Width of the paddle sprite in pixels
pub const PADDLE_WIDTH: f32 = 9.0;
/// Movement speed of the paddle in pixels per second
pub const PADDLE_SPEED: f32 = 500.0;
/// Distance from the edge of the screen in pixels
pub const PADDLE_OFFSET: f32 = 40.0;

/// Component that marks an entity as the left paddle
#[derive(Component)]
pub struct LeftPaddle;

/// Component that marks an entity as the right paddle
#[derive(Component)]
pub struct RightPaddle;

/// Spawns the left paddle at the starting position
///
/// Creates a white rectangular sprite with the specified dimensions
/// positioned near the left edge of the screen. Marks it with the
/// LeftPaddle component to identify it for movement and collision systems.
pub fn spawn_left_paddle(mut commands: Commands) {
    let half_width = WIDTH as f32 / 2.0;
    
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

/// Spawns the right paddle at the starting position
///
/// Creates a white rectangular sprite with the specified dimensions
/// positioned near the right edge of the screen. Marks it with the
/// RightPaddle component to identify it for movement and collision systems.
pub fn spawn_right_paddle(mut commands: Commands) {
    let half_width = WIDTH as f32 / 2.0;
    
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

/// Handles movement of the left paddle using W and S keys
///
/// Updates the left paddle's position based on keyboard input.
/// W key moves the paddle up, S key moves it down.
/// Prevents the paddle from moving beyond the screen boundaries.
pub fn move_left_paddle(
    mut query: Query<&mut Transform, With<LeftPaddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_boundary = (HEIGHT as f32 - PADDLE_HEIGHT) / 2.0;
    
    let mut transform = query.single_mut();
    let move_amount = PADDLE_SPEED * time.delta_secs();
    
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.y = (transform.translation.y + move_amount).min(paddle_boundary);
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.y = (transform.translation.y - move_amount).max(-paddle_boundary);
    }
}

/// Handles movement of the right paddle using arrow keys
///
/// Updates the right paddle's position based on keyboard input.
/// Up arrow key moves the paddle up, Down arrow key moves it down.
/// Prevents the paddle from moving beyond the screen boundaries.
pub fn move_right_paddle(
    mut query: Query<&mut Transform, With<RightPaddle>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_boundary = (HEIGHT as f32 - PADDLE_HEIGHT) / 2.0;
    
    let mut transform = query.single_mut();
    let move_amount = PADDLE_SPEED * time.delta_secs();
    
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.y = (transform.translation.y + move_amount).min(paddle_boundary);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.y = (transform.translation.y - move_amount).max(-paddle_boundary);
    }
} 
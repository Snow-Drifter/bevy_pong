use bevy::prelude::*;
use crate::paddle::{LeftPaddle, RightPaddle};
use crate::scoreboard::ScoreBoard;
use crate::pixel_grid::{PIXEL_SIZE, get_half_screen_size};

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct BounceCount(u32);

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)), // 1x1 pixel ball
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        Ball,
        Velocity {
            x: PIXEL_SIZE * 5.0, // Move 5 pixels per second
            y: PIXEL_SIZE * 5.0,
        },
        BounceCount(0),
    ));
}

pub fn update_ball(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut BounceCount), With<Ball>>,
    left_paddle_query: Query<&Transform, (With<LeftPaddle>, Without<Ball>)>,
    right_paddle_query: Query<&Transform, (With<RightPaddle>, Without<Ball>)>,
    time: Res<Time>,
    mut score_board: ResMut<ScoreBoard>,
) {
    let (half_width, half_height) = get_half_screen_size();
    
    let left_paddle = left_paddle_query.single();
    let right_paddle = right_paddle_query.single();
    
    // Get paddle sizes
    let paddle_width = PIXEL_SIZE;
    let paddle_height = PIXEL_SIZE * 4.0;
    let ball_size = PIXEL_SIZE;
    
    for (mut transform, mut velocity, mut bounce_count) in ball_query.iter_mut() {
        let speed_multiplier = match bounce_count.0 {
            0..=3 => {
                Vec2::new(1.0, 1.0)
            },
            4..=11 => {
                Vec2::new(1.6, 1.0)
            },
            _ => {
                Vec2::new(2.1, 1.0)
            }
        };

        // Calculate movement for this frame
        let delta = time.delta_secs();
        let movement = Vec2::new(velocity.x * delta * speed_multiplier.x, velocity.y * delta * speed_multiplier.y);
        
        // Apply movement
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // Collision detection with left paddle
        if transform.translation.x - ball_size/2.0 <= left_paddle.translation.x + paddle_width/2.0 && 
           transform.translation.x + ball_size/2.0 >= left_paddle.translation.x - paddle_width/2.0 &&
           transform.translation.y + ball_size/2.0 >= left_paddle.translation.y - paddle_height/2.0 &&
           transform.translation.y - ball_size/2.0 <= left_paddle.translation.y + paddle_height/2.0 &&
           velocity.x < 0.0
        {
            // Ball hit the left paddle, reverse x direction
            velocity.x = -velocity.x;
            // Push the ball outside the paddle to prevent sticking
            transform.translation.x = left_paddle.translation.x + paddle_width/2.0 + ball_size/2.0;
            bounce_count.0 += 1;
        }
        // Collision detection with right paddle
        else if transform.translation.x + ball_size/2.0 >= right_paddle.translation.x - paddle_width/2.0 &&
                transform.translation.x - ball_size/2.0 <= right_paddle.translation.x + paddle_width/2.0 &&
                transform.translation.y + ball_size/2.0 >= right_paddle.translation.y - paddle_height/2.0 &&
                transform.translation.y - ball_size/2.0 <= right_paddle.translation.y + paddle_height/2.0 &&
                velocity.x > 0.0
        {
            // Ball hit the right paddle, reverse x direction
            velocity.x = -velocity.x;
            // Push the ball outside the paddle to prevent sticking
            transform.translation.x = right_paddle.translation.x - paddle_width/2.0 - ball_size/2.0;
            bounce_count.0 += 1;
        }
        
        // Bounce off the top and bottom edges
        if transform.translation.y > half_height - ball_size/2.0 {
            velocity.y = -velocity.y.abs(); // Ensure negative
            transform.translation.y = half_height - ball_size/2.0;
        } else if transform.translation.y < -half_height + ball_size/2.0 {
            velocity.y = velocity.y.abs(); // Ensure positive
            transform.translation.y = -half_height + ball_size/2.0;
        }
        
        // Scoring logic for left/right sides
        if transform.translation.x > half_width {
            // Left player scores
            score_board.left += 1;
            info!("Score: Left {} - Right {}", score_board.left, score_board.right);
            
            // Reset ball position
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            
            // Send toward right player with slight y variation
            velocity.x = -PIXEL_SIZE * 5.0;
            velocity.y = PIXEL_SIZE * 5.0 * if (score_board.left + score_board.right) % 2 == 0 { 1.0 } else { -1.0 };
            bounce_count.0 = 0;
        } else if transform.translation.x < -half_width {
            // Right player scores
            score_board.right += 1;
            info!("Score: Left {} - Right {}", score_board.left, score_board.right);
            
            // Reset ball position
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            
            // Send toward left player with slight y variation
            velocity.x = PIXEL_SIZE * 5.0;
            velocity.y = PIXEL_SIZE * 5.0 * if (score_board.left + score_board.right) % 2 == 0 { 1.0 } else { -1.0 };
            bounce_count.0 = 0;
        }
    }
} 
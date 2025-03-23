use bevy::prelude::*;
use crate::paddle::{LeftPaddle, RightPaddle, PADDLE_WIDTH};
use crate::scoreboard::ScoreEvent;
use crate::pixel_grid::get_half_screen_size;
use rand::Rng;

pub const BALL_HEIGHT: f32 = PADDLE_WIDTH;
pub const BALL_WIDTH: f32 = PADDLE_WIDTH;
pub const INITIAL_BALL_SPEED: f32 = 125.0;

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
            custom_size: Some(Vec2::new(BALL_WIDTH, BALL_HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        Ball,
        Velocity {
            x: INITIAL_BALL_SPEED,
            y: INITIAL_BALL_SPEED,
        },
        BounceCount(0),
    ));
}

pub fn update_ball(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut BounceCount, &Sprite), With<Ball>>,
    left_paddle_query: Query<(&Transform, &Sprite), (With<LeftPaddle>, Without<Ball>)>,
    right_paddle_query: Query<(&Transform, &Sprite), (With<RightPaddle>, Without<Ball>)>,
    time: Res<Time>,
    mut score_event_writer: EventWriter<ScoreEvent>,
) {
    // Early return if paddles don't exist yet
    let Ok((left_paddle, left_paddle_sprite)) = left_paddle_query.get_single() else {
        return;
    };
    let Ok((right_paddle, right_paddle_sprite)) = right_paddle_query.get_single() else {
        return;
    };

    let (half_width, half_height) = get_half_screen_size();
    
    for (mut transform, mut velocity, mut bounce_count, ball_sprite) in ball_query.iter_mut() {
        let ball_size = ball_sprite.custom_size.unwrap();
        let left_paddle_size = left_paddle_sprite.custom_size.unwrap();
        let right_paddle_size = right_paddle_sprite.custom_size.unwrap();
        
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
        if transform.translation.x - ball_size.x/2.0 <= left_paddle.translation.x + left_paddle_size.x/2.0 && 
           transform.translation.x + ball_size.x/2.0 >= left_paddle.translation.x - left_paddle_size.x/2.0 &&
           transform.translation.y + ball_size.y/2.0 >= left_paddle.translation.y - left_paddle_size.y/2.0 &&
           transform.translation.y - ball_size.y/2.0 <= left_paddle.translation.y + left_paddle_size.y/2.0 &&
           velocity.x < 0.0
        {
            // Ball hit the left paddle, reverse x direction
            velocity.x = -velocity.x;
            // Push the ball outside the paddle to prevent sticking
            transform.translation.x = left_paddle.translation.x + left_paddle_size.x/2.0 + ball_size.x/2.0;
            bounce_count.0 += 1;
        }
        // Collision detection with right paddle
        else if transform.translation.x + ball_size.x/2.0 >= right_paddle.translation.x - right_paddle_size.x/2.0 &&
                transform.translation.x - ball_size.x/2.0 <= right_paddle.translation.x + right_paddle_size.x/2.0 &&
                transform.translation.y + ball_size.y/2.0 >= right_paddle.translation.y - right_paddle_size.y/2.0 &&
                transform.translation.y - ball_size.y/2.0 <= right_paddle.translation.y + right_paddle_size.y/2.0 &&
                velocity.x > 0.0
        {
            // Ball hit the right paddle, reverse x direction
            velocity.x = -velocity.x;
            // Push the ball outside the paddle to prevent sticking
            transform.translation.x = right_paddle.translation.x - right_paddle_size.x/2.0 - ball_size.x/2.0;
            bounce_count.0 += 1;
        }
        
        // Bounce off the top and bottom edges
        if transform.translation.y > half_height - ball_size.y/2.0 {
            velocity.y = -velocity.y.abs(); // Ensure negative
            transform.translation.y = half_height - ball_size.y/2.0;
        } else if transform.translation.y < -half_height + ball_size.y/2.0 {
            velocity.y = velocity.y.abs(); // Ensure positive
            transform.translation.y = -half_height + ball_size.y/2.0;
        }
        
        // Scoring logic for left/right sides
        if transform.translation.x > half_width {
            // Left player scores
            score_event_writer.send(ScoreEvent::LeftScored);
        } else if transform.translation.x < -half_width {
            // Right player scores
            score_event_writer.send(ScoreEvent::RightScored);
        }
    }
}

pub fn reset_ball_system(
    mut score_events: EventReader<ScoreEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut BounceCount), With<Ball>>,
) {
    for event in score_events.read() {
        if let Ok((mut transform, mut velocity, mut bounce_count)) = ball_query.get_single_mut() {
            // Reset ball position
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            
            // Reset bounce count
            bounce_count.0 = 0;
            
            // Determine direction based on who scored
            let send_right = match event {
                ScoreEvent::LeftScored => false, // Left scored, send to right
                ScoreEvent::RightScored => true, // Right scored, send to left
            };
            
            let direction = if send_right { 1.0 } else { -1.0 };
            
            // Random up or down direction
            let y_direction = if rand::rng().random_bool(0.5) { 1.0 } else { -1.0 };
            
            // Set velocity
            velocity.x = INITIAL_BALL_SPEED * direction;
            velocity.y = INITIAL_BALL_SPEED * y_direction;
        }
    }
} 
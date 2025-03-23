use bevy::prelude::*;
use crate::paddle::{LeftPaddle, RightPaddle, PADDLE_WIDTH};
use crate::scoreboard::ScoreEvent;
use crate::window::{WIDTH, HEIGHT};
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

// Tracks ball bounces to control speed progression
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

/// Check if two rectangles are colliding
fn is_colliding(
    ball_pos: Vec3,
    ball_size: Vec2,
    paddle_pos: Vec3,
    paddle_size: Vec2,
) -> bool {
    ball_pos.x - ball_size.x/2.0 <= paddle_pos.x + paddle_size.x/2.0 && 
    ball_pos.x + ball_size.x/2.0 >= paddle_pos.x - paddle_size.x/2.0 &&
    ball_pos.y + ball_size.y/2.0 >= paddle_pos.y - paddle_size.y/2.0 &&
    ball_pos.y - ball_size.y/2.0 <= paddle_pos.y + paddle_size.y/2.0
}

/// Get speed multiplier based on bounce count
fn get_speed_multiplier(bounce_count: u32) -> Vec2 {
    match bounce_count {
        0..=3 => Vec2::new(1.0, 1.0),
        4..=11 => Vec2::new(1.6, 1.0),
        _ => Vec2::new(2.1, 1.0)
    }
}

// Main ball update system - handles movement, collisions and scoring
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

    let half_width = WIDTH as f32 / 2.0;
    let half_height = HEIGHT as f32 / 2.0;
    
    for (mut transform, mut velocity, mut bounce_count, ball_sprite) in ball_query.iter_mut() {
        let ball_size = ball_sprite.custom_size.unwrap();
        let left_paddle_size = left_paddle_sprite.custom_size.unwrap();
        let right_paddle_size = right_paddle_sprite.custom_size.unwrap();
        
        // Speed increases after multiple bounces
        let speed_multiplier = get_speed_multiplier(bounce_count.0);

        // Calculate and apply movement
        let delta = time.delta_secs();
        let movement = Vec2::new(velocity.x * delta * speed_multiplier.x, velocity.y * delta * speed_multiplier.y);
        
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        handle_paddle_collisions(
            &mut transform, 
            &mut velocity, 
            &mut bounce_count,
            ball_size,
            left_paddle.translation, 
            left_paddle_size,
            right_paddle.translation, 
            right_paddle_size
        );
        
        handle_wall_collisions(&mut transform, &mut velocity, ball_size, half_height);
        
        check_for_scoring(&transform, half_width, &mut score_event_writer);
    }
}

fn handle_paddle_collisions(
    transform: &mut Transform,
    velocity: &mut Velocity,
    bounce_count: &mut BounceCount,
    ball_size: Vec2,
    left_paddle_pos: Vec3,
    left_paddle_size: Vec2,
    right_paddle_pos: Vec3,
    right_paddle_size: Vec2,
) {
    // Check left paddle collision
    if is_colliding(transform.translation, ball_size, left_paddle_pos, left_paddle_size) && velocity.x < 0.0 {
        velocity.x = -velocity.x;
        // Push the ball outside the paddle to prevent sticking
        transform.translation.x = left_paddle_pos.x + left_paddle_size.x/2.0 + ball_size.x/2.0;
        bounce_count.0 += 1;
    }
    // Check right paddle collision
    else if is_colliding(transform.translation, ball_size, right_paddle_pos, right_paddle_size) && velocity.x > 0.0 {
        velocity.x = -velocity.x;
        // Push the ball outside the paddle to prevent sticking
        transform.translation.x = right_paddle_pos.x - right_paddle_size.x/2.0 - ball_size.x/2.0;
        bounce_count.0 += 1;
    }
}

fn handle_wall_collisions(
    transform: &mut Transform,
    velocity: &mut Velocity,
    ball_size: Vec2,
    half_height: f32
) {
    // Bounce off the top wall
    if transform.translation.y > half_height - ball_size.y/2.0 {
        velocity.y = -velocity.y.abs(); // Ensure negative
        transform.translation.y = half_height - ball_size.y/2.0;
    } 
    // Bounce off the bottom wall
    else if transform.translation.y < -half_height + ball_size.y/2.0 {
        velocity.y = velocity.y.abs(); // Ensure positive
        transform.translation.y = -half_height + ball_size.y/2.0;
    }
}

// Checks if ball went past paddles and sends appropriate score events
fn check_for_scoring(
    transform: &Transform, 
    half_width: f32,
    score_event_writer: &mut EventWriter<ScoreEvent>
) {
    // Right side (left player scores)
    if transform.translation.x > half_width {
        score_event_writer.send(ScoreEvent::LeftScored);
    } 
    // Left side (right player scores)
    else if transform.translation.x < -half_width {
        score_event_writer.send(ScoreEvent::RightScored);
    }
}

// Resets ball position and sets velocity after scoring
pub fn reset_ball_system(
    mut score_events: EventReader<ScoreEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity, &mut BounceCount), With<Ball>>,
) {
    for event in score_events.read() {
        if let Ok((mut transform, mut velocity, mut bounce_count)) = ball_query.get_single_mut() {
            // Reset ball position and bounce count
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
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
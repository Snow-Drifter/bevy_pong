/// Module for handling the game's scoreboard functionality
use bevy::prelude::*;
use crate::window::{WIDTH, HEIGHT};

/// Tracks the score for both the left and right players
#[derive(Resource, Default)]
pub struct ScoreBoard {
    /// Score for the left player
    pub left: u32,
    /// Score for the right player
    pub right: u32,
}

/// Events triggered when a player scores
#[derive(Event)]
pub enum ScoreEvent {
    /// Event when left player scores
    LeftScored,
    /// Event when right player scores
    RightScored,
}

/// Component for score text UI elements
#[derive(Component)]
pub struct ScoreText {
    /// Whether this text represents the left player's score (false for right player)
    pub is_left: bool,
}

/// Updates the scoreboard resource when scoring events occur
///
/// Processes ScoreEvent events and increments the appropriate player's score
/// in the ScoreBoard resource. Logs the updated score to the console.
pub fn update_scoreboard(
    mut score_events: EventReader<ScoreEvent>,
    mut scoreboard: ResMut<ScoreBoard>,
) {
    for event in score_events.read() {
        match event {
            ScoreEvent::LeftScored => {
                scoreboard.left += 1;
                info!("Score: Left {} - Right {}", scoreboard.left, scoreboard.right);
            }
            ScoreEvent::RightScored => {
                scoreboard.right += 1;
                info!("Score: Left {} - Right {}", scoreboard.left, scoreboard.right);
            }
        }
    }
}

/// Spawns the scoreboard UI elements for displaying player scores
///
/// Creates two Text2d entities positioned at the top of the screen that will display
/// the current score for each player. Each text entity starts with a value of "0".
pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let half_width = WIDTH as f32 / 2.0;
    let half_height = HEIGHT as f32 / 2.0;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    // Left score text
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(-half_width / 2.0, half_height - 60.0, 1.0)),
        ScoreText { is_left: true },
    ));

    // Right score text
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(Vec3::new(half_width / 2.0, half_height - 60.0, 1.0)),
        ScoreText { is_left: false },
    ));
}

/// Updates the score text UI components with the current score values
///
/// Queries all Text2d components with the ScoreText marker, and updates their
/// displayed text to match the current values in the ScoreBoard resource.
pub fn update_scoreboard_text(
    scoreboard: Res<ScoreBoard>,
    mut query: Query<(&mut Text2d, &ScoreText)>,
) {
    for (mut text, score_text) in query.iter_mut() {
        if score_text.is_left {
            text.0 = scoreboard.left.to_string();
        } else {
            text.0 = scoreboard.right.to_string();
        }
    }
} 
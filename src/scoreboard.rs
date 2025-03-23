use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ScoreBoard {
    pub left: u32,
    pub right: u32,
}

#[derive(Event)]
pub enum ScoreEvent {
    LeftScored,
    RightScored,
}

#[derive(Component)]
pub struct ScoreText {
    pub is_left: bool,
}

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

pub fn spawn_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (half_width, half_height) = crate::pixel_grid::get_half_screen_size();
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

// Update the score text with the current score values
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
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
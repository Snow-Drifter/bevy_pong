use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ScoreBoard {
    pub left: u32,
    pub right: u32,
} 
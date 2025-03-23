use bevy::prelude::*;
use crate::{paddle::PADDLE_WIDTH, window::{WIDTH, HEIGHT}};

pub const MIDDLE_LINE_WIDTH: f32 = PADDLE_WIDTH;

pub fn spawn_background(mut commands: Commands) {
    let half_width = WIDTH as f32 / 2.0;
    let half_height = HEIGHT as f32 / 2.0;

    // Create grid border
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(half_width * 2.0, half_height * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
    ));

    // Create center line
    let center_pixel_size = MIDDLE_LINE_WIDTH;
    let center_pixel_height = center_pixel_size * 2.0;
    let center_offset = 0.0;

    for y in -((HEIGHT / 2) as i32)..(HEIGHT / 2) as i32 {
        if y % 2 == 0 {
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.3, 0.3, 0.3),
                    custom_size: Some(Vec2::new(center_pixel_size, center_pixel_height)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    0.0,
                    (y as f32 * center_pixel_height) + center_offset,
                    -0.05,
                )),
            ));
        }
    }
} 
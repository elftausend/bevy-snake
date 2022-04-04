use bevy::{prelude::{Commands, Transform, Color}, sprite::{SpriteBundle, Sprite}, math::Vec3};

use crate::Snake;


pub fn spawn_snake(commands: &mut Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0., -30., 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Snake {
            x_direction: 1.,
            y_direction: 0.,
            body: Vec::new()
        });
}
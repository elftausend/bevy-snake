use bevy::{prelude::{Commands, Transform, Component, Color}, sprite::{SpriteBundle, Sprite}, math::Vec3};

#[derive(Component)]
pub struct Wall;

pub fn spawn_wall(commands: &mut Commands, scale: Vec3, translation: Vec3, wall_color: Color) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Wall);
}
use bevy::{prelude::{Commands, Transform, Color}, sprite::{SpriteBundle, Sprite}, math::Vec3};
use rand::Rng;

use crate::Food;

pub fn rand_food() -> (i32, i32) {
    let mut thread_rng = rand::thread_rng();
    let x = thread_rng.gen_range(-540..=510); //540
    let x = (x / 30) * 30;

    let y = thread_rng.gen_range(-360..=330); //360
    let y = (y / 30) * 30;
    (x.min(510), y.min(330))
}

pub fn spawn_food(commands: &mut Commands) {
    let (x, y) = rand_food();
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(x as f32, y as f32, 0.0),
                //translation: Vec3::new(500., 0., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.8, 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food);
}
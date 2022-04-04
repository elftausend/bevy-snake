use bevy::{prelude::{Commands, Transform, Color, Query, With, EventWriter, Entity, Component}, sprite::{SpriteBundle, Sprite}, math::Vec3};
use rand::Rng;

use crate::{BodySegment, Snake, GrowSnake};


#[derive(Component)]
pub struct Food;

pub fn rand_food() -> (i32, i32) {
    let mut thread_rng = rand::thread_rng();
    let x = thread_rng.gen_range(-470..=470); //540
    let x = (x / 30) * 30;

    let y = thread_rng.gen_range(-320..=320); //360
    let y = (y / 30) * 30;
    (x, y)
}

pub fn spawn_food(commands: &mut Commands, segment_query: Query<&Transform, With<BodySegment>>) {
    let (mut x, mut y) = rand_food();
    let mut gen = false;
    loop {
        for segment in segment_query.iter() {
            let trans = segment.translation;
            if trans.x == x as f32 && trans.y == y as f32 {
                gen = true;
            }
        }
        if !gen {
            break;
        } else {
            (x, y) = rand_food();
            gen = false;
        }
    }
        
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


pub fn check_food_system(mut event: EventWriter<GrowSnake>, mut commands: Commands, mut food_query: Query<(Entity, &Transform), With<Food>>, mut snake_query: Query<&Transform, With<Snake>>, segment_query: Query<&Transform, With<BodySegment>>) {
    let (ent, food_trans) = food_query.single_mut();
    let snake_trans = snake_query.single_mut();

    let trans_food = &food_trans.translation;
    let trans_snake = &snake_trans.translation;

    if trans_food.x == trans_snake.x && trans_food.y == trans_snake.y {
        commands.entity(ent).despawn();
        spawn_food(&mut commands, segment_query);
        event.send(GrowSnake);
    }
}
use bevy::{prelude::{Commands, Transform, Color, Component, ResMut, EventWriter, Query, With, Res, KeyCode, EventReader, Entity, Without}, sprite::{SpriteBundle, Sprite}, math::Vec3, input::Input};

use crate::{Counter, LostEvent, GrowSnake, wall::Wall};

#[derive(Component)]
pub struct BodySegment {
    pub num: usize,
}

#[derive(Component)]
pub struct Snake {
    pub x_direction: f32,
    pub y_direction: f32,
}

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
        });
}

pub fn spawn_segment(commands: &mut Commands, counter: &mut ResMut<Counter>, x: f32, y: f32) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(x, y, 0.0),
                //translation: Vec3::new(500., 0., 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.9, 0.9, 0.9),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BodySegment {num: counter.num } );
    counter.num += 1;   
}


pub fn grow_snake_system(mut commands: Commands, mut counter: ResMut<Counter>, mut event: EventReader<GrowSnake>, mut snake_query: Query<&Transform, With<Snake>>) {
    if let Some(_) = event.iter().next() {
        let snake_trans = snake_query.single_mut();
        
        let translation = snake_trans.translation;
        spawn_segment(&mut commands, &mut counter, translation.x, translation.y);
        
        println!("grow");
    }
}

pub fn snake_move_system(mut commands: Commands, mut event: EventWriter<LostEvent>, mut counter: ResMut<Counter>, 
    mut snake_query: Query<(&Snake, With<Snake>, &mut Transform)>, 
    segments: Query<(&BodySegment, Entity, &Transform), (With<BodySegment>, Without<Snake>)>, 
    wall_query: Query<&Transform, (Without<BodySegment>, Without<Snake>, With<Wall>)>) 
{
    let (snake, _, mut transform) = snake_query.single_mut();

    let snake_scale = transform.scale.truncate();

    let mut mv = true;

    for wall in wall_query.iter() {
        let mut a_trans = transform.translation.truncate();
        if snake.x_direction == 1. {
            a_trans.x += 30.;
        } 
        if snake.x_direction == -1. {
            a_trans.x -= 30.;
        }
        if snake.y_direction == 1. {
            a_trans.y += 30.;
        } 
        if snake.y_direction == -1. {
            a_trans.y -= 30.;
        }
        
        let a_min = (a_trans) - snake_scale / 2.0;
        let a_max = (a_trans) + snake_scale / 2.0;

        let b_trans = wall.translation.truncate();

        let b_min = b_trans - wall.scale.truncate() / 2.0;
        let b_max = b_trans + wall.scale.truncate() / 2.0;
        
        if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
            event.send(LostEvent);
            mv = false;
        }
    }
    if mv {
        
        let translation = &mut transform.translation;
        translation.x += snake.x_direction * 30.;
        translation.y += snake.y_direction * 30.;
    
        let x = translation.x;
        let y = translation.y;
        
        spawn_segment(&mut commands, &mut counter, x, y);
        let mut segments2 = vec![];
        
        for segment in segments.iter() {
            let trans_seg = segment.2.translation;
            if trans_seg.x == translation.x && trans_seg.y == translation.y {
                event.send(LostEvent);
                return;
            }
            segments2.push((segment.0, segment.1))
        }
    
        segments2.sort_by(|a, b| a.0.num.cmp(&b.0.num));
        let a = segments2[0];
        commands.entity(a.1).despawn();
    }
    
}

pub fn snake_change_direction_system(mut snake_query: Query<&mut Snake, With<Snake>>, keyboard_input: Res<Input<KeyCode>>) {
    let mut snake = snake_query.single_mut();

    if snake.y_direction != 0. {
        if keyboard_input.just_pressed(KeyCode::A) {
            snake.x_direction = -1.;
            snake.y_direction = 0.;
        }
    
        if keyboard_input.just_pressed(KeyCode::D) {
            snake.x_direction = 1.;
            snake.y_direction = 0.;            
        }
    }
    
    if snake.x_direction != 0. {
        if keyboard_input.just_pressed(KeyCode::S) {
            snake.y_direction = -1.;
            snake.x_direction = 0.;
        }
    
        if keyboard_input.just_pressed(KeyCode::W) {
            snake.y_direction = 1.;
            snake.x_direction = 0.;
        }  
    }
}
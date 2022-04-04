mod food;
mod snake;

use bevy::{prelude::{App, Commands, OrthographicCameraBundle, UiCameraBundle, SystemSet, Color, Transform, Component, Query, With, KeyCode, Res, Entity, EventWriter, EventReader, ResMut, Without}, DefaultPlugins, core_pipeline::ClearColor, core::FixedTimestep, sprite::{SpriteBundle, Sprite, collide_aabb::collide}, math::{Vec3, Vec2}, window::WindowDescriptor, input::Input, ecs::system::Command};
use food::spawn_food;
use snake::spawn_snake;

const TIME_STEP: f32 = 1. / 60.;

pub struct EatFood;
pub struct GrowSnake;
pub struct LostEvent;

pub struct Counter {
    num: usize,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 1080.,
            height: 720.,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(Counter { num: 1})
        .add_event::<GrowSnake>()
        .add_event::<LostEvent>()
        .add_startup_system(setup)
        .add_system(snake_change_direction_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(grow_snake_system)
                .with_system(check_food_system)
                .with_system(lost_game_system)
                
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(snake_move_system)
                //.with_system(ungrow_snake_system)
                
        )
        /*.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.4))
                
        )
        */
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn lost_game_system(mut event: EventReader<LostEvent>, mut commands: Commands, mut counter: ResMut<Counter>, segment_query: Query<Entity, With<BodySegment>>, snake_query: Query<Entity, With<Snake>>) {
    if event.iter().next().is_some() {
        for entity in segment_query.iter() {
            commands.entity(entity).despawn();
        }
        let snake = snake_query.single();
        commands.entity(snake).despawn();

        spawn_snake(&mut commands);

        spawn_segment(&mut commands, &mut counter, -30. as f32 as f32, -30.);
       // spawn_segment(&mut commands, &mut counter, -30. as f32*2. as f32, -30.);
        

    }
}

fn check_food_system(mut event: EventWriter<GrowSnake>, mut commands: Commands, mut food_query: Query<(With<Food>, Entity, &Transform)>, mut snake_query: Query<(With<Snake>, &Transform)>, segment_query: Query<&Transform, With<BodySegment>>) {
    let (_, ent, food_trans) = food_query.single_mut();
    let (_, snake_trans) = snake_query.single_mut();

    let trans_food = &food_trans.translation;
    let trans_snake = &snake_trans.translation;

    if trans_food.x == trans_snake.x && trans_food.y == trans_snake.y {
        commands.entity(ent).despawn();
        spawn_food(&mut commands, segment_query);
        event.send(GrowSnake);
    }
}

#[derive(Component)]
pub struct Wall;

fn setup(mut commands: Commands, mut counter: ResMut<Counter>, segment_query: Query<&Transform, With<BodySegment>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let wall_color = Color::rgb(0.8, 0., 0.8);
    let wall_thickness = 10.0;
    let bounds = Vec2::new(940.0, 640.0);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Wall);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        }).insert(Wall);




    spawn_snake(&mut commands);
    spawn_segment(&mut commands, &mut counter, -30. as f32 as f32, -30.);
    //spawn_segment(&mut commands, &mut counter, -30. as f32*2. as f32, -30.);
    //for n in 0..=5 {
    //    spawn_segment(&mut commands, &mut counter, -30. * n as f32 as f32, -30.);
    //}
    spawn_food(&mut commands, segment_query);
}

fn spawn_segment(commands: &mut Commands, counter: &mut ResMut<Counter>, x: f32, y: f32) {
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

#[derive(Component)]
pub struct Food;

fn grow_snake_system(mut commands: Commands, mut counter: ResMut<Counter>, mut event: EventReader<GrowSnake>, mut snake_query: Query<(With<Snake>, &mut Transform)>) {
    if let Some(_) = event.iter().next() {
        let (_, mut snake_trans) = snake_query.single_mut();
        
        let translation = &mut snake_trans.translation;

        let x = translation.x - (30. * counter.num as f32);
        let y = translation.y;

        spawn_segment(&mut commands, &mut counter, x, y);
        
        println!("grow");
    }
}

#[derive(Component)]
pub struct BodySegment {
    num: usize,
}

#[derive(Component)]
pub struct Snake {
    x_direction: f32,
    y_direction: f32,
    body: Vec<BodySegment>,
}


pub fn snake_move_system(mut commands: Commands, mut event: EventWriter<LostEvent>, mut counter: ResMut<Counter>, mut snake_query: Query<(&Snake, With<Snake>, &mut Transform)>, segments: Query<(&BodySegment, With<BodySegment>, Entity, Without<Snake>, &Transform)>, wall_query: Query<&Transform, (Without<BodySegment>, Without<Snake>, With<Wall>)>) {
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
            let trans_seg = segment.4.translation;
            if trans_seg.x == translation.x && trans_seg.y == translation.y {
                event.send(LostEvent)
            }
            segments2.push((segment.0, segment.2))
        }
    
        segments2.sort_by(|a, b| a.0.num.cmp(&b.0.num));
        let a = segments2[0];
        commands.entity(a.1).despawn();
    }
    
    //let seg: Vec<(BodySegment, Entity)> = segments.iter().map(|(b, _, t)| (b, t)).collect();

    /*
    let count = segments.iter().count();
    println!("count: {count}");
 
    let ent = segments.iter().last();
    
    if let Some(ent) = ent {
        commands.entity(ent.2).despawn();
    }

    let ent = segments.iter().nth(0);
    
    if let Some(ent) = ent {
        commands.entity(ent.2).despawn();
    }
    */
    //let ent = segments.iter().last();
    
    //if let Some(ent) = ent {
     //   commands.entity(ent.2).despawn();
    //}

    /*
    for (idx, ent) in entities.iter().rev().enumerate() {
        println!("{idx}");
        if idx >= 5 {
            commands.entity(*ent).despawn();
        }
    }
    */
}

pub fn snake_change_direction_system(mut snake_query: Query<(&mut Snake, With<Snake>, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>) {
    let (mut snake, _, mut transform) = snake_query.single_mut();

/*
    let translation = &mut transform.translation;

    if keyboard_input.just_pressed(KeyCode::Up) && keyboard_input.just_pressed(KeyCode::Right) {
        translation.x += 30.;
        translation.y += 30.;
    }    
*/

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

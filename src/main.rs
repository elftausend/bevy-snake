mod food;

use bevy::{prelude::{App, Commands, OrthographicCameraBundle, UiCameraBundle, SystemSet, Color, Transform, Component, Query, With, KeyCode, Res, Entity, EventWriter, EventReader, ResMut}, DefaultPlugins, core_pipeline::ClearColor, core::FixedTimestep, sprite::{SpriteBundle, Sprite}, math::Vec3, window::WindowDescriptor, input::Input, ecs::system::Command};
use food::spawn_food;

const TIME_STEP: f32 = 1. / 60.;

pub struct EatFood;
pub struct GrowSnake;

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
        .add_startup_system(setup)
        .add_system(snake_change_direction_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(grow_snake_system)
                .with_system(check_food_system)
                
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

fn check_food_system(mut event: EventWriter<GrowSnake>, mut commands: Commands, mut food_query: Query<(With<Food>, Entity, &Transform)>, mut snake_query: Query<(With<Snake>, &Transform)>) {
    let (_, ent, food_trans) = food_query.single_mut();
    let (_, snake_trans) = snake_query.single_mut();

    let trans_food = &food_trans.translation;
    let trans_snake = &snake_trans.translation;

    if trans_food.x == trans_snake.x && trans_food.y == trans_snake.y {
        commands.entity(ent).despawn();
        spawn_food(&mut commands);
        event.send(GrowSnake);
    }
}



fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

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

    for n in 0..=5 {

        spawn_segment(&mut commands, -30.*n as f32, -30.);
    }
    spawn_food(&mut commands);
}


fn spawn_segment(commands: &mut Commands, x: f32, y: f32) {
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
        .insert(BodySegment {num: 0} );
    //counter.num += 1;
}

#[derive(Component)]
pub struct Food;

fn grow_snake_system(mut commands: Commands, mut counter: ResMut<Counter>, mut event: EventReader<GrowSnake>, mut snake_query: Query<(&mut Snake, With<Snake>, &mut Transform)>) {
    if let Some(_) = event.iter().next() {
        let (mut snake, _, mut snake_trans) = snake_query.single_mut();
        
        let translation = &mut snake_trans.translation;

        let x = translation.x - (30. * counter.num as f32);
        let y = translation.y;

        spawn_segment(&mut commands, x, y);
        
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

pub fn ungrow_snake_system(mut commands: Commands, mut counter: ResMut<Counter>, segments: Query<(&BodySegment, With<BodySegment>, Entity)>) {

    
    /*
    for ent in entities.iter().rev() {
        println!("count {}, ent: {}", counter.num, ent.0.num);
        if ent.0.num > counter.num-4 {
        
            commands.entity(ent.1).despawn();
            counter.num -= 1;
        }
    }
    */
}

pub fn snake_move_system(mut commands: Commands, counter: ResMut<Counter>, mut snake_query: Query<(&Snake, With<Snake>, &mut Transform)>, segments: Query<(&BodySegment, With<BodySegment>, Entity)>) {
    let (snake, _, mut transform) = snake_query.single_mut();

    let translation = &mut transform.translation;
    translation.x += snake.x_direction * 30.;
    translation.y += snake.y_direction * 30.;

    let x = translation.x;
    let y = translation.y;

    spawn_segment(&mut commands, x, y);


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

    if keyboard_input.just_pressed(KeyCode::Left) {
        snake.x_direction = -1.;
        snake.y_direction = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        snake.x_direction = 1.;
        snake.y_direction = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        snake.y_direction = -1.;
        snake.x_direction = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        snake.y_direction = 1.;
        snake.x_direction = 0.;
    }  
}

use bevy::{prelude::{App, Commands, OrthographicCameraBundle, UiCameraBundle, SystemSet, Color, Transform, Component, Query, With, KeyCode, Res}, DefaultPlugins, core_pipeline::ClearColor, core::FixedTimestep, sprite::{SpriteBundle, Sprite}, math::Vec3, window::WindowDescriptor, input::Input};
use rand::Rng;

const TIME_STEP: f32 = 1. / 60.;

pub struct EatFood;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 1080.,
            height: 720.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_event::<EatFood>()
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(snake_change_direction_system)
                
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(snake_move_system)
                
        )
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn rand_food() -> (i32, i32) {
    let mut thread_rng = rand::thread_rng();
    let x = thread_rng.gen_range(-540..=540);
    
    let x = (x / 30) * 30;

    let y = thread_rng.gen_range(-360..=360);
    
    let y = (y / 30) * 30;
    (x, y)
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
        });

    
    
    let (x, y) = rand_food();
    println!("x: {x}, y: {y}");


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

#[derive(Component)]
pub struct Food;


pub fn spawn_food_system() {

}


#[derive(Component)]
pub struct Snake {
    x_direction: f32,
    y_direction: f32,
}

pub fn snake_move_system(mut snake_query: Query<(&Snake, With<Snake>, &mut Transform)>) {
    let (snake, _, mut transform) = snake_query.single_mut();

    let translation = &mut transform.translation;
    translation.x += snake.x_direction * 30.;
    translation.y += snake.y_direction * 30.;
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

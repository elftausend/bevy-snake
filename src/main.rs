mod food;
mod snake;
mod wall;

use bevy::{prelude::{App, Commands, OrthographicCameraBundle, UiCameraBundle, SystemSet, Color, Transform, Query, With, Entity, EventReader, ResMut}, DefaultPlugins, core_pipeline::ClearColor, core::FixedTimestep, math::{Vec3, Vec2}, window::WindowDescriptor};
use food::{spawn_food, check_food_system};
use snake::{spawn_snake, Snake, BodySegment, spawn_segment, snake_change_direction_system, grow_snake_system, snake_move_system};
use wall::spawn_wall;

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
        .add_plugins (DefaultPlugins)
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

fn setup(mut commands: Commands, mut counter: ResMut<Counter>, segment_query: Query<&Transform, With<BodySegment>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let wall_color = Color::rgb(0.8, 0., 0.8);
    let wall_thickness = 10.0;
    let bounds = Vec2::new(940.0, 640.0);

    // left
    spawn_wall(
        &mut commands, 
        Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0), 
        Vec3::new(-bounds.x / 2.0, 0.0, 0.0), 
        wall_color
    );

    // right
    spawn_wall(
        &mut commands,
        Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0), 
        Vec3::new(bounds.x / 2.0, 0.0, 0.0), 
        wall_color
    );

    // bottom
    spawn_wall(
        &mut commands,
        Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
        Vec3::new(0.0, -bounds.y / 2.0, 0.0),
        wall_color
    );

    // top
    spawn_wall(
        &mut commands, 
        Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0), 
        Vec3::new(0.0, bounds.y / 2.0, 0.0), 
        wall_color
    );

    spawn_snake(&mut commands);

    spawn_segment(&mut commands, &mut counter, 0., -30.);
    spawn_segment(&mut commands, &mut counter, 0., -30.);
    //spawn_segment(&mut commands, &mut counter, -30. as f32*2. as f32, -30.);
    //for n in 0..=5 {
    //    spawn_segment(&mut commands, &mut counter, -30. * n as f32 as f32, -30.);
    //}


    spawn_food(&mut commands, segment_query);
}


fn lost_game_system(mut event: EventReader<LostEvent>, mut commands: Commands, mut counter: ResMut<Counter>, segment_query: Query<Entity, With<BodySegment>>, snake_query: Query<Entity, With<Snake>>) {
    if event.iter().next().is_some() {
        for entity in segment_query.iter() {
            commands.entity(entity).despawn();
        }
        let snake = snake_query.single();
        commands.entity(snake).despawn();

        spawn_snake(&mut commands);

        spawn_segment(&mut commands, &mut counter, 0., -30.); 
        spawn_segment(&mut commands, &mut counter, 0., -30.); 

    }
}

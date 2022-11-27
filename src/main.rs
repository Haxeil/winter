mod math;
mod player;
mod game;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use player::player::*;
use player::{components::*, systems::*};
use game::defaults::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_player)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_gravity)
                .with_system(handle_movement)
                .with_system(move_all_physics_bodies),
        )
        .run();
}

fn add_player(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    command
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(2.0, 2.5, 0.0),
            ..default()
        })
        .insert((
            Player,
            Velocity::zero(),
            Speed(SPEED),
            Acceleration(ACCELERATION),
            Friction(FRICTION),
        ));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4500.0,
            shadows_enabled: true,
            range: 1000.0,
            radius: 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 15.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 40.5, 0.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });
}

pub fn move_all_physics_bodies(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
                // Move and slide.
        transform.translation += velocity.0 * time.delta_seconds();
        
    }
}
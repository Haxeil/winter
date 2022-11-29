mod math;
mod player;
mod game;
mod bodies;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_rapier3d::prelude::*;

use player::player::*;
use player::{components::*, systems::*};
use game::defaults::*;
use bodies::rigidbodies::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(add_player)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_gravity_to_bodies)
                .with_system(handle_movement)
                .with_system(apply_movement_to_bodies),
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
            transform: Transform::from_xyz(2.0, 10., 0.0),
            ..default()
        })
        .insert((
            Player,
            Vel::zero(),
            Speed(SPEED),
            Acceleration(ACCELERATION),
            Friction(FRICTION),
            RigidBody::KinematicPositionBased,
            Ccd::enabled(),
            Collider::capsule(Vec3::ZERO, Vec3::ZERO, 1.0),
            KinematicCharacterController { 
                up: Vec3::Z, 
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.5),
                    min_width: CharacterLength::Absolute(0.2),
                    include_dynamic_bodies: true,
                }),
                slide: true,
                ..default()
            },
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
    }).insert((
        RigidBody::Fixed,
        Collider::cuboid(16.0, 0., 16.0)
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 100.0,
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
        transform: Transform::from_xyz(-20.0, 86.0, 0.0).looking_at(Vec3::ZERO, Vec3::X),
        ..default()
    });
}


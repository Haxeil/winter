use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn zero() -> Self {
        Velocity(Vec3::ZERO)
    }
}

// Maybe add all of this into a 'physics' struct/Component.
#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Acceleration(pub f32);

#[derive(Component)]
pub struct Friction(pub f32);
//

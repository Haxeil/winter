use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub(crate) jumping: bool,
    pub(crate) grounded: bool,
}

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component, Debug)]
pub struct Vel(pub Vec3);

impl Vel {
    pub fn zero() -> Self {
        Self(Vec3::ZERO)
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

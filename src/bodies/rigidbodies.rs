use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController, Collider, CollisionEvent};

use crate::{game::defaults::GRAVITY_SCALE, player::components::Vel};


pub fn apply_gravity_to_bodies(
    mut query: Query<(&mut Vel, &mut KinematicCharacterController)>,
    time: Res<Time>,

) {
    for (velocity, mut controller) in query.iter_mut() {

        controller.translation = Some(velocity.0 * time.delta_seconds());

    }
}

pub fn apply_movement_to_bodies(mut query: Query<&mut Vel>) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec3 {x: velocity.0.x, y: -GRAVITY_SCALE, z: velocity.0.z};
    }
    
}
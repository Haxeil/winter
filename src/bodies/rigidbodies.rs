use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{game::defaults::GRAVITY_SCALE, player::components::{Vel, Player}};


pub fn apply_movement_to_bodies(
    mut query: Query<(&mut Vel, &mut KinematicCharacterController)>,
    time: Res<Time>,

) {
    for (velocity, mut controller) in query.iter_mut() {

        controller.translation = Some(velocity.0 * time.delta_seconds());

    }
}


pub fn apply_gravity_to_bodies(
    mut query: Query<(&mut Player, &mut Vel)>,
    character_controller_outputs: Query<&mut KinematicCharacterControllerOutput>
) {
    for (mut player, mut velocity) in query.iter_mut() {
        if let Ok(output) = character_controller_outputs.get_single() {
            if !output.grounded {
                velocity.0.y -= GRAVITY_SCALE;
                player.jumping = false;
                player.grounded = false;

            } else {
                if !player.jumping {
                    velocity.0.y = 0.;
                    player.grounded = true;
                }
            }
        }

    }
    
}


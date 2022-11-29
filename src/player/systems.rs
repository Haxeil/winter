use bevy::prelude::*;
use bevy_rapier3d::prelude::{Velocity, KinematicCharacterController};

#[doc(inline)]
use crate::math::math::*;

use crate::game::defaults::*;

use super::{
    components::*,
    player::{ACCELERATION, FRICTION},
};

pub fn handle_movement(
    mut query: Query<(&Speed, &mut Vel), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (speed, mut velocity) in query.iter_mut() {
        //Forward
        //Backward

        if input.pressed(KeyCode::W) && !input.pressed(KeyCode::S) {
            velocity.0.x = lerp(velocity.0.x..=(speed.0), ACCELERATION);
        } else if input.pressed(KeyCode::S) && !input.pressed(KeyCode::W) {
            velocity.0.x = lerp(velocity.0.x..=(-speed.0), ACCELERATION);
        } else {
            velocity.0.x = lerp(velocity.0.x..=0.0, FRICTION);
        }

        //Right
        //Left
        if input.pressed(KeyCode::D) && !input.pressed(KeyCode::A) {
            velocity.0.z = lerp(velocity.0.z..=(speed.0), ACCELERATION);
        } else if input.pressed(KeyCode::A) && !input.pressed(KeyCode::D) {
            velocity.0.z = lerp(velocity.0.z..=(-speed.0), ACCELERATION);
        } else {
            velocity.0.z = lerp(velocity.0.z..=0.0, FRICTION);
        }

        // to normalize diagonal velocity.0: Just a temporary solution i guess this is a bad practice !
        if input.pressed(KeyCode::W) && input.pressed(KeyCode::D)
            || input.pressed(KeyCode::W) && input.pressed(KeyCode::A)
            || input.pressed(KeyCode::S) && input.pressed(KeyCode::D)
            || input.pressed(KeyCode::S) && input.pressed(KeyCode::A)
        {
            velocity.0 = velocity.0.normalize() * speed.0;
        }
        
        let v = velocity.0.clone();

        velocity.0 += v * time.delta_seconds();
        
        

    }
}


use bevy::prelude::*;

#[doc(inline)]
use crate::math::math::*;

use crate::game::defaults::*;

use super::{
    components::*,
    player::{ACCELERATION, FRICTION},
};

pub fn handle_movement(
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
    input: Res<Input<KeyCode>>,
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

        // to normalize diagonal velocity: Just a temporary solution i guess this is a bad practice !
        if input.pressed(KeyCode::W) && input.pressed(KeyCode::D)
            || input.pressed(KeyCode::W) && input.pressed(KeyCode::A)
            || input.pressed(KeyCode::S) && input.pressed(KeyCode::D)
            || input.pressed(KeyCode::S) && input.pressed(KeyCode::A)
        {
            velocity.0 = velocity.0.normalize() * speed.0;
        }


    }
}

pub fn apply_gravity(mut query: Query<&mut Velocity, With<Player>>) {

    let mut velocity = query.get_single_mut().unwrap();
    
    velocity.0.y -= GRAVITY_SCALE;

    // TODO: Collision detection using rapier.
}
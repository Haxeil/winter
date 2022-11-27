use bevy::prelude::*;

use crate::components::{Player, Velocity, Speed};


pub fn handle_movement(mut query: Query<(&mut Velocity, &Speed), With<Player>>, input: Res<Input<KeyCode>>) { 
    
    for (mut velocity, speed) in query.iter_mut() {

        //Forward
        //Backward
        if input.pressed(KeyCode::W) && !input.pressed(KeyCode::S) {
            velocity.0.x += speed.0;

        } else if input.pressed(KeyCode::S) && !input.pressed(KeyCode::W) {
            velocity.0.x -= speed.0;
        }

        //Right
        //Left
        if input.pressed(KeyCode::D) && !input.pressed(KeyCode::A) {
            velocity.0.z += speed.0;

        } else if input.pressed(KeyCode::A) && !input.pressed(KeyCode::D) {
            velocity.0.z -= speed.0;
            println!("{}", velocity.0.z);
        }
    }

    
    
}
   


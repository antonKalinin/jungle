use bevy::prelude::*;

use super::super::components::{Block, Player, AABB};
use super::super::constants::{GRAVITY, PLAYER_HORIZONTAL_SPEED, PLAYER_INITIAL_VERTICAL_SPEED};

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<&mut Player>,
  mut block_query: Query<&Block>,
) {
  for mut player in &mut player_query.iter() {
    if keyboard_input.pressed(KeyCode::Right) {
      player.velocity.set_x(PLAYER_HORIZONTAL_SPEED);
    }

    if keyboard_input.pressed(KeyCode::Left) {
      player.velocity.set_x(-PLAYER_HORIZONTAL_SPEED);
    }

    if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::Left) {
      player.velocity.set_x(0.0);
    }

    if keyboard_input.just_released(KeyCode::Up) {
      if player.velocity.y() == 0.0 {
        player.velocity.set_y(PLAYER_INITIAL_VERTICAL_SPEED);
      }
    }

    // player is constantly affected by gravity
    *player.velocity.y_mut() -= GRAVITY * time.delta_seconds;

    *player.position.x_mut() += player.velocity.x();
    *player.position.y_mut() += player.velocity.y();

    for object in &mut block_query.iter() {
      if player.collides(object) {
        let collision = player.collision_by_axis(object);
        let collision_sign_x = collision.x().signum();
        let collision_sign_y = collision.y().signum();
        let velocity_sign_y = player.velocity.y().signum();

        if collision.x().abs() > collision.y().abs() && collision_sign_y == velocity_sign_y {
          *player.position.y_mut() = object.position.y()
            - collision_sign_y * (object.size.y() / 2.0 + player.size.y() / 2.0);

          player.velocity.set_y(0.0);
        } else {
          *player.position.x_mut() = object.position.x()
            - collision_sign_x * (object.size.x() / 2.0 + player.size.x() / 2.0);

          player.velocity.set_x(0.0);
        }
      }
    }
  }
}

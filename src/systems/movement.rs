use bevy::prelude::*;

use super::super::components::{Block, Hook, Player};
use super::super::constants::{GRAVITY, PLAYER_HORIZONTAL_SPEED, PLAYER_INITIAL_VERTICAL_SPEED};
use super::super::utils::collide_aabb;

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
  mut block_query: Query<(&Block, &Transform)>,
  mut hook_query: Query<(&Hook, &Transform)>,
) {
  for (mut player, mut player_transform) in &mut player_query.iter() {
    if keyboard_input.pressed(KeyCode::Right) {
      player.velocity.set_x(PLAYER_HORIZONTAL_SPEED);
      player_transform.rotation = Quat::from_rotation_y(0.0);
    }

    if keyboard_input.pressed(KeyCode::Left) {
      player.velocity.set_x(-PLAYER_HORIZONTAL_SPEED);
      player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }

    if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::Left) {
      player.velocity.set_x(0.0);
    }

    if keyboard_input.pressed(KeyCode::Up) {
      if player.velocity.y() == 0.0 {
        player.velocity.set_y(PLAYER_INITIAL_VERTICAL_SPEED);
      }
    }

    // player is constantly affected by gravity
    if !player.is_grabbing {
      *player.velocity.y_mut() -= GRAVITY * time.delta_seconds;
    }

    let mut player_next_translation = player_transform.translation + player.velocity;

    for (block, block_transform) in &mut block_query.iter() {
      let block_translation = block_transform.translation;
      let collision = collide_aabb(
        player_next_translation,
        player.size,
        block_translation,
        block.size,
      );

      if let Some(collision) = collision {
        let collision_sign_y = collision.y().signum();
        let velocity_sign_y = player.velocity.y().signum();

        if collision.x().abs() > collision.y().abs() && collision_sign_y == velocity_sign_y {
          *player_next_translation.y_mut() -= collision.y();

          player.velocity.set_y(0.0);
        } else {
          *player_next_translation.x_mut() -= collision.x();

          player.velocity.set_x(0.0);
        }
      }
    }

    for (hook, hook_transform) in &mut hook_query.iter() {
      let hook_translation = hook_transform.translation;
      let player_translation = player_transform.translation;
      let collision = collide_aabb(player_translation, player.size, hook_translation, hook.size);

      if let Some(_collision) = collision {
        if (player_translation.y() - hook_translation.y()).abs() < 8.0 && player.velocity.y() < 0.0
        {
          player.is_grabbing = true;
          player.velocity.set_y(0.0);
          *player_next_translation.y_mut() =
            hook_translation.y() + hook.size.y() / 2.0 - player.size.y() / 2.0;
        }
      }
    }

    if player.velocity.y() > 0.0 || player.velocity.x().abs() > 0.0 {
      player.is_grabbing = false;
    }

    player_transform.translation = player_next_translation;
  }
}

use bevy::prelude::*;

use super::super::components::{Block, Player};
use super::super::constants::{GRAVITY, PLAYER_HORIZONTAL_SPEED, PLAYER_INITIAL_VERTICAL_SPEED};

type Collision = Option<Vec2>;

fn collide_aabb(a_position: Vec3, a_size: Vec2, b_position: Vec3, b_size: Vec2) -> Collision {
  let (a_x, a_y, _) = <(f32, f32, f32)>::from(a_position);
  let (a_w, a_h) = <(f32, f32)>::from(a_size);
  let (b_x, b_y, _) = <(f32, f32, f32)>::from(b_position);
  let (b_size_x, b_size_y) = <(f32, f32)>::from(b_size);

  if (a_x - b_x).abs() < (a_w / 2.0 + b_size_x / 2.0)
    && (a_y - b_y).abs() < (a_h / 2.0 + b_size_y / 2.0)
  {
    let h = (a_x - b_x).signum() * ((a_x - b_x).abs() - (a_w / 2.0 + b_size_x / 2.0));
    let v = (a_y - b_y).signum() * ((a_y - b_y).abs() - (a_h / 2.0 + b_size_y / 2.0));

    return Some(Vec2::new(h, v));
  }

  None
}

pub fn movement(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
  mut block_query: Query<(&Block, &Transform)>,
) {
  for (mut player, mut player_transform) in &mut player_query.iter() {
    if keyboard_input.pressed(KeyCode::Right) {
      player.velocity.set_x(PLAYER_HORIZONTAL_SPEED);
      player_transform.set_rotation(Quat::from_rotation_y(0.0));
    }

    if keyboard_input.pressed(KeyCode::Left) {
      player.velocity.set_x(-PLAYER_HORIZONTAL_SPEED);
      player_transform.set_rotation(Quat::from_rotation_y(std::f32::consts::PI));
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

    let mut next_player_position = player_transform.translation() + player.velocity;

    for (block, block_transform) in &mut block_query.iter() {
      let block_translate = block_transform.translation();
      let collision = collide_aabb(
        next_player_position,
        player.size,
        block_translate,
        block.size,
      );

      if let Some(collision) = collision {
        let collision_sign_y = collision.y().signum();
        let velocity_sign_y = player.velocity.y().signum();

        if collision.x().abs() > collision.y().abs() && collision_sign_y == velocity_sign_y {
          *next_player_position.y_mut() -= collision.y();

          player.velocity.set_y(0.0);
        } else {
          *next_player_position.x_mut() -= collision.x();

          player.velocity.set_x(0.0);
        }
      }
    }

    player_transform.set_translation(next_player_position);
  }
}

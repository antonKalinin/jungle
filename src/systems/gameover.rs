use bevy::prelude::*;

use super::super::components::Player;
use super::super::resources::{Game, GameState};

pub fn gameover(
  mut game_state: ResMut<GameState>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
) {
  for (mut player, mut player_transform) in player_query.iter_mut() {
    if player.velocity.y < -50.0 {
      player.velocity = Vec3::new(0.0, 0.0, 0.0);
      player_transform.translation = player.initial_position;
      player_transform.rotation = Quat::from_rotation_y(0.0);
    }
  }
}

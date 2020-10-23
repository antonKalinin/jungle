use bevy::prelude::*;

use super::super::components::Player;
use super::super::resources::GameState;

pub fn gameover(
  mut game_state: ResMut<GameState>,
  mut player_query: Query<(&mut Player, &mut Transform)>,
) {
  for (mut player, mut player_transform) in &mut player_query.iter() {
    if player.velocity.y() < -50.0 {
      game_state.game_over = true;

      player.velocity = Vec3::new(0.0, 0.0, 0.0);
      player_transform.set_translation(player.initial_position);
      player_transform.set_rotation(Quat::from_rotation_y(0.0));
    }
  }
}

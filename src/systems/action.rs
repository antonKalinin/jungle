use bevy::prelude::*;

use super::super::components::{Coin, Player};
use super::super::resources::GameState;
use super::super::utils::collide_aabb;

pub fn action(
  mut commands: Commands,
  mut state: ResMut<GameState>,
  mut player_query: Query<(&Player, &Transform)>,
  mut coin_query: Query<(Entity, &Coin, &Transform)>,
) {
  for (player, player_transform) in &mut player_query.iter() {
    // Collecting coins

    let player_translate = player_transform.translation();

    for (coin_entity, coin, coin_transform) in &mut coin_query.iter() {
      let coin_translate = coin_transform.translation();
      let collision = collide_aabb(player_translate, player.size, coin_translate, coin.size);

      if let Some(_collision) = collision {
        state.coins += 1;
        commands.despawn(coin_entity);
        // println!("I got {} coins!", state.coins);
      }
    }
  }
}
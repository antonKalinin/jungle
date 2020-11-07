use bevy::prelude::*;

use super::super::components::CoinsText;
use super::super::resources::GameState;

pub fn ui(game_state: ResMut<GameState>, mut query: Query<(&mut Text, &CoinsText)>) {
  for (mut text, _tag) in query.iter_mut() {
    text.value = format!("Coins: {}", game_state.coins);
  }
}

use bevy::prelude::*;

use super::super::components::{CoinsText, TimerText};
use super::super::resources::{Game, GameState};

pub fn ui(
  time: Res<Time>,
  mut state: ResMut<GameState>,
  mut coins_query: Query<(&mut Text, &CoinsText)>,
  mut timer_query: Query<(&mut Text, &mut Timer, &TimerText)>,
) {
  for (mut text, _tag) in coins_query.iter_mut() {
    text.value = format!("Coins: {}", state.coins);
  }

  for (mut text, mut timer, _tag) in timer_query.iter_mut() {
    timer.tick(time.delta_seconds());
    if let Game::Started = state.game {
      if timer.finished() {
        state.timer += timer.duration();
        text.value = format!("Timer: {:.1$}", state.timer, 1);
      }
    }
  }
}

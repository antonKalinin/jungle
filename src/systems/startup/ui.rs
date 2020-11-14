use bevy::prelude::*;

use super::super::super::components::{CoinsText, TimerText};
use super::super::super::resources::GameState;

pub fn ui(mut commands: Commands, state: ResMut<GameState>, asset_server: Res<AssetServer>) {
  // Coins counter
  commands
    .spawn(TextComponents {
      style: Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
          top: Val::Px(16.0),
          left: Val::Px(24.0),
          ..Default::default()
        },
        ..Default::default()
      },
      text: Text {
        value: format!("Coins: {}", state.coins),
        font: asset_server.load("font/SBH1.ttf"),
        style: TextStyle {
          font_size: 48.0,
          color: Color::rgb(34., 32., 52.),
        },
      },
      ..Default::default()
    })
    .with(CoinsText);

  // Timer
  commands
    .spawn(TextComponents {
      style: Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: Rect {
          top: Val::Px(56.0),
          left: Val::Px(24.0),
          ..Default::default()
        },
        ..Default::default()
      },
      text: Text {
        value: format!("Time: {}", 0.),
        font: asset_server.load("font/SBH1.ttf"),
        style: TextStyle {
          font_size: 48.0,
          color: Color::rgb(34., 32., 52.),
        },
      },
      ..Default::default()
    })
    .with(TimerText)
    .with(Timer::from_seconds(0.1, true));
}

use bevy::prelude::*;

mod components;
mod constants;
mod systems;

use systems::{animation, movement, player, world};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(player.system())
      .add_startup_system(world.system())
      .add_system(movement.system())
      .add_system(animation.system());
  }
}

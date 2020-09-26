use bevy::prelude::*;

mod components;
mod constants;
mod systems;

use systems::{animation, movement, startup};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .add_startup_system(startup.system())
      .add_system(movement.system())
      .add_system(animation.system());
  }
}

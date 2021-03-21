extern crate tiled;

mod components;
mod constants;
mod resources;
mod systems;
mod utils;

use bevy::{prelude::*, render::pass::ClearColor};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::{GameState, Options, Sprites};
use systems::{action, animation, gameover, movement, player, world};

fn main() {
  let options: Options = argh::from_env();
  let window_width = options.scale as f32 * WINDOW_WIDTH;
  let window_height = options.scale as f32 * WINDOW_HEIGHT;

  let window = WindowDescriptor {
    title: "Jungle".to_string(),
    width: window_width,
    height: window_height,
    ..Default::default()
  };

  let sprites = Sprites::new();

  App::build()
    .init_resource::<GameState>()
    .insert_resource(window)
    .insert_resource(options)
    .insert_resource(sprites)
    .insert_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
    .add_plugins(DefaultPlugins)
    .add_startup_system(player.system())
    .add_startup_system(world.system())
    // .add_startup_system(sui.system())
    .add_system(movement.system())
    .add_system(action.system())
    .add_system(animation.system())
    .add_system(gameover.system())
    // .add_system(ui.system())
    .run();
}

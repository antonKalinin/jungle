extern crate tiled;

mod components;
mod constants;
mod resources;
mod systems;
mod utils;

use bevy::{prelude::*, render::pass::ClearColor, window::WindowMode};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::{GameState, Options, Sprites};
use systems::{action, animation, gameover, movement, player, world};

#[no_mangle]
extern "C" fn main_rs() {
  let options: Options = argh::from_env();
  let window_width = options.scale * WINDOW_WIDTH as u32;
  let window_height = options.scale * WINDOW_HEIGHT as u32;

  let window = WindowDescriptor {
    vsync: true,
    resizable: false,
    mode: WindowMode::BorderlessFullscreen,
    ..Default::default()
  };

  let sprites = Sprites::new();

  let state = GameState {
    coins: 0,
    game_over: false,
  };

  App::build()
    .add_resource(window)
    .add_resource(options)
    .add_resource(sprites)
    .add_resource(state)
    .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
    .add_default_plugins()
    .add_startup_system(player.system())
    .add_startup_system(world.system())
    .add_system(movement.system())
    .add_system(action.system())
    .add_system(animation.system())
    .add_system(gameover.system())
    .run();
}

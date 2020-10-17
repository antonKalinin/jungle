use bevy::{prelude::*, render::pass::ClearColor};

mod components;
mod resources;
mod systems;

use resources::Options;
use systems::{movement, world};

const WINDOW_WIDTH: f32 = 384.0;
const WINDOW_HEIGHT: f32 = 216.0;

fn main() {
  let options: Options = argh::from_env();
  let window_width = options.scale * WINDOW_WIDTH as u32;
  let window_height = options.scale * WINDOW_HEIGHT as u32;

  let window = WindowDescriptor {
    title: "Jungle".to_string(),
    width: window_width,
    height: window_height,
    ..Default::default()
  };

  App::build()
    .add_resource(window)
    .add_resource(options)
    .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
    .add_default_plugins()
    .add_startup_system(world.system())
    .add_system(movement.system())
    .run();
}

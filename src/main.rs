extern crate tiled;

mod play;
mod resources;

use bevy::{prelude::*, render::pass::ClearColor};
use play::PlayPlugin;
use resources::{GameState, Options, Sprites};

pub const WINDOW_WIDTH: f32 = 384.0;
pub const WINDOW_HEIGHT: f32 = 216.0;

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

  let sprites = Sprites::new();

  App::build()
    .add_resource(window)
    .add_resource(options)
    .add_resource(sprites)
    .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
    .init_resource::<GameState>()
    .add_default_plugins()
    .add_plugin(PlayPlugin)
    .run();
}

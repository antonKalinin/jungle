use argh::FromArgs;
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 384.0;
pub const WINDOW_HEIGHT: f32 = 216.0;

#[derive(FromArgs)]
#[argh(description = "Jungle game settings")]
pub struct Options {
  #[argh(
    option,
    default = "4",
    short = 's',
    description = "scale of game window"
  )]
  pub scale: u32,
}

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
    .add_default_plugins()
    .run();
}

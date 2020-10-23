use argh::FromArgs;
use bevy::prelude::*;
use std::collections::HashMap;

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

#[derive(Default)]
pub struct GameState {
  pub coins: u8,
  pub game_over: bool,
}

pub struct Sprites {
  library: HashMap<String, Handle<TextureAtlas>>,
}

impl Sprites {
  pub fn new() -> Sprites {
    Sprites {
      library: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: String, atlas_handle: Handle<TextureAtlas>) {
    self.library.insert(key, atlas_handle);
  }

  pub fn get(&self, key: &str) -> Option<&Handle<TextureAtlas>> {
    self.library.get(key)
  }
}

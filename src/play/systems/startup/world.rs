use std::path::Path;

use bevy::prelude::*;
use tiled::parse_file;

use super::super::super::super::resources::Options;
use super::super::super::components::{Background, Block, Camera};
use super::super::super::constants::{BG_HEIGHT, BG_WIDTH};

pub fn world(
  mut commands: Commands,
  options: Res<Options>,
  asset_server: Res<AssetServer>,
  mut textures: ResMut<Assets<Texture>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let scale = options.scale as f32;
  let bottom = scale * (-BG_HEIGHT / 2.0 + 8.0);

  commands
    .spawn(Camera2dComponents {
      translation: Translation::new(0.0, 0.0, 20.0),
      ..Default::default()
    })
    .with(Camera {});

  let map = parse_file(&Path::new("assets/level1.tmx")).unwrap();
  println!("{:?}", map.get_tileset_by_gid(1));

  for i in 1..=5 {
    let bg_handle = asset_server
      .load(format!("assets/background/plx-{}.png", i))
      .unwrap();

    for j in 0..=1 {
      commands
        .spawn(SpriteComponents {
          scale: Scale(scale),
          material: materials.add(bg_handle.into()),
          translation: Translation::new(scale * BG_WIDTH * j as f32, 0.0, i as f32),
          ..Default::default()
        })
        .with(Background {
          acceleration: 0.2 * (5 - i) as f32,
        });
    }
  }

  let block_texture_handle = asset_server
    .load_sync(&mut textures, "assets/tileset.png")
    .unwrap();
  let block_texture = textures.get(&block_texture_handle).unwrap();
  let block_texture_atlas =
    TextureAtlas::from_grid(block_texture_handle, block_texture.size, 48, 23);

  let block_atlas_handle = texture_atlases.add(block_texture_atlas);

  for i in -20..=20 {
    commands
      .spawn(SpriteSheetComponents {
        scale: Scale(scale),
        sprite: TextureAtlasSprite::new(101),
        texture_atlas: block_atlas_handle.clone(),
        translation: Translation::new(i as f32 * 16.0 * scale, bottom, 10.0),
        ..Default::default()
      })
      .with(Block {
        size: Vec2::new(16.0 * scale, 16.0 * scale),
        position: Vec2::new(i as f32 * 16.0 * scale, bottom),
      });
  }
}

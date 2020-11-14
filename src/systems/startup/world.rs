use std::path::Path;

use bevy::prelude::*;
use tiled::parse_file;

use super::super::super::components::{Background, Block, Camera, CheckPoint, Coin, Hook};
use super::super::super::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use super::super::super::resources::Options;

#[derive(Bundle)]
struct BlockComponent {
  id: u32,
}

pub fn world(
  mut commands: Commands,
  options: Res<Options>,
  window: Res<WindowDescriptor>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let scale = options.scale as f32;

  // Camera

  commands
    .spawn(UiCameraComponents::default())
    .spawn(Camera2dComponents::default())
    .with(Camera);

  // Background

  for i in 1..=5 {
    let bg_handle = asset_server.load(format!("background/plx-{}.png", i).as_str());

    for j in 0..=1 {
      commands
        .spawn(SpriteComponents {
          material: materials.add(bg_handle.clone().into()),
          transform: Transform {
            translation: Vec3::new(scale * WINDOW_WIDTH * j as f32, 0.0, i as f32),
            scale: Vec3::splat(scale),
            ..Default::default()
          },
          ..Default::default()
        })
        .with(Background {
          acceleration: 0.2 * (5 - i) as f32,
        });
    }
  }

  // Surfaces

  let block_handle = asset_server.load("tileset.png");
  let block_atlas = TextureAtlas::from_grid(block_handle, Vec2::new(16.0, 16.0), 48, 23);
  let block_atlas_handle = texture_atlases.add(block_atlas);

  let map = parse_file(&Path::new("assets/level1.tmx")).unwrap();

  let tile_width = 16.0;
  let tile_height = 16.0;

  for layer in map.layers.iter() {
    for i in 0..map.width {
      for j in 0..map.height {
        let tile = match &layer.tiles {
          tiled::LayerData::Finite(tiles) => &tiles[j as usize][i as usize],
          _ => panic!("Infinte maps not supported"),
        };

        if tile.gid == 0 {
          continue;
        }

        let tile_x = (tile_width * i as f32) * scale;
        let tile_y = (WINDOW_HEIGHT / 2.0 - tile_height * j as f32) * scale;

        commands
          .spawn(SpriteSheetComponents {
            transform: Transform {
              translation: Vec3::new(tile_x, tile_y, 10.0),
              scale: Vec3::splat(scale),
              ..Default::default()
            },
            sprite: TextureAtlasSprite::new(tile.gid - 1),
            texture_atlas: block_atlas_handle.clone(),
            ..Default::default()
          })
          .with(Block {
            size: Vec2::new(tile_width * scale, tile_height * scale),
          });
      }
    }
  }

  let coin_handle = asset_server.load("objects/coin.png");
  let coin_atlas = TextureAtlas::from_grid(coin_handle, Vec2::new(16.0, 16.0), 8, 1);
  let coin_atlas_handle = texture_atlases.add(coin_atlas);

  let totem_handle = asset_server.load("objects/totem.png");
  let totem_atlas = TextureAtlas::from_grid(totem_handle, Vec2::new(19.0, 27.0), 1, 1);
  let totem_atlas_handle = texture_atlases.add(totem_atlas);

  // Objects

  for group in map.object_groups.iter() {
    for object in group.objects.iter() {
      // Coins
      if object.obj_type == "coin" {
        commands
          .spawn(SpriteSheetComponents {
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
              translation: Vec3::new(
                scale * object.x,
                window.height as f32 / 2.0 - scale * object.y,
                10.0,
              ),
              scale: Vec3::splat(scale),
              ..Default::default()
            },
            texture_atlas: coin_atlas_handle.clone(),
            ..Default::default()
          })
          .with(Coin {
            size: Vec2::new(object.width * scale, object.height * scale),
          })
          .with(Timer::from_seconds(0.08, true));
      }

      // Hooks
      if object.obj_type == "hook" {
        commands
          .spawn(SpriteComponents {
            transform: Transform {
              translation: Vec3::new(
                scale * object.x,
                window.height as f32 / 2.0 - scale * object.y,
                10.0,
              ),
              scale: Vec3::splat(scale),
              ..Default::default()
            },
            ..Default::default()
          })
          .with(Hook {
            size: Vec2::new(object.width * scale, object.height * scale),
          });
      }

      // Checkpoints
      if object.obj_type == "checkpoint" {
        commands
          .spawn(SpriteSheetComponents {
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
              translation: Vec3::new(
                scale * object.x,
                window.height as f32 / 2.0 - scale * object.y,
                9.0,
              ),
              scale: Vec3::splat(scale),
              ..Default::default()
            },
            texture_atlas: totem_atlas_handle.clone(),
            ..Default::default()
          })
          .with(CheckPoint {
            size: Vec2::new(object.width * scale, object.height * scale),
          });
      }
    }
  }
}

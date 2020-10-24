use std::path::Path;

use bevy::prelude::*;
use tiled::parse_file;

use super::super::super::components::{Background, Block, Camera, Coin, Hook};
use super::super::super::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use super::super::super::resources::Options;

#[derive(Bundle)]
struct BlockComponent {
  id: u32,
}

pub fn world(
  mut commands: Commands,
  window: Res<WindowDescriptor>,
  options: Res<Options>,
  asset_server: Res<AssetServer>,
  mut textures: ResMut<Assets<Texture>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let scale = options.scale as f32;

  // Camera

  commands
    .spawn(Camera2dComponents::default())
    .with(Camera {});

  // Background

  for i in 1..=5 {
    let bg_handle = asset_server
      .load(format!("assets/background/plx-{}.png", i))
      .unwrap();

    for j in 0..=1 {
      commands
        .spawn(SpriteComponents {
          material: materials.add(bg_handle.into()),
          transform: Transform::from_translation(Vec3::new(
            scale * WINDOW_WIDTH * j as f32,
            0.0,
            i as f32,
          ))
          .with_scale(scale),
          ..Default::default()
        })
        .with(Background {
          acceleration: 0.2 * (5 - i) as f32,
        });
    }
  }

  // Surfaces

  let block_texture_handle = asset_server
    .load_sync(&mut textures, "assets/tileset.png")
    .unwrap();
  let block_texture = textures.get(&block_texture_handle).unwrap();
  let block_texture_atlas =
    TextureAtlas::from_grid(block_texture_handle, block_texture.size, 48, 23);

  let block_atlas_handle = texture_atlases.add(block_texture_atlas);

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
            transform: Transform::from_translation(Vec3::new(tile_x, tile_y, 10.0))
              .with_scale(scale),
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

  let object_handle = asset_server
    .load_sync(&mut textures, "assets/objects/coin.png")
    .unwrap();
  let object_texture = textures.get(&object_handle).unwrap();
  let object_texture_atlas = TextureAtlas::from_grid(object_handle, object_texture.size, 8, 1);

  let object_atlas_handle = texture_atlases.add(object_texture_atlas);

  // Objects

  for group in map.object_groups.iter() {
    for object in group.objects.iter() {
      // Coins
      if object.obj_type == "coin" {
        commands
          .spawn(SpriteSheetComponents {
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(
              scale * object.x,
              window.height as f32 / 2.0 - scale * object.y,
              10.0,
            ))
            .with_scale(scale),
            texture_atlas: object_atlas_handle.clone(),
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
            transform: Transform::from_translation(Vec3::new(
              scale * object.x,
              window.height as f32 / 2.0 - scale * object.y,
              10.0,
            ))
            .with_scale(scale),
            ..Default::default()
          })
          .with(Hook {
            size: Vec2::new(object.width * scale, object.height * scale),
          });
      }
    }
  }
}

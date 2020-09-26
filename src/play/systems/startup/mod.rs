use bevy::prelude::*;

use super::super::super::resources::{Options, Sprites};
use super::super::components::{Background, Block, Camera, Player};
use super::super::constants::{BG_HEIGHT, BG_WIDTH};

pub fn startup(
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

  let run_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/run.png")
    .unwrap();
  let run_texture = textures.get(&run_texture_handle).unwrap();
  let run_texture_atlas = TextureAtlas::from_grid(run_texture_handle, run_texture.size, 8, 1);

  let idle_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/idle.png")
    .unwrap();
  let idle_texture = textures.get(&idle_texture_handle).unwrap();
  let idle_texture_atlas = TextureAtlas::from_grid(idle_texture_handle, idle_texture.size, 12, 1);

  let jump_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/jump.png")
    .unwrap();
  let jump_texture = textures.get(&jump_texture_handle).unwrap();
  let jump_texture_atlas = TextureAtlas::from_grid(jump_texture_handle, jump_texture.size, 1, 1);

  let air_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/air.png")
    .unwrap();
  let air_texture = textures.get(&air_texture_handle).unwrap();
  let air_texture_atlas = TextureAtlas::from_grid(air_texture_handle, air_texture.size, 2, 1);

  let land_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/land.png")
    .unwrap();
  let land_texture = textures.get(&land_texture_handle).unwrap();
  let land_texture_atlas = TextureAtlas::from_grid(land_texture_handle, land_texture.size, 1, 1);

  let block_texture_handle = asset_server
    .load_sync(&mut textures, "assets/tileset.png")
    .unwrap();
  let block_texture = textures.get(&block_texture_handle).unwrap();
  let block_texture_atlas =
    TextureAtlas::from_grid(block_texture_handle, block_texture.size, 48, 23);

  let air_atlas_handle = texture_atlases.add(air_texture_atlas);
  let run_atlas_handle = texture_atlases.add(run_texture_atlas);
  let land_texture_atlas = texture_atlases.add(land_texture_atlas);
  let idle_atlas_handle = texture_atlases.add(idle_texture_atlas);
  let jump_atlas_handle = texture_atlases.add(jump_texture_atlas);
  let block_atlas_handle = texture_atlases.add(block_texture_atlas);

  commands
    .spawn(SpriteSheetComponents {
      scale: Scale(scale),
      translation: Translation::new(0.0, bottom, 15.0),
      texture_atlas: idle_atlas_handle.clone(),
      ..Default::default()
    })
    .with(Player {
      size: Vec2::new(19.0 * scale, 33.0 * scale),
      position: Vec2::new(0.0, 0.0),
      velocity: Vec2::new(0.0, 0.0),
    })
    .with(Timer::from_seconds(0.1, true));

  let mut sprites = Sprites::new();

  sprites.add("player_air".to_string(), air_atlas_handle);
  sprites.add("player_run".to_string(), run_atlas_handle);
  sprites.add("player_idle".to_string(), idle_atlas_handle);
  sprites.add("player_jump".to_string(), jump_atlas_handle);
  sprites.add("player_land".to_string(), land_texture_atlas);

  commands.insert_resource(sprites);

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

  commands
    .spawn(SpriteSheetComponents {
      scale: Scale(scale),
      sprite: TextureAtlasSprite::new(101),
      texture_atlas: block_atlas_handle.clone(),
      translation: Translation::new(5.0 * 16.0 * scale, bottom + 16.0 * scale, 10.0),
      ..Default::default()
    })
    .with(Block {
      size: Vec2::new(16.0 * scale, 16.0 * scale),
      position: Vec2::new(5.0 * 16.0 * scale, bottom + 16.0 * scale),
    });

  commands
    .spawn(SpriteSheetComponents {
      scale: Scale(scale),
      sprite: TextureAtlasSprite::new(101),
      texture_atlas: block_atlas_handle.clone(),
      translation: Translation::new(5.0 * 16.0 * scale, bottom + 32.0 * scale, 10.0),
      ..Default::default()
    })
    .with(Block {
      size: Vec2::new(16.0 * scale, 16.0 * scale),
      position: Vec2::new(5.0 * 16.0 * scale, bottom + 32.0 * scale),
    });

  commands
    .spawn(SpriteSheetComponents {
      scale: Scale(scale),
      sprite: TextureAtlasSprite::new(101),
      texture_atlas: block_atlas_handle.clone(),
      translation: Translation::new(5.0 * 16.0 * scale, bottom + 48.0 * scale, 10.0),
      ..Default::default()
    })
    .with(Block {
      size: Vec2::new(16.0 * scale, 16.0 * scale),
      position: Vec2::new(5.0 * 16.0 * scale, bottom + 48.0 * scale),
    });
}

use bevy::prelude::*;

use super::super::super::components::Player;
use super::super::super::resources::{Options, Sprites};

pub fn player(
  mut commands: Commands,
  options: Res<Options>,
  mut sprites: ResMut<Sprites>,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let scale = options.scale as f32;

  let run_handle = asset_server.load("player/run.png");
  let run_atlas = TextureAtlas::from_grid(run_handle, Vec2::new(21.0, 33.0), 8, 1);

  let idle_handle = asset_server.load("player/idle.png");
  let idle_atlas = TextureAtlas::from_grid(idle_handle, Vec2::new(19.0, 34.0), 12, 1);

  let jump_handle = asset_server.load("player/jump.png");
  let jump_atlas = TextureAtlas::from_grid(jump_handle, Vec2::new(17.0, 34.0), 1, 1);

  let air_handle = asset_server.load("player/air.png");
  let air_atlas = TextureAtlas::from_grid(air_handle, Vec2::new(20.0, 35.0), 2, 1);

  let land_handle = asset_server.load("player/land.png");
  let land_atlas = TextureAtlas::from_grid(land_handle, Vec2::new(20.0, 35.0), 1, 1);

  let grab_handle = asset_server.load("player/grab.png");
  let grab_atlas = TextureAtlas::from_grid(grab_handle, Vec2::new(20.0, 40.0), 6, 1);

  let air_atlas_handle = texture_atlases.add(air_atlas);
  let run_atlas_handle = texture_atlases.add(run_atlas);
  let land_atlas_handle = texture_atlases.add(land_atlas);
  let idle_atlas_handle = texture_atlases.add(idle_atlas);
  let jump_atlas_handle = texture_atlases.add(jump_atlas);
  let grab_atlas_handle = texture_atlases.add(grab_atlas);

  let player = Player {
    size: Vec2::new(19.0 * scale, 31.0 * scale),
    velocity: Vec3::new(0.0, 0.0, 0.0),
    initial_position: Vec3::new(16.0 * scale, 32.0 * scale * 5.0, 15.0),
    is_grabbing: false,
  };

  commands
    .spawn(SpriteSheetComponents {
      sprite: TextureAtlasSprite::new(0),
      transform: Transform {
        translation: player.initial_position,
        scale: Vec3::splat(scale),
        ..Default::default()
      },
      texture_atlas: idle_atlas_handle.clone(),
      ..Default::default()
    })
    .with(player)
    .with(Timer::from_seconds(0.1, true));

  sprites.add("player_air".to_string(), air_atlas_handle);
  sprites.add("player_run".to_string(), run_atlas_handle);
  sprites.add("player_idle".to_string(), idle_atlas_handle);
  sprites.add("player_jump".to_string(), jump_atlas_handle);
  sprites.add("player_land".to_string(), land_atlas_handle);
  sprites.add("player_grab".to_string(), grab_atlas_handle);
}

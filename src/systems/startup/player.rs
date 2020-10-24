use bevy::prelude::*;

use super::super::super::components::Player;
use super::super::super::resources::{Options, Sprites};

pub fn player(
  mut commands: Commands,
  options: Res<Options>,
  mut sprites: ResMut<Sprites>,
  asset_server: Res<AssetServer>,
  mut textures: ResMut<Assets<Texture>>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
  let scale = options.scale as f32;

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

  let grab_texture_handle = asset_server
    .load_sync(&mut textures, "assets/player/grab.png")
    .unwrap();
  let grab_texture = textures.get(&grab_texture_handle).unwrap();
  let grab_texture_atlas = TextureAtlas::from_grid(grab_texture_handle, grab_texture.size, 6, 1);

  let air_atlas_handle = texture_atlases.add(air_texture_atlas);
  let run_atlas_handle = texture_atlases.add(run_texture_atlas);
  let land_atlas_handle = texture_atlases.add(land_texture_atlas);
  let idle_atlas_handle = texture_atlases.add(idle_texture_atlas);
  let jump_atlas_handle = texture_atlases.add(jump_texture_atlas);
  let grab_atlas_handle = texture_atlases.add(grab_texture_atlas);

  let player = Player {
    size: Vec2::new(21.0 * scale, 33.0 * scale),
    velocity: Vec3::new(0.0, 0.0, 0.0),
    initial_position: Vec3::new(16.0 * scale, 32.0 * scale * 5.0, 15.0),
    is_grabbing: false,
  };

  commands
    .spawn(SpriteSheetComponents {
      sprite: TextureAtlasSprite::new(0),
      transform: Transform::from_translation(player.initial_position).with_scale(scale),
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

use bevy::prelude::*;

use super::super::components::{Background, Camera, Coin, Player};
use super::super::resources::Sprites;

pub fn animation(
  time: Res<Time>,
  sprites: Res<Sprites>,
  window: Res<WindowDescriptor>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut player_query: Query<(
    &Player,
    &Transform,
    &mut Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
  mut camera_query: Query<(&Camera, &mut Transform)>,
  mut coin_query: Query<(
    &Coin,
    &mut Timer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
  mut background_query: Query<(&Background, &mut Transform)>,
) {
  for (player, player_transform, mut timer, mut sprite, mut texture_atlas_handle) in
    player_query.iter_mut()
  {
    timer.tick(time.delta_seconds());

    if player.velocity.x != 0.0 {
      if let Some(player_run) = sprites.get("player_run") {
        texture_atlas_handle.id = player_run.id;
      }
    } else {
      if let Some(player_idle) = sprites.get("player_idle") {
        texture_atlas_handle.id = player_idle.id;
      }
    }

    if player.velocity.y > 0.0 {
      if let Some(player_jump) = sprites.get("player_jump") {
        texture_atlas_handle.id = player_jump.id;
      }
    }

    if player.velocity.y < 0.0 {
      if let Some(player_land) = sprites.get("player_land") {
        texture_atlas_handle.id = player_land.id;
      }
    }

    if player.is_grabbing {
      if let Some(player_grab) = sprites.get("player_grab") {
        texture_atlas_handle.id = player_grab.id;
      }
    }

    if timer.finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle.clone()).unwrap();
      if !(player.is_grabbing && sprite.index == 5) {
        sprite.index = ((sprite.index as usize + 1) % texture_atlas.len()) as u32;
      }
    }

    for (_camera, mut camera_transform) in camera_query.iter_mut() {
      camera_transform.translation.x = player_transform.translation.x;
    }

    for (background, mut background_transform) in background_query.iter_mut() {
      let player_translation = player_transform.translation;
      let background_translation = background_transform.translation;

      background_transform.translation.x += player.velocity.x * background.acceleration;

      if player_translation.x - background_translation.x > window.width as f32 {
        background_transform.translation.x += 2.0 * window.width as f32;
      } else if player_translation.x - background_translation.x < -(window.width as f32) {
        background_transform.translation.x -= 2.0 * window.width as f32;
      }
    }
  }

  for (_coin, mut timer, mut sprite, texture_atlas_handle) in coin_query.iter_mut() {
    timer.tick(time.delta_seconds());
    if timer.finished() {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    }
  }
}

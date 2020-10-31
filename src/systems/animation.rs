use bevy::prelude::*;

use super::super::components::{Background, Camera, Coin, Player};
use super::super::resources::Sprites;

pub fn animation(
  sprites: Res<Sprites>,
  window: Res<WindowDescriptor>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut player_query: Query<(
    &Player,
    &Transform,
    &Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
  mut camera_query: Query<(&Camera, &mut Transform)>,
  mut coin_query: Query<(
    &Coin,
    &Timer,
    &mut TextureAtlasSprite,
    &Handle<TextureAtlas>,
  )>,
  mut background_query: Query<(&Background, &mut Transform)>,
) {
  for (player, player_transform, timer, mut sprite, mut texture_atlas_handle) in
    &mut player_query.iter()
  {
    if player.velocity.x() != 0.0 {
      if let Some(player_run) = sprites.get("player_run") {
        texture_atlas_handle.set(player_run);
      }
    } else {
      if let Some(player_idle) = sprites.get("player_idle") {
        texture_atlas_handle.set(player_idle);
      }
    }

    if player.velocity.y() > 0.0 {
      if let Some(player_jump) = sprites.get("player_jump") {
        texture_atlas_handle.set(player_jump);
      }
    }

    if player.velocity.y() < 0.0 {
      if let Some(player_land) = sprites.get("player_land") {
        texture_atlas_handle.set(player_land);
      }
    }

    if player.is_grabbing {
      if let Some(player_grab) = sprites.get("player_grab") {
        texture_atlas_handle.set(player_grab);
      }
    }

    if timer.finished {
      let texture_atlas = texture_atlases.get(texture_atlas_handle.clone()).unwrap();
      if !(player.is_grabbing && sprite.index == 5) {
        sprite.index = ((sprite.index as usize + 1) % texture_atlas.len()) as u32;
      }
    }

    for (_camera, mut camera_transform) in &mut camera_query.iter() {
      camera_transform
        .translation
        .set_x(player_transform.translation.x());
    }

    for (background, mut background_transform) in &mut background_query.iter() {
      let player_translation = player_transform.translation;
      let background_translation = background_transform.translation;

      *background_transform.translation.x_mut() += player.velocity.x() * background.acceleration;

      if player_translation.x() - background_translation.x() > window.width as f32 {
        *background_transform.translation.x_mut() += 2.0 * window.width as f32;
      } else if player_translation.x() - background_translation.x() < -(window.width as f32) {
        *background_transform.translation.x_mut() -= 2.0 * window.width as f32;
      }
    }
  }

  for (_coin, timer, mut sprite, texture_atlas_handle) in &mut coin_query.iter() {
    if timer.finished {
      let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    }
  }
}

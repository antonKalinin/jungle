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
      if let Some(sprite_handle) = sprites.get("player_run") {
        *texture_atlas_handle = *sprite_handle;
      }
    } else {
      if let Some(sprite_handle) = sprites.get("player_idle") {
        *texture_atlas_handle = *sprite_handle;
      }
    }

    if player.velocity.y() > 0.0 {
      if let Some(sprite_handle) = sprites.get("player_jump") {
        *texture_atlas_handle = *sprite_handle;
      }
    }

    if player.velocity.y() < 0.0 {
      if let Some(sprite_handle) = sprites.get("player_land") {
        *texture_atlas_handle = *sprite_handle;
      }
    }

    if timer.finished {
      let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    }

    for (_camera, mut camera_transform) in &mut camera_query.iter() {
      camera_transform
        .translation_mut()
        .set_x(player_transform.translation().x());
    }

    for (background, mut background_transform) in &mut background_query.iter() {
      let player_translation = player_transform.translation();
      let background_translation = background_transform.translation_mut();

      *background_translation.x_mut() += player.velocity.x() * background.acceleration;

      if player_translation.x() - background_translation.x() > window.width as f32 {
        *background_translation.x_mut() += 2.0 * window.width as f32;
      } else if player_translation.x() - background_translation.x() < -(window.width as f32) {
        *background_translation.x_mut() -= 2.0 * window.width as f32;
      }
    }
  }

  for (_coin, timer, mut sprite, texture_atlas_handle) in &mut coin_query.iter() {
    if timer.finished {
      let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
      sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
    }
  }
}

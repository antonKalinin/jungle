use bevy::prelude::*;

use super::super::super::resources::{Options, Sprites};
use super::super::components::{Background, Camera, Player};
use super::super::constants::BG_WIDTH;

pub fn animation(
  options: Res<Options>,
  sprites: Res<Sprites>,
  texture_atlases: Res<Assets<TextureAtlas>>,
  mut player_query: Query<(
    &Player,
    &Transform,
    &mut Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
  mut camera_query: Query<(&Camera, &mut Transform)>,
  mut background_query: Query<(&Background, &mut Transform)>,
) {
  let scale = options.scale as f32;

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

      if player_translation.x() - background_translation.x() > BG_WIDTH * scale {
        *background_translation.x_mut() += 2.0 * BG_WIDTH * scale;
      } else if player_translation.x() - background_translation.x() < -BG_WIDTH * scale {
        *background_translation.x_mut() -= 2.0 * BG_WIDTH * scale;
      }
    }
  }
}

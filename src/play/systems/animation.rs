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
    &mut Rotation,
    &mut Translation,
    &mut Timer,
    &mut TextureAtlasSprite,
    &mut Handle<TextureAtlas>,
  )>,
  mut camera_query: Query<(&Camera, &mut Translation)>,
  mut background_query: Query<(&Background, &mut Translation)>,
) {
  let scale = options.scale as f32;

  for (player, mut rotation, mut translation, timer, mut sprite, mut texture_atlas_handle) in
    &mut player_query.iter()
  {
    *translation.x_mut() = player.position.x();
    *translation.y_mut() = player.position.y();

    if player.velocity.x() != 0.0 {
      if let Some(sprite_handle) = sprites.get("player_run") {
        *texture_atlas_handle = *sprite_handle;
      }

      if player.velocity.x() > 0.0 {
        *rotation = Rotation(Quat::from_rotation_y(0.0));
      } else {
        *rotation = Rotation(Quat::from_rotation_y(std::f32::consts::PI));
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

    for (_, mut translation) in &mut camera_query.iter() {
      translation.set_x(player.position.x());
    }

    for (background, mut translation) in &mut background_query.iter() {
      *translation.0.x_mut() += player.velocity.x() * background.acceleration;

      if player.position.x() - translation.0.x() > BG_WIDTH * scale {
        *translation.0.x_mut() += 2.0 * BG_WIDTH * scale;
      } else if player.position.x() - translation.0.x() < -BG_WIDTH * scale {
        *translation.0.x_mut() -= 2.0 * BG_WIDTH * scale;
      }
    }
  }
}

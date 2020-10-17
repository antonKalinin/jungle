use bevy::prelude::*;

use super::super::components::{Background, Camera};
use super::super::resources::Options;

pub fn world(
  mut commands: Commands,
  options: Res<Options>,
  window: Res<WindowDescriptor>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
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
            window.width as f32 * j as f32,
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
}

use bevy::prelude::*;

use super::super::components::{Background, Camera};

const HORIZONTAL_SPEED: f32 = 16.0;

pub fn movement(
  window: Res<WindowDescriptor>,
  keyboard_input: Res<Input<KeyCode>>,
  mut camera_query: Query<(&Camera, &mut Transform)>,
  mut background_query: Query<(&Background, &mut Transform)>,
) {
  for (mut _camera, mut camera_transform) in &mut camera_query.iter() {
    let mut velocity = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Right) {
      velocity = Vec3::new(HORIZONTAL_SPEED, 0.0, 0.0);
    }

    if keyboard_input.pressed(KeyCode::Left) {
      velocity = Vec3::new(-HORIZONTAL_SPEED, 0.0, 0.0);
    }

    camera_transform.translate(velocity);

    for (background, mut background_transform) in &mut background_query.iter() {
      let camera_translation = camera_transform.translation();
      let background_translation = background_transform.translation_mut();

      *background_translation.x_mut() += velocity.x() * background.acceleration;

      if camera_translation.x() - background_translation.x() > window.width as f32 {
        *background_translation.x_mut() += 2.0 * window.width as f32;
      } else if camera_translation.x() - background_translation.x() < -(window.width as f32) {
        *background_translation.x_mut() -= 2.0 * window.width as f32;
      }
    }
  }
}

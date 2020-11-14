use bevy::prelude::*;

type Collision = Option<Vec2>;

pub fn collide_aabb(a_position: Vec3, a_size: Vec2, b_position: Vec3, b_size: Vec2) -> Collision {
  let (a_x, a_y, _) = <(f32, f32, f32)>::from(a_position);
  let (a_w, a_h) = <(f32, f32)>::from(a_size);
  let (b_x, b_y, _) = <(f32, f32, f32)>::from(b_position);
  let (b_w, b_h) = <(f32, f32)>::from(b_size);

  if (a_x - b_x).abs() < (a_w / 2.0 + b_w / 2.0) && (a_y - b_y).abs() < (a_h / 2.0 + b_h / 2.0) {
    let x = (a_x - b_x).signum() * ((a_x - b_x).abs() - (a_w / 2.0 + b_w / 2.0));
    let y = (a_y - b_y).signum() * ((a_y - b_y).abs() - (a_h / 2.0 + b_h / 2.0));

    return Some(Vec2::new(x, y));
  }

  None
}

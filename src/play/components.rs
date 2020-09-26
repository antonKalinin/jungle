use bevy::prelude::*;

pub trait Object {
  fn position(&self) -> Vec2;
  fn size(&self) -> Vec2;
}

pub trait AABB: Object {
  fn collides(&self, other: &impl Object) -> bool {
    let (self_x, self_y) = <(f32, f32)>::from(self.position());
    let (self_size_x, self_size_y) = <(f32, f32)>::from(self.size());
    let (other_x, other_y) = <(f32, f32)>::from(other.position());
    let (other_size_x, other_size_y) = <(f32, f32)>::from(other.size());

    if (self_x - other_x).abs() < (self_size_x / 2.0 + other_size_x / 2.0)
      && (self_y - other_y).abs() < (self_size_y / 2.0 + other_size_y / 2.0)
    {
      return true;
    }

    false
  }

  fn collision_by_axis(&self, other: &impl Object) -> Vec2 {
    let (self_x, self_y) = <(f32, f32)>::from(self.position());
    let (self_size_x, self_size_y) = <(f32, f32)>::from(self.size());
    let (other_x, other_y) = <(f32, f32)>::from(other.position());
    let (other_size_x, other_size_y) = <(f32, f32)>::from(other.size());

    let h = (self_x - other_x).signum()
      * ((self_x - other_x).abs() - (self_size_x / 2.0 + other_size_x / 2.0));

    let v = (self_y - other_y).signum()
      * ((self_y - other_y).abs() - (self_size_y / 2.0 + other_size_y / 2.0));

    Vec2::new(h, v)
  }
}

pub struct Player {
  pub position: Vec2,
  pub size: Vec2,
  pub velocity: Vec2,
}

impl Object for Player {
  fn position(&self) -> Vec2 {
    self.position.clone()
  }

  fn size(&self) -> Vec2 {
    self.size.clone()
  }
}

impl AABB for Player {}

pub struct Block {
  pub position: Vec2,
  pub size: Vec2,
}

impl Object for Block {
  fn position(&self) -> Vec2 {
    self.position.clone()
  }

  fn size(&self) -> Vec2 {
    self.size.clone()
  }
}

pub struct Background {
  pub acceleration: f32,
}

pub struct Camera {}

use bevy::prelude::*;

pub struct Background {
  pub acceleration: f32,
}

pub struct Block {
  pub size: Vec2,
}

pub struct Camera {}

pub struct Object {}

pub struct Player {
  pub size: Vec2,
  pub velocity: Vec3,
  pub initial_position: Vec3,
}

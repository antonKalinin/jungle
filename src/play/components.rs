use bevy::prelude::*;

pub struct Player {
  pub size: Vec2,
  pub velocity: Vec3,
}

pub struct Block {
  pub size: Vec2,
}

pub struct Background {
  pub acceleration: f32,
}

pub struct Camera {}

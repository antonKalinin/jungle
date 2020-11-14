use bevy::prelude::*;

pub struct Background {
  pub acceleration: f32,
}

pub struct Block {
  pub size: Vec2,
}

pub struct CheckPoint {
  pub size: Vec2,
}

pub struct Coin {
  pub size: Vec2,
}

pub struct CoinsText;

pub struct Camera;

pub struct Hook {
  pub size: Vec2,
}

pub struct Player {
  pub size: Vec2,
  pub velocity: Vec3,
  pub initial_position: Vec3,
  pub is_grabbing: bool,
  pub is_in_air: bool,
}

pub struct TimerText;

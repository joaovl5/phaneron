use derive_more::{Display, Error, From};
use raylib::ffi::Vector2;

#[derive(Error, From, Debug, Display)]
pub enum PhaneronError {}

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  /// Gets the inverse of that direction.
  pub fn inverse(&self) -> Self {
    match self {
      | Self::Up => Self::Down,
      | Self::Down => Self::Up,
      | Self::Left => Self::Right,
      | Self::Right => Self::Left,
    }
  }

  /// Generates a vector for deriving the center-point of the side of a box
  /// corresponding to that direction. This only returns the factor that you
  /// have to multiply the box's size by in order to add to the position and get
  /// the final point.
  pub fn get_directional_center(&self) -> Vector2 {
    (match self {
      | Self::Up => (0.5, 0.0),
      | Self::Down => (0.5, 1.0),
      | Self::Left => (0.0, 0.5),
      | Self::Right => (1.0, 0.5),
    })
    .into()
  }

  /// Gets per-axis distances for two points: "along" and "across".
  ///
  /// - "along" is within the same axis as this dimension
  /// - "across" is within the perpendicular axis
  pub fn get_along_across(&self, from : &Vector2, to : &Vector2) -> (f32, f32) {
    match self {
      | Self::Up => (from.y - to.y, (to.x - from.x).abs()),
      | Self::Down => (to.y - from.y, (to.x - from.x).abs()),
      | Self::Left => (from.x - to.x, (to.y - from.y).abs()),
      | Self::Right => (to.x - from.x, (to.y - from.y).abs()),
    }
  }

  pub fn contains_across(
    &self,
    pt : &Vector2,
    pos : Vector2,
    dims : Vector2,
  ) -> bool {
    match self {
      | Self::Up | Self::Down => pos.x <= pt.x && pt.x <= pos.x + dims.x,
      | Self::Left | Self::Right => pos.y <= pt.y && pt.y <= pos.y + dims.y,
    }
  }
}

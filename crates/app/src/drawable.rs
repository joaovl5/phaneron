use raylib::ffi::Vector2;

use crate::draw::Draw;

pub trait Drawable {
  /// Draws and returns:
  /// - Positional offset (how much should the draw offset move)
  /// - Dimensional info (how much width+height was used)
  fn draw(&self, d : &mut Draw<'_>, pos : Vector2) -> (Vector2, Vector2);

  // TODO:
  // - add `measure` fn to get/propagate size info accurately
}

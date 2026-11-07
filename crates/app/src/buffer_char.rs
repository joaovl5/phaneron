use raylib::drawing::RaylibDraw;
use raylib::ffi::{Color, Vector2};
use raylib::text::{RaylibFont, WeakFont};

use crate::draw::Draw;
use crate::drawable::Drawable;

pub const FONT_SIZE : f32 = 40.0;
pub const FONT_SPACING : f32 = 7.0;

pub struct BufferChar<'a> {
  value : char,
  color : &'a Color,
  font : &'a WeakFont,
  font_size : &'a f32,
  font_spacing : &'a f32,
}

impl<'a> BufferChar<'a> {
  pub fn new(value : char, color : &'a Color, font : &'a WeakFont) -> Self {
    Self {
      value,
      color,
      font,
      font_size : &FONT_SIZE,
      font_spacing : &FONT_SPACING,
    }
  }
}

impl<'a> Drawable for BufferChar<'a> {
  fn draw(&self, d : &mut Draw<'_>, pos : Vector2) -> (Vector2, Vector2) {
    let value = &self.value.to_string();
    let dims = self.font.measure_text(value, *self.font_size, 0.0);
    d.handle.draw_text_codepoints(
      self.font,
      value,
      pos,
      *self.font_size,
      0.0,
      *self.color,
    );
    let offset : Vector2 = (dims.x + self.font_spacing, 0.0).into();
    (offset, (offset.x, dims.y).into())
  }
}

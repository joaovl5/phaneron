use raylib::drawing::RaylibDrawHandle;
use raylib::text::WeakFont;

pub struct Draw<'d> {
  pub handle : RaylibDrawHandle<'d>,
  pub default_font : WeakFont,
}

impl<'d> Draw<'d> {
  pub fn new(d : RaylibDrawHandle<'d>) -> Self {
    Self {
      default_font : d.get_font_default(),
      handle : d,
    }
  }
}

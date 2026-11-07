use raylib::drawing::RaylibDraw;
use raylib::ffi::{Color, KeyboardKey};
use raylib::text::WeakFont;
use raylib::{RaylibHandle, RaylibThread};

use crate::buffer::{Buffer, BufferOp};
use crate::buffer_char::BufferChar;
use crate::draw::Draw;
use crate::drawable::Drawable;
use crate::types::Direction;

pub struct App {
  pub rl : RaylibHandle,
  pub thread : RaylibThread,
}

pub const HOLD_COOLDOWN : f64 = (1.0 / 12.0);
pub const HOLD_DELAY_FACTOR : f64 = (1.0 / 6.0);

// TODO:
// - rethink how line breaks will reset the x-axis
// - think of how to structure soft line breaks (visual line breaks)
// - add modal editing backbone thing

impl App {
  pub fn new(rl : RaylibHandle, thread : RaylibThread) -> Self {
    Self { rl, thread }
  }

  pub fn run(&mut self) -> anyhow::Result<()> {
    let mut current_font : &WeakFont = &self.rl.get_font_default();
    let ops : Vec<BufferOp<'_, BufferChar>> = "Hello, world! Greetings to all \
                                               that seek greetings.\nWelcome!"
      .chars()
      .map(|c| match c {
        | '\n' => BufferOp::BreakLine,
        | _ => {
          BufferOp::Content(BufferChar::new(c, &Color::WHITE, current_font))
        },
      })
      .collect();
    let mut buffer = Buffer::new(ops);
    let mut hold_time : f64 = -HOLD_COOLDOWN;
    while !self.rl.window_should_close() {
      let mut draw = Draw::new(self.rl.begin_drawing(&self.thread));
      current_font = &draw.default_font;
      draw.handle.clear_background(Color::BLACK);
      buffer.draw(&mut draw, (0.0, 0.0).into());
      let current_time = draw.handle.get_time();
      let hold_repeat = current_time > hold_time + HOLD_COOLDOWN;
      if !hold_repeat {
        continue;
      }
      hold_time = current_time;
      if draw.handle.is_key_down(KeyboardKey::KEY_H) {
        buffer.move_caret(Direction::Left);
      }
      if draw.handle.is_key_down(KeyboardKey::KEY_L) {
        buffer.move_caret(Direction::Right);
      }
      if draw.handle.is_key_down(KeyboardKey::KEY_J) {
        buffer.move_caret(Direction::Down);
      }
      if draw.handle.is_key_down(KeyboardKey::KEY_K) {
        buffer.move_caret(Direction::Up);
      }
    }

    anyhow::Ok(())
  }
}

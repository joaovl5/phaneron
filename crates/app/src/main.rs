#![allow(unused)]
use raylib::prelude::*;

mod app;
mod buffer;
mod buffer_char;
mod draw;
mod drawable;
mod types;

use self::app::App;

fn main() -> anyhow::Result<()> {
  let (rl, thread) = raylib::init().size(1280, 960).title("Phaneron").build();

  let mut app = App { rl, thread };
  app.run()
}

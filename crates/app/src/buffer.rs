use std::cell::RefCell;
use std::collections::BTreeMap;

use raylib::drawing::RaylibDraw;
use raylib::ffi::{Color, Vector2};

use crate::draw::Draw;
use crate::drawable::Drawable;
use crate::types::Direction;

pub const LINE_BREAK_SIZE : f32 = 60.0;
pub const PAGE_BREAK_SIZE : f32 = 180.0;

pub enum BufferOp<'a, T : Drawable> {
  /// Main buffer content
  Content(T),
  /// Embedded drawable of some other kind
  EmbeddedOther(&'a dyn Drawable),
  BreakLine,
  BreakPage,
}

pub enum BufferView<'a, T : Drawable> {
  Op(&'a BufferOp<'a, T>),
  View(&'a BufferView<'a, T>),
  Pair(&'a BufferView<'a, T>, &'a BufferView<'a, T>),
}

pub struct Buffer<'a, T : Drawable> {
  pub ops : Vec<BufferOp<'a, T>>,
  pub views : Vec<BufferView<'a, T>>,
  /// Stores the caret's index of the `ops` item it belongs at.
  pub caret : usize,
  /// Stores the latest placement of the operation "draw" results.
  ops_placement : RefCell<BTreeMap<usize, (Vector2, Vector2)>>,
}

impl<'a, T : Drawable> Buffer<'a, T> {
  pub fn new(ops : Vec<BufferOp<'a, T>>) -> Self {
    Self {
      ops,
      views : Vec::new(),
      caret : 0,
      ops_placement : RefCell::new(BTreeMap::new()),
    }
  }

  /// Moves caret to the nearest object in that direction
  ///
  /// Algorithm currently approximates calculations to only deal in
  /// center-points of the relevant sides of the calculation — e.g.: if
  /// direction is "down", we use the bottom-center of the caret and the
  /// top-center of the other blocks for comparisons.
  pub fn move_caret(&mut self, direction : Direction) -> anyhow::Result<()> {
    let ops_placement = self.ops_placement.try_borrow()?;
    let Some((caret_pos, caret_dims)) = ops_placement.get(&self.caret) else {
      anyhow::bail!("`ops_placement` had no element at `{}`", self.caret);
    };
    // calculate center point based on bounds of pos → dims
    let caret_center =
      *caret_pos + (*caret_dims * direction.get_directional_center());
    // let best_index : (f32, f32, usize) = ops_placement.iter().fold(
    //   (f32::MAX, f32::MAX, self.caret),
    //   |out, (op_i, (op_pos, op_dims))| {
    //     let op_center =
    //       *op_pos + (*op_dims *
    // direction.inverse().get_directional_center());     let (along,
    // across) =       direction.get_along_across(&caret_center,
    // &op_center);     let is_better =
    //       *op_i != self.caret
    //       && along > 0.0
    //       && (along, across) < (out.0, out.1)
    //       // && along < out.0 && across < out.1
    //       ;
    //     if is_better {
    //       (along, across, *op_i)
    //     } else {
    //       out
    //     }
    //   },
    // );
    let mut best : Option<(bool, f32, f32, usize)> = None;
    for (op_i, (op_pos, op_dims)) in ops_placement.iter() {
      if *op_i == self.caret {
        continue;
      }
      let op_center_pos =
        *op_pos + *op_dims * direction.inverse().get_directional_center();
      let (along, across) =
        direction.get_along_across(&caret_center, &op_center_pos);
      if along < 0.0 {
        continue;
      }
      let overlaps =
        direction.contains_across(&caret_center, *op_pos, *op_dims);
      if best.is_none_or(|(o, a, b, _)| (!overlaps, along, across) < (o, a, b))
      {
        best = Some((overlaps, along, across, *op_i))
      }
    }
    if let Some((_, _, _, i)) = best {
      self.caret = i;
    }

    anyhow::Ok(())
  }

  fn draw_caret(&self, d : &mut Draw<'_>, pos : Vector2, dims : Vector2) {
    let dims : Vector2 = (dims.x.max(4.0), dims.y).into();
    d.handle.draw_rectangle_v(pos, dims, Color::BLUE.alpha(0.5));
  }
}
impl<'a, T : Drawable> Drawable for Buffer<'a, T> {
  fn draw(&self, d : &mut Draw<'_>, pos : Vector2) -> (Vector2, Vector2) {
    let mut offset : Vector2 = pos;
    let mut dims : Vector2 = Vector2::zero();
    // we secretly mutate state via a refcell trick, don't tell anyone!
    let mut ops_placement = self.ops_placement.borrow_mut();
    for (op_i, op) in self.ops.iter().enumerate() {
      let (op_offset, op_dims) = op.draw(d, offset);
      ops_placement.insert(op_i, (offset, op_dims));
      if op_i == self.caret || self.caret >= self.ops.len() {
        self.draw_caret(d, offset, op_dims);
      }
      offset += op_offset;
      dims += op_dims;
    }

    (offset - pos, dims)
  }
}

impl<'a, T : Drawable> Drawable for BufferOp<'a, T> {
  fn draw(&self, d : &mut Draw<'_>, pos : Vector2) -> (Vector2, Vector2) {
    match self {
      | Self::Content(value) => value.draw(d, pos),
      | Self::EmbeddedOther(value) => value.draw(d, pos),
      | Self::BreakLine => (
        (-pos.x, LINE_BREAK_SIZE).into(),
        (0.0, LINE_BREAK_SIZE).into(),
      ),
      | Self::BreakPage => (
        (-pos.x, PAGE_BREAK_SIZE).into(),
        (0.0, PAGE_BREAK_SIZE).into(),
      ),
    }
  }
}

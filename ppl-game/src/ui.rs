//! # Abstract User Interface
//! This module contain traits that describes basic game interface.
//!
//! ## Implementing
//!
//! Start from implementing 3 fragments: status(T), main(B) and lore(T). (T - text, B - block
//! types). It should implements [`Fragment`] and one of [`BlockFragment`] or [`TextFragment`].
//! Then implement [`Context`].
//!
//! ## Example usage
//! ```no_run
//! use std::fmt::Write;
//! use ppl_game::ui::{Context, Block, Fragment, BlockFragment, Point};
//! use ppl_game::ui_impls::tui; // one of implementation. feature `tui`
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut ctx = tui::Context::init()?;
//!
//!     let mut s = ctx.status();
//!     writeln!(s, "Hello world!!!!")?;
//!     writeln!(s, " {}nd line", 2)?;
//!
//!     let mut l = ctx.lore();
//!     write!(l, "Lore line ")?;
//!     writeln!(l, "1")?;
//!     writeln!(l, "Lore line 2\n")?; // skip one line
//!     writeln!(l, "Lore line 4")?;
//!
//!     let mut m = ctx.main();
//!     m.set_pos(Point(1, 1))?;
//!     m.put_block(Block::Wheat)?; // puts block W on (1; 1)
//!
//!     ctx.apply()?;
//!     // Or use ctx.transaction(|ctx| {...})
//!
//!     Ok(())
//! }
//! ```
//! ```text
//! Hello world!!!!
//!  2nd line
//! -[ ^ STATUS ]---[ v MAIN ]----|-[LORE]------
//!                               | Lore line 1
//!  W                            | Lore line 2
//!                               |
//!                               | Lore line 4
//! ```
//!

use std::fmt;

use crate::assets::maps::BlockState;

/// Represents an block.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockTy {
    Air,
    Player,

    Wheat,
    GrowingWheat,
    
    Water,
}

/// Represents text color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
// TODO: change color names to thing names (ex. BoldRed -> Health)
pub enum Color {
    Normal,
    Red,
    BoldRed,
    Green,
    Blue,
    Magenta,
    Cyan,
    Gold,
}

/// Represents a point(X; Y).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Point(pub u16, pub u16);

/// Full user interface context
pub trait Context {
    /// 2 lines fragment with status.
    type Status<'a>: TextFragment<Error = Self::Error>
    where
        Self: 'a;
    /// Main fragment, 12 lines and 30 cols.
    type Main<'a>: BlockFragment<Error = Self::Error>
    where
        Self: 'a;
    /// Lore fragment, 12 lines and about 20 cols
    type Lore<'a>: TextFragment<Error = Self::Error>
    where
        Self: 'a;

    type Error: std::error::Error;

    /// Obtain status handle.
    fn status(&mut self) -> Self::Status<'_>;
    /// Obtain main handle.
    fn main(&mut self) -> Self::Main<'_>;
    /// Obtain lore handle.
    fn lore(&mut self) -> Self::Lore<'_>;

    /// Apply all changes
    fn apply(&mut self) -> Result<(), Self::Error>;
}
/// Basic interface fragment
pub trait Fragment {
    type Error: std::error::Error;

    /// Set position to pos.
    fn set_pos(&mut self, pos: Point) -> Result<(), Self::Error>;
    /// Set position to `(0; {line})` like [`Self::set_pos`].
    fn set_line(&mut self, line: u16) -> Result<(), Self::Error> {
        self.set_pos(Point(0, line))
    }

    /// Clear all fragment and jump to (0; 0)
    fn clear(&mut self) -> Result<(), Self::Error>;
}
/// Interface [`Fragment`] that made of blocks
pub trait BlockFragment: Fragment {
    /// Put block and go to next X cord like writting one char.
    fn put_block(&mut self, block: BlockTy) -> Result<(), Self::Error>;
}
/// Interface [`Fragment`] that made of texts
///
/// # Notes
/// [`fmt::Write`] implementation for this trait must have correct
/// newlines (`\n`) handling.
// FIXME: use Fragment<Error = fmt::Error> here
pub trait TextFragment: Fragment + fmt::Write {
    /// Set text color.
    fn set_color(&mut self, color: Color) -> Result<(), fmt::Error>;
}

/// Extension for Context
pub trait ContextExt: Context {
    /// Perform (and apply) transaction.
    ///
    /// # Example
    /// ```no_run
    /// use std::fmt::Write;
    /// use ppl_game::ui::{Context, ContextExt};
    ///
    /// _ = ctx.transaction(|ctx| {
    ///     let mut l = ctx.lore();
    ///     writeln!(l, "Hello world")?;
    ///
    ///     Ok(())
    /// });
    /// ```
    fn transaction<F>(&mut self, f: F) -> Result<(), <Self as Context>::Error>
    where
        F: FnOnce(&mut Self) -> Result<(), <Self as Context>::Error> 
    {
        f(self)?;
        self.apply()
    }
}
impl<T: Context> ContextExt for T {}

pub trait BlockFragmentExt: BlockFragment {
    /// Put block by it's state like [`BlockFragment::put_block`]
    fn put_block_state(&mut self, block_state: &BlockState) -> Result<(), <Self as Fragment>::Error> {
        self.set_pos(block_state.pos)?;
        self.put_block(block_state.ty)
    }
}
impl<T: BlockFragment> BlockFragmentExt for T {}


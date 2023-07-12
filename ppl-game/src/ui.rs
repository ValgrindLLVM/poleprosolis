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
//! use ppl_game::ui::{Context, BlockTy, Fragment, BlockFragment, Point, TextFragmentFmt};
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
//!     m.put_block(BlockTy::Wheat)?; // puts block W on (1; 1)
//!
//!     ctx.apply()?;
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

use crate::things::BlockState;

/// Represents an block.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockTy {
    Air,
    
    Player,
    NPCFarmer,

    Wheat,
    GrowingWheat,

    Wall,
    WallDoor,

    Water,
    BridgeV,
    BridgeH,
}

/// Represents text color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Normal,
    RareItem,
    SpecialItem,

    GrowingWheat,
    Wheat,
    Water,

    Health,
    XP,
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
pub trait TextFragment: Fragment {
    /// Set text color.
    fn set_color(&mut self, color: Color) -> Result<(), Self::Error>;

    /// Write a string. After action cursor position can be any
    fn put_str(&mut self, s: &str) -> Result<(), Self::Error>;
}

pub trait BlockFragmentExt: BlockFragment {
    /// Put block by it's state like [`BlockFragment::put_block`]
    fn put_block_state(
        &mut self,
        block_state: &BlockState,
    ) -> Result<(), <Self as Fragment>::Error> {
        self.set_pos(block_state.pos)?;
        self.put_block(block_state.ty)
    }
}
impl<T: BlockFragment> BlockFragmentExt for T {}

/// Provides `write_fmt` method required by [`write!`]
pub trait TextFragmentFmt: TextFragment {
    /// Works like [`std::fmt::Write::write_fmt`] but saves error
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), Self::Error> {
        use fmt::Write;

        pub struct Adapter<'a, T: TextFragment + ?Sized>(&'a mut T, Option<T::Error>)
        where
            T::Error: Sized;
        impl<'a, T: TextFragment + ?Sized> fmt::Write for Adapter<'a, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.put_str(s).map_err(|e| {
                    self.1 = Some(e);
                    fmt::Error
                })
            }
        }

        let mut adapter = Adapter(self, None);
        adapter
            .write_fmt(args)
            .map_err(|_| adapter.1.expect("fmt error"))
    }
}
impl<T: TextFragment + ?Sized> TextFragmentFmt for T where T::Error: Sized {}

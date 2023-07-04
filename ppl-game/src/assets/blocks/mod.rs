use enum_dispatch::enum_dispatch;

pub mod wheat;
use wheat::Wheat;
pub mod generic;
use generic::Generic;

use crate::{
    map::{PartialBlockState, UpdateContext},
    ui::{self, Point},
};

/// Updates made by block update.
///
/// # Examples
/// ```rust
/// use ppl_game::{map::PartialBlockState, ui::Point, assets::blocks::BlockUpdates};
///
/// /// Some update function...
/// fn update() -> Result<BlockUpdates, ()> {
///     BlockUpdates::new() // creates new empty update
///         .this(PartialBlockState {
///             pos: Some(Point(10, 10)), // move self to Point(10, 10)
///             ..Default::default()      // init other fields with None
///         })                            // (don't update it)
///         .other(Point(1, 1), PartialBlockState {
///             pos: Some(Point(11, 10)), // move block at (1, 1) to
///             ..Default::default()      // (11, 10)
///         })
///         .ok() // wrap into Result::<BlockUpdates, _>::Ok(...)
/// }
///
/// let updates = update().unwrap();
/// assert_eq!(updates.this.pos, Some(Point(10, 10)));
/// assert_eq!(updates.other.len(), 1);
/// ```
#[derive(Default)]
pub struct BlockUpdates {
    /// Update on self
    pub this: PartialBlockState,
    /// Updates on others
    pub other: Vec<(Point, PartialBlockState)>,
}

impl BlockUpdates {
    /// Creates new empty update
    /// See [`BlockUpdates`] docs for examples.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder function, set `self.this` property.
    /// See [`BlockUpdates`] docs for examples.
    pub fn this(mut self, this: PartialBlockState) -> Self {
        self.this = this;
        self
    }

    /// Builder function, append other state to `self.other` vector.
    /// See [`BlockUpdates`] docs for examples.
    pub fn other(mut self, point: Point, state: PartialBlockState) -> Self {
        self.other.push((point, state));
        self
    }

    /// Builder function, append others state to `self.other` vector.
    /// See [`BlockUpdates`] docs for examples.
    pub fn others(mut self, iter: impl IntoIterator<Item = (Point, PartialBlockState)>) -> Self {
        iter.into_iter().for_each(|v| self.other.push(v));
        self
    }

    /// Wraps [`BlockUpdates`] into [`Result`]
    /// See [`BlockUpdates`] docs for examples.
    pub fn ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

/// Represents block
#[allow(unused_variables)]
#[enum_dispatch]
pub trait BlockBehavior {
    /// Update block. Called every random tick
    fn update<UI: ui::Context>(&mut self, ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        Default::default()
    }
    /// Interact with block.
    fn interact<UI: ui::Context>(&mut self, ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        Default::default()
    }
}

#[enum_dispatch(BlockBehavior)]
pub enum Block {
    Generic,
    Wheat,
}

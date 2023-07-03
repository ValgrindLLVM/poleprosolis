pub mod wheat;

use crate::ui::{self, Point};

use super::maps::{PartialBlockState, UpdateContext};

/// Updates made by block update.
#[derive(Default)]
pub struct BlockUpdates {
    /// Update on self
    pub this: PartialBlockState,
    /// Updates on others
    pub other: Vec<(Point, PartialBlockState)>,
}

/// Represents block
#[allow(unused_variables)]
pub trait Block<UI: ui::Context> {
    /// Update block. Called every random tick
    fn update(&mut self, ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        Default::default()
    }
    /// Interact with block.
    fn interact(&mut self, ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        Default::default()
    }
}

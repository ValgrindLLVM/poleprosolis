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
#[derive(Default)]
pub struct BlockUpdates {
    /// Update on self
    pub this: PartialBlockState,
    /// Updates on others
    pub other: Vec<(Point, PartialBlockState)>,
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

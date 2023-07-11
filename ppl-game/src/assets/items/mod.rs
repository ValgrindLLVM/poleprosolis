use std::borrow::Cow;

use enum_dispatch::enum_dispatch;

use crate::{things::ItemUpdateContext, ui};

pub mod bread;
use bread::Bread;

/// Updates made by item update
#[derive(Default)]
pub struct ItemUpdates;

impl ItemUpdates {
    /// Creates new empty update
    pub fn new() -> Self {
        Self
    }

    /// Wraps into [`Result`]
    pub fn ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

#[allow(unused_variables)]
#[enum_dispatch]
pub trait ItemBehavior {
    /// Item name. May be prefixed with "Rare" or other kind of [`ItemTier`]
    fn name(&self) -> Cow<'_, str>;

    /// Item meta. Shows after name without color.
    fn meta(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    /// Item color.
    fn color(&self) -> ui::Color {
        ui::Color::Normal
    }

    /// Update item. Called every random tick
    fn update<UI: ui::Context>(
        &mut self,
        ctx: ItemUpdateContext<UI>,
    ) -> Result<ItemUpdates, UI::Error> {
        ItemUpdates::new().ok()
    }
}

#[enum_dispatch(ItemBehavior)]
pub enum Item {
    Bread,
}

use std::borrow::Cow;

use crate::things::{ItemData, ItemState};

use super::{ItemBehavior, ItemUpdates};

#[derive(Clone, Debug, PartialEq)]
pub enum Bread {
    Baking {
        /// Tick of baking. Max: 3, min: 0.
        /// On tick 4 bread turning into [`Bread::Normal`].
        tick: u8,
    },
    Normal {
        /// Bread mass. Max: 255, min: 0.
        mass: u8,
    },
}

impl Bread {
    pub fn new() -> ItemData {
        ItemData {
            state: ItemState::default(),
            item: Self::Baking { tick: 0 }.into(),
        }
    }
}

impl ItemBehavior for Bread {
    fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Baking { .. } => Cow::Borrowed("Baking bread"),
            Self::Normal { .. } => Cow::Borrowed("Bread"),
        }
    }

    fn update<UI: crate::ui::Context>(
        &mut self,
        _ctx: crate::things::ItemUpdateContext<UI>,
    ) -> Result<ItemUpdates, UI::Error> {
        match self {
            Self::Baking { tick: 3 } => *self = Self::Normal { mass: 123 },
            Self::Baking { tick } => *tick += 1,
            _ => {}
        }
        Ok(ItemUpdates::new())
    }
}

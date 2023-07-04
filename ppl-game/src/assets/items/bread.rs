use super::{ItemBehavior, ItemUpdates};

#[derive(Clone, Debug, PartialEq)]
pub enum Bread {
    Cooking {
        /// Tick of cooking. Max: 3, min: 0.
        /// On tick 4 bread turning into [`Bread::Normal`].
        tick: u8,
    },
    Normal {
        /// Bread mass. Max: 255, min: 0.
        mass: u8,
    },
}

impl ItemBehavior for Bread {
    fn update<UI: crate::ui::Context>(
        &mut self,
        _ctx: crate::things::ItemUpdateContext<UI>,
    ) -> Result<ItemUpdates, UI::Error> {
        match self {
            Self::Cooking { tick: 3 } => *self = Self::Normal { mass: 123 },
            Self::Cooking { tick } => *tick += 1,
            _ => {}
        }
        Ok(ItemUpdates::new())
    }
}

use super::Tier;

#[derive(Clone, Debug, PartialEq)]
pub enum Bread {
    Cooking {
        /// Tick of cooking. Max: 3, min: 0.
        /// On tick 4 bread turning into [`Bread::Normal`].
        tick: u8,
    },
    Normal {
        /// Bread tier
        tier: Tier,
        /// Bread mass. Max: 255, min: 0.
        mass: u8,
    }
}

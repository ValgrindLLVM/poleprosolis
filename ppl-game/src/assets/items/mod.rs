pub mod bread;

/// Item tier.
// TODO: move to item.rs
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    Common,
    Uncommon,
    Rare,
    Silver,
    Gold,
    Platinum,
}

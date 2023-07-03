pub mod bread;

/// Item tier.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    Common,
    Uncommon,
    Rare,
    Silver,
    Gold,
    Platinum,
}


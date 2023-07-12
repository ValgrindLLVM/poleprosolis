//! Player information

use crate::things::{ItemData, ItemState};

/// Represents player
pub struct Player {
    pub health: u32,
    pub xp: u32,
    pub gold: u32,

    pub wheat: u32,
    pub water: u32,
}

/// Represents player's limits
pub struct PlayerLimits {
    pub health: u32,

    pub wheat: u32,
    pub water: u32,
}

impl PlayerLimits {
    /// Obtain default player's limit
    pub fn new() -> Self {
        Self {
            health: 10,
            wheat: 500,
            water: 4,
        }
    }
    /// Apply item effects on limits
    pub fn with<'a>(self, _iter: impl Iterator<Item = &'a ItemState>) -> Self {
        self
    }
}
impl Default for PlayerLimits {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    /// Creates new player.
    pub fn new() -> Self {
        Self {
            health: 10,
            xp: 1,
            gold: 0,
            wheat: 0,
            water: 0,
        }
    }
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents player's inventory
#[derive(Default)]
pub struct PlayerInventory {
    pub items: Vec<ItemData>,
}

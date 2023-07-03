//! # Player information

use crate::assets::items::bread::Bread;

/// Represents player
#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub health: u32,
    pub xp: u32,
    pub gold: u32,

    pub inventory: PlayerInventory,
}

impl Player {
    /// Creates new player.
    pub fn new() -> Self {
        Self {
            health: 10,
            xp: 1,
            gold: 0,
            inventory: Default::default(),
        }
    }
}

/// Represents player's inventory
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PlayerInventory {
    pub wheat: u32,
    pub water: u32,

    pub breads: Vec<Bread>,
}

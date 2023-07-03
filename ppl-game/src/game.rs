//! # Main game state

use std::fmt::{Write, self};

use crate::{
    assets::maps::{BlockData, CollisionTy, GameMaps, BlockState},
    player::Player,
    ui::{BlockFragment, BlockTy, Context, Fragment, Point, TextFragment, Color},
};

/// Maximum point of map
pub const MAX_POINT: Point = Point(29, 11);

/// Represent current game
pub struct Game<UI: Context> {
    pub handle: GameHandle<UI>,
    pub player_pos: Point,
    pub maps: GameMaps<UI>,
}

/// Main game handle with user interface and player information.
pub struct GameHandle<UI: Context> {
    pub ui: UI,
    pub player: Player,
    pub lore: LoreContents,
}

/// Contents of lore
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LoreContents {
    /// Nothing to display
    Nothing,
    /// Inventory
    Inventory,
    /// Endless custom content, set by some game thing.
    CustomEndless,
    /// Custom content, set by some game thing. Erases if ticks less than zero.
    Custom(u8),
}

impl<UI: Context> GameHandle<UI> {
    /// Draws player information in status like HP, XP, etc...
    pub fn draw_player_info(&mut self) -> Result<(), fmt::Error> {
        let mut s = self.ui.status();
        s.set_color(Color::Green)?;
        write!(s, "狐 ")?;
        s.set_color(Color::BoldRed)?;
        write!(s, "{} HP ", self.player.health)?;
        s.set_color(Color::Cyan)?;
        write!(s, "{} XP ", self.player.xp)?;
        s.set_color(Color::Gold)?;
        write!(s, "{}g", self.player.gold)
    }
    /// Draws (or clears) lore
    pub fn draw_lore(&mut self) -> Result<(), fmt::Error> {
        match self.lore {
            LoreContents::Nothing | LoreContents::Custom(0) => self.ui.lore().clear().unwrap(),
            LoreContents::Custom(v) => self.lore = LoreContents::Custom(v - 1),
            LoreContents::Inventory => {
                let mut l = self.ui.lore();
                l.set_color(Color::Green)?;
                writeln!(l, "INVENTORY")?;
                if self.player.inventory.wheat != 0 {
                    l.set_color(Color::Gold)?;
                    writeln!(l, "Wheat ({})", self.player.inventory.wheat)?;
                }
                if self.player.inventory.water != 0 {
                    l.set_color(Color::Cyan)?;
                    write!(l, "Bucket of water")?;
                    if self.player.inventory.water > 1 {
                        write!(l, " ({})", self.player.inventory.water)?;
                    }
                    writeln!(l)?;
                }
                l.set_color(Color::Normal)?;
            },
            _ => {},
        }

        Ok(())
    }
    
    /// Toggle inventory in lore
    pub fn toggle_inventory(&mut self) {
        match self.lore {
            LoreContents::Nothing => self.lore = LoreContents::Inventory,
            _ => self.lore = LoreContents::Nothing,
        }
    }
}

/// Action that player can do.
pub enum GameAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Interact,
}

impl<UI: Context> Game<UI> {
    /// Creates new game and init map.
    pub fn new(ui: UI) -> Self {
        Self {
            handle: GameHandle {
                ui,
                player: Player::new(),
                lore: LoreContents::Nothing,
            },
            player_pos: Point(0, 0),
            maps: GameMaps::init(),
        }
    }

    /// Redraw all blocks on the map
    pub fn redraw_all(&mut self) -> Result<(), UI::Error> {
        let mut m = self.handle.ui.main();
        for block in self.maps.get_current() {
            m.set_pos(block.state.pos)?;
            m.put_block(block.state.ty)?;
        }
        m.set_pos(self.player_pos)?;
        m.put_block(BlockTy::Player)
    }

    /// Do [`GameAction`]
    pub fn do_action(&mut self, act: GameAction) -> Result<(), UI::Error> {
        use GameAction::*;
        match act {
            MoveUp | MoveDown | MoveLeft | MoveRight => 'brk: {
                let mut pos = self.player_pos;
                let old_pos = pos;
                match act {
                    MoveUp if pos.1 != 0 => pos.1 -= 1,
                    MoveDown if pos.1 != MAX_POINT.1 => pos.1 += 1,
                    MoveLeft if pos.0 != 0 => pos.0 -= 1,
                    MoveRight if pos.0 != MAX_POINT.0 => pos.0 += 1,
                    _ => break 'brk,
                };
                if let Some(BlockData {
                    state: BlockState {
                        collision: CollisionTy::Collision,
                        ..
                    },
                    ..
                }) = self.maps.find_at(pos)
                {
                    break 'brk;
                }
                self.player_pos = pos;
                self.maps.do_random_tick(&mut self.handle)?;
                let mut m = self.handle.ui.main();
                m.set_pos(old_pos)?;
                m.put_block(self.maps.find_at(old_pos).map_or(BlockTy::Air, |f| f.state.ty))?;
                m.set_pos(pos)?;
                m.put_block(BlockTy::Player)?;
            }
            Interact => {
                self.maps.interact_at(self.player_pos, &mut self.handle);
            }
        }
        Ok(())
    }
}

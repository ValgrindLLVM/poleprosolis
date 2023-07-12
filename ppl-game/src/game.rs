//! # Main game state

use rand::{thread_rng, Rng};

use crate::{
    assets::items::ItemBehavior,
    map::GameMaps,
    player::{Player, PlayerInventory},
    things::{BlockData, BlockState, CollisionTy, ItemData, ItemTier, ItemUpdateContext},
    ui::{BlockFragment, BlockTy, Color, Context, Fragment, Point, TextFragment, TextFragmentFmt},
};

/// Maximum point of map
pub const MAX_POINT: Point = Point(29, 11);

/// Represent current game
pub struct Game<UI: Context> {
    pub handle: GameHandle<UI>,
    pub player_inventory: PlayerInventory,
    pub player_pos: Point,
    pub maps: GameMaps,
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
    /// Items with page
    Items(u8),
    /// Endless custom content, set by some game thing.
    CustomEndless,
    /// Custom content, set by some game thing. Erases if ticks less than zero.
    Custom(u8),
}

impl<UI: Context> GameHandle<UI> {
    /// Draws player information in status like HP, XP, etc...
    pub fn draw_player_info(&mut self) -> Result<(), UI::Error> {
        let mut s = self.ui.status();
        s.set_color(Color::GrowingWheat)?;
        write!(s, "狐 ")?;
        s.set_color(Color::Health)?;
        write!(s, "{} HP ", self.player.health)?;
        s.set_color(Color::XP)?;
        write!(s, "{:3} XP ", self.player.xp)?;
        s.set_color(Color::Gold)?;
        write!(s, "{:4}g", self.player.gold)
    }
    /// Draws (or clears) lore
    pub fn draw_lore(&mut self, inventory: &PlayerInventory) -> Result<(), UI::Error> {
        self.ui.lore().clear()?;
        match self.lore {
            LoreContents::Nothing | LoreContents::Custom(0) | LoreContents::CustomEndless => {}
            LoreContents::Custom(v) => self.lore = LoreContents::Custom(v - 1),
            LoreContents::Inventory => {
                let mut l = self.ui.lore();
                l.set_color(Color::GrowingWheat)?;
                writeln!(l, "INVENTORY")?;
                if self.player.wheat != 0 {
                    l.set_color(Color::Gold)?;
                    writeln!(l, "Wheat ({})", self.player.wheat)?;
                }
                if self.player.water != 0 {
                    l.set_color(Color::XP)?;
                    write!(l, "Bucket of water")?;
                    if self.player.water > 1 {
                        write!(l, " ({})", self.player.water)?;
                    }
                    writeln!(l)?;
                }
                l.set_color(Color::Normal)?;
            }
            LoreContents::Items(page) => {
                let mut l = self.ui.lore();
                l.set_color(Color::GrowingWheat)?;
                write!(l, "INVENTORY")?;
                l.set_color(Color::Normal)?;
                writeln!(l, " page #{}", page as u16 + 1)?;
                writeln!(l)?;
                let items = inventory
                    .items
                    .iter()
                    .enumerate()
                    .skip(page as usize * 9)
                    .take(9);
                for (no, item) in items {
                    write!(l, "{}. ", no + 1)?;
                    l.set_color(item.item.color())?;
                    write!(l, "{}", item.item.name())?;
                    if item.state.tier != ItemTier::Common {
                        l.set_color(Color::Normal)?;
                        write!(l, " ")?;
                        item.state.tier.suffix(&mut l)?;
                    }
                    l.set_color(Color::Normal)?;
                    let meta = item.item.meta();
                    if !meta.is_empty() {
                        write!(l, " ({meta})")?;
                    }
                    writeln!(l)?;
                }
            }
        }

        Ok(())
    }

    /// Toggle inventory in lore
    pub fn toggle_inventory(&mut self) {
        match self.lore {
            LoreContents::Inventory => self.lore = LoreContents::Nothing,
            _ => self.lore = LoreContents::Inventory,
        }
    }

    /// Show next (or first) page of items in lore
    pub fn toggle_items(&mut self) {
        match &mut self.lore {
            LoreContents::Items(v) => *v = v.saturating_add(1),
            _ => self.lore = LoreContents::Items(0),
        }
    }

    /// Do random tick that updates all items. Call it on interval or on player move, etc...
    /// It updates only items, use [`Game::do_random_tick`] to update all things.
    pub fn do_random_tick(&mut self, inventory: &mut PlayerInventory) -> Result<(), UI::Error> {
        for ItemData { state, item } in &mut inventory.items {
            if thread_rng().gen_range(0..100) >= 15 {
                continue;
            }
            let update = ItemUpdateContext {
                game_handle: self,
                this: state,
            };
            let _updates = item.update(update)?;
        }
        Ok(())
    }
}

/// Action that player can do.
pub enum GameAction {
    /// Move player one block up (-1 by Y)
    MoveUp,
    /// Move player one block down (+1 by Y)
    MoveDown,
    /// Move player one block left (-1 by X)
    MoveLeft,
    /// Move player one block right (+1 by X)
    MoveRight,
    /// Interact with block at player position
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
            player_inventory: Default::default(),
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

    /// Draw (or clear) lore
    pub fn draw_lore(&mut self) -> Result<(), UI::Error> {
        self.handle.draw_lore(&self.player_inventory)
    }

    /// Do random tick that updates all things. It automaticly calls on player move, etc...
    pub fn do_random_tick(&mut self) -> Result<(), UI::Error> {
        self.maps
            .do_random_tick(&mut self.handle, &mut self.player_inventory)?;
        self.handle.do_random_tick(&mut self.player_inventory)
    }

    fn update_status_if_needed(&mut self) -> Result<(), UI::Error> {
        let pos = self.player_pos;
        if let Some(BlockData {
            state:
                BlockState {
                    collision: CollisionTy::CanUse,
                    ..
                },
            ..
        }) = self.maps.find_at(pos)
        {
            let mut s = self.handle.ui.status();
            s.set_line(1)?;
            s.set_color(Color::Water)?;
            write!(s, " [can use]")
        } else {
            let mut s = self.handle.ui.status();
            // FIXME: clear one line, not a whole fragment
            s.clear()
        }
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
                    state:
                        BlockState {
                            collision: CollisionTy::Collision,
                            ..
                        },
                    ..
                }) = self.maps.find_at(pos)
                {
                    break 'brk;
                }
                self.player_pos = pos;
                self.do_random_tick()?;
                let mut m = self.handle.ui.main();
                m.set_pos(old_pos)?;
                m.put_block(
                    self.maps
                        .find_at(old_pos)
                        .map_or(BlockTy::Air, |f| f.state.ty),
                )?;
                m.set_pos(pos)?;
                m.put_block(BlockTy::Player)?;
                drop(m);
                self.update_status_if_needed()?;
            }
            Interact => {
                self.maps.interact_at(
                    self.player_pos,
                    &mut self.handle,
                    &mut self.player_inventory,
                )?;
                self.update_status_if_needed()?;
            }
        }
        Ok(())
    }
}

use rand::{thread_rng, Rng};

use crate::{
    assets::{blocks::BlockBehavior, maps::GameMap},
    game::GameHandle,
    player::PlayerInventory,
    things::{BlockData, BlockState, BlockUpdateContext},
    ui::{self, BlockFragmentExt, Point},
};

/// Game maps. Some maps may be not init. Default map is [`GameMap::Farm`]
pub struct GameMaps {
    pub current_map: GameMap,
    pub farm: Vec<BlockData>,
}

impl GameMaps {
    /// Initialize default game map
    pub fn init() -> Self {
        Self {
            current_map: GameMap::Farm,
            farm: GameMap::Farm.init(),
        }
    }

    /// Get current map (blocks on it)
    pub fn get_current(&self) -> &Vec<BlockData> {
        match self.current_map {
            GameMap::Farm => &self.farm,
        }
    }
    /// Get current map (blocks on it)
    pub fn get_current_mut(&mut self) -> &mut Vec<BlockData> {
        match self.current_map {
            GameMap::Farm => &mut self.farm,
        }
    }

    /// Find block by position
    pub fn find_at(&self, pos: Point) -> Option<&BlockData> {
        self.get_current().iter().find(|p| p.state.pos == pos)
    }
    /// Find block by position
    pub fn find_at_mut(&mut self, pos: Point) -> Option<&mut BlockData> {
        self.get_current_mut()
            .iter_mut()
            .find(|p| p.state.pos == pos)
    }

    /// Do random tick that updates all blocks. Call it on interval or on player move, etc...
    /// It updates only blocks, use [`Game::do_random_tick`] to update all things.
    pub fn do_random_tick<UI: ui::Context>(
        &mut self,
        game_handle: &mut GameHandle<UI>,
        player_inventory: &mut PlayerInventory,
    ) -> Result<(), UI::Error> {
        let mut other_updates = Vec::new();
        let mut states: Vec<&BlockState> = Vec::new();
        for BlockData { state, block } in self.get_current_mut().iter_mut() {
            if thread_rng().gen_range(0..100) >= 15 {
                continue;
            }
            let update = BlockUpdateContext {
                game_handle,
                this: state,
                player_inventory,
            };
            let updates = block.update(update)?;
            states.push(state);
            other_updates.push(updates.other);
        }
        let mut m = game_handle.ui.main();
        for state in states {
            m.put_block_state(state)?;
        }
        for (p, s) in other_updates.into_iter().flatten() {
            if let Some(data) = self.find_at_mut(p) {
                data.state.merge_with(s);
                m.put_block_state(&data.state)?;
            }
        }
        Ok(())
    }

    /// Interact with block by it's position
    pub fn interact_at<UI: ui::Context>(
        &mut self,
        pos: Point,
        game_handle: &mut GameHandle<UI>,
        player_inventory: &mut PlayerInventory,
    ) -> Result<(), UI::Error> {
        let data = self
            .get_current_mut()
            .iter_mut()
            .find(|p| p.state.pos == pos);
        if let Some(BlockData { state, block }) = data {
            let update = BlockUpdateContext {
                game_handle,
                this: state,
                player_inventory,
            };
            let updates = block.interact(update)?;
            updates.other.into_iter().for_each(|(p, s)| {
                if let Some(data) = self.find_at_mut(p) {
                    data.state.merge_with(s);
                }
            })
        }
        Ok(())
    }
}

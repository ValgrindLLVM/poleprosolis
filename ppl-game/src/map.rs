use rand::{thread_rng, Rng};

use crate::{
    assets::{
        blocks::{Block, BlockBehavior},
        maps::GameMap,
    },
    game::GameHandle,
    ui::{self, BlockFragmentExt, BlockTy, Point},
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
    pub fn do_random_tick<UI: ui::Context>(
        &mut self,
        game_handle: &mut GameHandle<UI>,
    ) -> Result<(), UI::Error> {
        let mut other_updates = Vec::new();
        let mut states: Vec<&BlockState> = Vec::new();
        for BlockData { state, block } in self.get_current_mut().iter_mut() {
            if thread_rng().gen_range(0..100) >= 15 {
                continue;
            }
            let updates = {
                let mut update = UpdateContext::new(game_handle);
                block.update(&mut update)
            };
            state.merge_with(updates.this);
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
    pub fn interact_at<UI: ui::Context>(&mut self, pos: Point, game_handle: &mut GameHandle<UI>) {
        let data = self
            .get_current_mut()
            .iter_mut()
            .find(|p| p.state.pos == pos);
        if let Some(BlockData { state, block }) = data {
            let mut update = UpdateContext::new(game_handle);
            let updates = block.interact(&mut update);
            state.merge_with(updates.this);
            updates.other.into_iter().for_each(|(p, s)| {
                if let Some(data) = self.find_at_mut(p) {
                    data.state.merge_with(s);
                }
            })
        }
    }
}

/// Block update/interact/etc context
pub struct UpdateContext<'a, UI: ui::Context> {
    pub game_handle: &'a mut GameHandle<UI>,
}

impl<'a, UI: ui::Context> UpdateContext<'a, UI> {
    pub fn new(game_handle: &'a mut GameHandle<UI>) -> Self {
        Self { game_handle }
    }
}

/// Block state
pub struct BlockState {
    pub pos: Point,
    pub collision: CollisionTy,
    pub ty: BlockTy,
}
/// Partial block state. Can be merged into [`BlockState`] using [`BlockState::merge_with`]
#[derive(Default)]
pub struct PartialBlockState {
    pub pos: Option<Point>,
    pub collision: Option<CollisionTy>,
    pub ty: Option<BlockTy>,
}

impl BlockState {
    pub fn merge_with(&mut self, partial: PartialBlockState) {
        if let Some(pos) = partial.pos {
            self.pos = pos
        }
        if let Some(collision) = partial.collision {
            self.collision = collision
        }
        if let Some(ty) = partial.ty {
            self.ty = ty
        }
    }
}

/// Full block data in map
pub struct BlockData {
    /// Block generic state
    pub state: BlockState,
    /// Local block state and behavior
    pub block: Block,
}
/// Type of collision
pub enum CollisionTy {
    /// Player can move into block
    NoCollision,
    /// Player can move into block and interact with it
    CanUse,
    /// Player can't move into block
    Collision,
}

impl BlockData {
    /// Creates new block
    pub fn new(pos: Point, collision: CollisionTy, ty: BlockTy, block: Block) -> Self {
        Self {
            state: BlockState { pos, collision, ty },
            block,
        }
    }
}

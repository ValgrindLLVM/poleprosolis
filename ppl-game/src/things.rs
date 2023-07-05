//! # Game things (items and blocks)
//!
//! All things have state and data structs. Also it have update context with game handle and etc.
//!
//! | Name           | Item Type              | Block Type             |
//! |----------------|------------------------|------------------------|
//! | State          | [`ItemState`]          | [`BlockState`]         |
//! | Inner data     | [`Item`]               | [`Block`]              |
//! | Data           | [`ItemData`]           | [`BlockData`]          |
//! | Update context | [`ItemUpdateContext`]  | [`BlockUpdateContext`] |

use crate::{
    assets::{blocks::Block, items::Item},
    game::GameHandle,
    player::PlayerInventory,
    ui::{self, BlockTy, Point},
};

/// Thing update/interact/etc context
pub struct UpdateContext<'a, UI: ui::Context, T> {
    pub game_handle: &'a mut GameHandle<UI>,
    pub this: &'a mut T,
}
/// Item update/interact/etc context
pub type ItemUpdateContext<'a, UI> = UpdateContext<'a, UI, ItemState>;

/// Thing update/interact/etc context
pub struct BlockUpdateContext<'a, UI: ui::Context> {
    pub game_handle: &'a mut GameHandle<UI>,
    pub player_inventory: &'a mut PlayerInventory,
    pub this: &'a mut BlockState,
}

impl<'a, UI: ui::Context, T> UpdateContext<'a, UI, T> {
    pub fn new(game_handle: &'a mut GameHandle<UI>, this: &'a mut T) -> Self {
        Self { game_handle, this }
    }
}

/// Tier of item.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemTier {
    Common,
    Uncommon,
    Rare,
    Silver,
    Gold,
    Platinum,
}
/// Item state
pub struct ItemState {
    pub tier: ItemTier,
}
pub struct ItemData {
    pub state: ItemState,
    pub item: Item,
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
    /// Merges with partial state.
    ///
    /// # Example
    /// ```
    /// use ppl_game::{things::{BlockState, PartialBlockState, CollisionTy}, ui::{BlockTy, Point}};
    ///
    /// let mut block = BlockState {
    ///     ty: BlockTy::GrowingWheat,
    ///     collision: CollisionTy::NoCollision,
    ///     pos: Point(1, 1),
    /// };
    /// let partial = PartialBlockState {
    ///     ty: Some(BlockTy::Wheat),
    ///     ..Default::default()
    /// };
    ///
    /// block.merge_with(partial);
    /// assert_eq!(block.ty, BlockTy::Wheat);
    /// assert_eq!(block.pos, Point(1, 1));
    /// ```
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

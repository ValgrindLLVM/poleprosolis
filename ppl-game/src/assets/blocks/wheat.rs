use crate::{
    assets::maps::{CollisionTy, PartialBlockState, UpdateContext},
    ui::{self, BlockTy},
};

use super::{Block, BlockUpdates};

pub struct Wheat {
    tick: u8,
}

impl Wheat {
    pub fn new() -> Wheat {
        Wheat { tick: 0 }
    }
}

impl<UI: ui::Context> Block<UI> for Wheat {
    fn update(&mut self, _ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        if self.tick == 0 {
            return Default::default();
        }
        self.tick -= 1;

        if self.tick == 0 {
            BlockUpdates {
                this: PartialBlockState {
                    ty: Some(BlockTy::Wheat),
                    collision: Some(CollisionTy::CanUse),
                    ..Default::default()
                },
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }

    fn interact(&mut self, ctx: &mut UpdateContext<'_, UI>) -> BlockUpdates {
        if self.tick != 0 {
            return Default::default();
        }
        ctx.game_handle.player.inventory.wheat += 10;
        self.tick = 4;

        BlockUpdates {
            this: PartialBlockState {
                collision: Some(CollisionTy::NoCollision),
                ty: Some(BlockTy::GrowingWheat),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

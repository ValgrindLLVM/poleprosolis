use crate::{
    assets::blocks::{BlockBehavior, BlockUpdates},
    things::{BlockUpdateContext, CollisionTy, PartialBlockState},
    ui::{self, BlockTy},
};

pub struct Wheat {
    tick: u8,
}

impl Wheat {
    pub fn new() -> Wheat {
        Wheat { tick: 0 }
    }
}

impl BlockBehavior for Wheat {
    fn update<UI: ui::Context>(&mut self, _ctx: &mut BlockUpdateContext<'_, UI>) -> BlockUpdates {
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

    fn interact<UI: ui::Context>(&mut self, ctx: &mut BlockUpdateContext<'_, UI>) -> BlockUpdates {
        if self.tick != 0 {
            return Default::default();
        }
        ctx.game_handle.player.wheat += 10;
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

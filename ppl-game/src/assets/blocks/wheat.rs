use crate::{
    assets::{
        blocks::{BlockBehavior, BlockUpdates},
        items::bread::Bread,
    },
    things::{BlockUpdateContext, CollisionTy},
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
    fn update<UI: ui::Context>(
        &mut self,
        ctx: BlockUpdateContext<'_, UI>,
    ) -> Result<BlockUpdates, UI::Error> {
        if self.tick != 0 {
            self.tick -= 1;

            if self.tick == 0 {
                ctx.this.ty = BlockTy::Wheat;
                ctx.this.collision = CollisionTy::CanUse;
            }
        }
        BlockUpdates::new().ok()
    }

    fn interact<UI: ui::Context>(
        &mut self,
        ctx: BlockUpdateContext<'_, UI>,
    ) -> Result<BlockUpdates, UI::Error> {
        if self.tick == 0 {
            ctx.game_handle.player.wheat += 10;
            self.tick = 4;

            ctx.this.ty = BlockTy::GrowingWheat;
            ctx.this.collision = CollisionTy::NoCollision;

            ctx.player_inventory.items.push(Bread::new());
        }

        BlockUpdates::new().ok()
    }
}

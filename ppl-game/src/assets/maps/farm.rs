use crate::{
    assets::blocks::wheat::Wheat,
    ui::{self, Point},
};

use super::{BlockData, CollisionTy};

#[rustfmt::skip]
pub fn init<UI: ui::Context>() -> Vec<BlockData<UI>> {
    vec![
        BlockData::new(Point(1, 1), CollisionTy::CanUse, crate::ui::BlockTy::Wheat, Some(Box::new(Wheat::new()))),
        BlockData::new(Point(2, 1), CollisionTy::CanUse, crate::ui::BlockTy::Wheat, Some(Box::new(Wheat::new()))),
        BlockData::new(Point(3, 1), CollisionTy::CanUse, crate::ui::BlockTy::Wheat, Some(Box::new(Wheat::new()))),
        BlockData::new(Point(4, 1), CollisionTy::CanUse, crate::ui::BlockTy::Wheat, Some(Box::new(Wheat::new()))),
        BlockData::new(Point(5, 1), CollisionTy::CanUse, crate::ui::BlockTy::Wheat, Some(Box::new(Wheat::new()))),

        BlockData::new(Point(1, 2), CollisionTy::Collision, crate::ui::BlockTy::Water, None),
        BlockData::new(Point(2, 2), CollisionTy::Collision, crate::ui::BlockTy::Water, None),
        BlockData::new(Point(3, 2), CollisionTy::Collision, crate::ui::BlockTy::Water, None),
        BlockData::new(Point(4, 2), CollisionTy::Collision, crate::ui::BlockTy::Water, None),
        BlockData::new(Point(5, 2), CollisionTy::Collision, crate::ui::BlockTy::Water, None),
    ]
}

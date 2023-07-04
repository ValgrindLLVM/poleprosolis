use crate::{
    assets::blocks::{generic::Generic, wheat::Wheat},
    map::{BlockData, CollisionTy},
    ui::{BlockTy, Point},
};

#[rustfmt::skip]
pub fn init() -> Vec<BlockData> {
    vec![
        BlockData::new(Point(1, 1), CollisionTy::CanUse, BlockTy::Wheat, Wheat::new().into()),
        BlockData::new(Point(2, 1), CollisionTy::CanUse, BlockTy::Wheat, Wheat::new().into()),
        BlockData::new(Point(3, 1), CollisionTy::CanUse, BlockTy::Wheat, Wheat::new().into()),
        BlockData::new(Point(4, 1), CollisionTy::CanUse, BlockTy::Wheat, Wheat::new().into()),
        BlockData::new(Point(5, 1), CollisionTy::CanUse, BlockTy::Wheat, Wheat::new().into()),

        BlockData::new(Point(1, 2), CollisionTy::Collision, BlockTy::Water, Generic.into()),
        BlockData::new(Point(2, 2), CollisionTy::Collision, BlockTy::Water, Generic.into()),
        BlockData::new(Point(3, 2), CollisionTy::Collision, BlockTy::Water, Generic.into()),
        BlockData::new(Point(4, 2), CollisionTy::Collision, BlockTy::Water, Generic.into()),
        BlockData::new(Point(5, 2), CollisionTy::Collision, BlockTy::Water, Generic.into()),
    ]
}

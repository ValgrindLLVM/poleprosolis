use crate::{
    assets::blocks::{generic::Generic, wheat::Wheat, Block},
    map::{BlockData, CollisionTy},
    ui::{BlockTy, Point},
};

#[rustfmt::skip]
pub fn init() -> Vec<BlockData> {
    vec![
        BlockData::new(Point(1, 1), CollisionTy::CanUse, BlockTy::Wheat, Block::Wheat(Wheat::new())),
        BlockData::new(Point(2, 1), CollisionTy::CanUse, BlockTy::Wheat, Block::Wheat(Wheat::new())),
        BlockData::new(Point(3, 1), CollisionTy::CanUse, BlockTy::Wheat, Block::Wheat(Wheat::new())),
        BlockData::new(Point(4, 1), CollisionTy::CanUse, BlockTy::Wheat, Block::Wheat(Wheat::new())),
        BlockData::new(Point(5, 1), CollisionTy::CanUse, BlockTy::Wheat, Block::Wheat(Wheat::new())),

        BlockData::new(Point(1, 2), CollisionTy::Collision, BlockTy::Water, Block::Generic(Generic)),
        BlockData::new(Point(2, 2), CollisionTy::Collision, BlockTy::Water, Block::Generic(Generic)),
        BlockData::new(Point(3, 2), CollisionTy::Collision, BlockTy::Water, Block::Generic(Generic)),
        BlockData::new(Point(4, 2), CollisionTy::Collision, BlockTy::Water, Block::Generic(Generic)),
        BlockData::new(Point(5, 2), CollisionTy::Collision, BlockTy::Water, Block::Generic(Generic)),
    ]
}

use crate::map::BlockData;

pub mod farm;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GameMap {
    #[default]
    Farm,
}
impl GameMap {
    pub fn init(self) -> Vec<BlockData> {
        match self {
            GameMap::Farm => farm::init(),
        }
    }
}

use super::terrain::{generator::TerrainGenerator, Terrain};

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn from(pos: &TilePosition, x_off: i32, y_off: i32) -> Self {
        return Self {
            x: pos.x + x_off,
            y: pos.y + y_off,
        };
    }
}

impl Into<TilePosition> for (i32, i32) {
    fn into(self) -> TilePosition {
        TilePosition {
            x: self.0,
            y: self.1,
        }
    }
}

type TilePosTerrainTuple = (Terrain, TilePosition);

pub struct TilePositionNeighbors {
    pub top_left: TilePosTerrainTuple,
    pub top: TilePosTerrainTuple,
    pub top_right: TilePosTerrainTuple,
    pub left: TilePosTerrainTuple,
    pub center: TilePosTerrainTuple,
    pub right: TilePosTerrainTuple,
    pub bottom_left: TilePosTerrainTuple,
    pub bottom: TilePosTerrainTuple,
    pub bottom_right: TilePosTerrainTuple,
}

impl TilePositionNeighbors {
    pub fn new(gen: &TerrainGenerator, you: TilePosition) -> Self {
        let top_left = TilePosition::from(&you, -1, 1);
        let top = TilePosition::from(&you, 0, 1);
        let top_right = TilePosition::from(&you, 1, 1);

        let left = TilePosition::from(&you, -1, 0);
        let right = TilePosition::from(&you, 1, 0);

        let bottom_left = TilePosition::from(&you, -1, -1);
        let bottom = TilePosition::from(&you, 0, -1);
        let bottom_right = TilePosition::from(&you, 1, -1);

        Self {
            top_left: (gen.get_terrain(&top_left), top_left),
            top: (gen.get_terrain(&top), top),
            top_right: (gen.get_terrain(&top_right), top_right),
            left: (gen.get_terrain(&left), left),
            right: (gen.get_terrain(&right), right),
            bottom_left: (gen.get_terrain(&bottom_left), bottom_left),
            bottom: (gen.get_terrain(&bottom), bottom),
            bottom_right: (gen.get_terrain(&bottom_right), bottom_right),
            center: (gen.get_terrain(&you), you),
        }
    }
}

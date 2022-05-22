#[derive(Hash, PartialEq, Eq, Clone)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl Into<TilePosition> for (i32, i32) {
    fn into(self) -> TilePosition {
        TilePosition {
            x: self.0,
            y: self.1,
        }
    }
}

pub struct TileNeighbors {
    pub top_left: TilePosition,
    pub top: TilePosition,
    pub top_right: TilePosition,
    pub left: TilePosition,
    pub center: TilePosition,
    pub right: TilePosition,
    pub bottom_left: TilePosition,
    pub bottom: TilePosition,
    pub bottom_right: TilePosition,
}

impl TileNeighbors {
    pub fn new(you: TilePosition) -> Self {
        Self {
            top_left: TilePosition {
                x: you.x - 1,
                y: you.y + 1,
            },
            top: TilePosition {
                x: you.x,
                y: you.y + 1,
            },
            top_right: TilePosition {
                x: you.x + 1,
                y: you.y + 1,
            },
            left: TilePosition {
                x: you.x - 1,
                y: you.y,
            },
            right: TilePosition {
                x: you.x + 1,
                y: you.y,
            },
            bottom_left: TilePosition {
                x: you.x - 1,
                y: you.y - 1,
            },
            bottom: TilePosition {
                x: you.x,
                y: you.y - 1,
            },
            bottom_right: TilePosition {
                x: you.x + 1,
                y: you.y - 1,
            },
            center: you,
        }
    }
}

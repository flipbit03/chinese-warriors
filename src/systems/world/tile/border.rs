use super::{position::TilePositionNeighbors, terrain::Terrain};

#[derive(Clone, Debug)]
pub enum TileBorderType {
    VerticalWall = 1,
    HorizontalWall = 2,
    InnerCorner = 4,
    OuterCorner = 8,
}

#[derive(Clone, Debug)]
pub struct TileBorder {
    pub terrain: Terrain,
    pub upper_left: Option<TileBorderType>,
    pub upper_right: Option<TileBorderType>,
    pub bottom_left: Option<TileBorderType>,
    pub bottom_right: Option<TileBorderType>,
}

fn get_border_from_neighbor_effects(
    horizontal_neighbor: bool,
    vertical_neighbor: bool,
    diagonal_neighbor: bool,
) -> Option<TileBorderType> {
    match (horizontal_neighbor, vertical_neighbor, diagonal_neighbor) {
        (true, true, true) | (true, true, false) => Some(TileBorderType::InnerCorner),
        (true, false, true) | (true, false, false) => Some(TileBorderType::VerticalWall),
        (false, true, true) | (false, true, false) => Some(TileBorderType::HorizontalWall),
        (false, false, true) => Some(TileBorderType::OuterCorner),
        (false, false, false) => None,
    }
}

impl TileBorder {
    pub fn from(matrix: TilePositionNeighbors) -> Vec<Self> {
        let center = matrix.center.0 as usize;

        let stronger_than_me = center + 1;

        if stronger_than_me == Terrain::VARIANT_COUNT {
            return Vec::new();
        }

        let left = matrix.left.0 as usize;
        let top = matrix.top.0 as usize;
        let right = matrix.right.0 as usize;
        let bottom = matrix.bottom.0 as usize;

        let top_left = matrix.top_left.0 as usize;
        let top_right = matrix.top_right.0 as usize;
        let bottom_left = matrix.bottom_left.0 as usize;
        let bottom_right = matrix.bottom_right.0 as usize;

        (stronger_than_me..Terrain::VARIANT_COUNT)
            .map(|terrain| {
                let upper_left = get_border_from_neighbor_effects(
                    left == terrain,
                    top == terrain,
                    top_left == terrain,
                );

                let upper_right = get_border_from_neighbor_effects(
                    right == terrain,
                    top == terrain,
                    top_right == terrain,
                );

                let bottom_left = get_border_from_neighbor_effects(
                    left == terrain,
                    bottom == terrain,
                    bottom_left == terrain,
                );

                let bottom_right = get_border_from_neighbor_effects(
                    right == terrain,
                    bottom == terrain,
                    bottom_right == terrain,
                );

                TileBorder {
                    terrain: terrain.into(),
                    upper_left,
                    upper_right,
                    bottom_left,
                    bottom_right,
                }
            })
            .collect()
    }

    pub fn get_texture_indexes(&self) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();

        if let Some(border) = &self.upper_left {
            ret.push(match border {
                TileBorderType::HorizontalWall => 0,
                TileBorderType::VerticalWall => 7,
                TileBorderType::InnerCorner => 8,
                TileBorderType::OuterCorner => 12,
            });
        }

        if let Some(border) = &self.upper_right {
            ret.push(match border {
                TileBorderType::HorizontalWall => 1,
                TileBorderType::VerticalWall => 2,
                TileBorderType::InnerCorner => 9,
                TileBorderType::OuterCorner => 13,
            });
        };

        if let Some(border) = &self.bottom_right {
            ret.push(match border {
                TileBorderType::HorizontalWall => 4,
                TileBorderType::VerticalWall => 3,
                TileBorderType::InnerCorner => 10,
                TileBorderType::OuterCorner => 14,
            });
        };

        if let Some(border) = &self.bottom_left {
            ret.push(match border {
                TileBorderType::HorizontalWall => 5,
                TileBorderType::VerticalWall => 6,
                TileBorderType::InnerCorner => 11,
                TileBorderType::OuterCorner => 15,
            });
        };

        ret
    }
}

use strum::EnumCount;

use self::{
    neighbor::get_border_from_neighbor_effects,
    structs::{
        BorderSpec, DiagonalLocation, OpenEnd, TileBorder, TileBorderType, TunnelOrientation,
    },
};

use super::{position::TilePositionNeighbors, terrain::BaseTerrain};

pub mod neighbor;
pub mod structs;

impl TileBorder {
    pub fn from(matrix: TilePositionNeighbors) -> Vec<Self> {
        let cloned = matrix.clone();

        let center = cloned.center.0.base as usize;
        let stronger_than_me = center + 1;

        if stronger_than_me == BaseTerrain::COUNT {
            return Vec::new();
        }

        let left = cloned.left.0.base as usize;
        let top = cloned.top.0.base as usize;
        let right = cloned.right.0.base as usize;
        let bottom = cloned.bottom.0.base as usize;

        let top_left = cloned.top_left.0.base as usize;
        let top_right = cloned.top_right.0.base as usize;
        let bottom_left = cloned.bottom_left.0.base as usize;
        let bottom_right = cloned.bottom_right.0.base as usize;

        (stronger_than_me..BaseTerrain::COUNT)
            .map(|terrain| {
                let tileborder_from_4: Option<TileBorder> =
                    get_tileborder_from_terrain(terrain, matrix.clone());

                if let Some(border) = tileborder_from_4 {
                    return border;
                };

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

                let borders = [upper_left, upper_right, bottom_right, bottom_left];

                TileBorder {
                    terrain: terrain.into(),
                    spec: borders.into(),
                }
            })
            .collect()
    }

    pub fn get_texture_indexes(&self) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();

        if let Some(border) = &self.spec.upper_left {
            ret.push(match border {
                TileBorderType::HorizontalWall => 0,
                TileBorderType::VerticalWall => 7,
                TileBorderType::InnerCorner => 8,
                TileBorderType::OuterCorner => 12,
                TileBorderType::Diagonal => 18,
            });
        }

        if let Some(border) = &self.spec.upper_right {
            ret.push(match border {
                TileBorderType::HorizontalWall => 1,
                TileBorderType::VerticalWall => 2,
                TileBorderType::InnerCorner => 9,
                TileBorderType::OuterCorner => 13,
                TileBorderType::Diagonal => 19,
            });
        };

        if let Some(border) = &self.spec.bottom_right {
            ret.push(match border {
                TileBorderType::HorizontalWall => 4,
                TileBorderType::VerticalWall => 3,
                TileBorderType::InnerCorner => 10,
                TileBorderType::OuterCorner => 14,
                TileBorderType::Diagonal => 16,
            });
        };

        if let Some(border) = &self.spec.bottom_left {
            ret.push(match border {
                TileBorderType::HorizontalWall => 5,
                TileBorderType::VerticalWall => 6,
                TileBorderType::InnerCorner => 11,
                TileBorderType::OuterCorner => 15,
                TileBorderType::Diagonal => 17,
            });
        };

        ret
    }
}

fn get_tileborder_from_terrain(
    terrain: usize,
    matrix: TilePositionNeighbors,
) -> Option<TileBorder> {
    let left = matrix.left.0.base as usize;
    let top = matrix.top.0.base as usize;
    let right = matrix.right.0.base as usize;
    let bottom = matrix.bottom.0.base as usize;

    let top_left = matrix.top_left.0.base as usize;
    let top_right = matrix.top_right.0.base as usize;
    let bottom_left = matrix.bottom_left.0.base as usize;
    let bottom_right = matrix.bottom_right.0.base as usize;

    match (top == terrain, right == terrain, bottom == terrain, left == terrain) {

        // All Inner Corners
        (true, true, true, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::all_inner_corners()
        }),

        // Open C's
        (false, true, true, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::open_c(OpenEnd::Top)
        }),
        (true, false, true, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::open_c(OpenEnd::Right)
        }),
        (true, true, false, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::open_c(OpenEnd::Bottom)
        }),
        (true, true, true, false) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::open_c(OpenEnd::Left)
        }),

        // Tunnels
        (true, false, true, false) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::tunnel(TunnelOrientation::Horizontal)
        }),
        (false, true, false, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::tunnel(TunnelOrientation::Vertical)
        }),

        // Diagonals
        (true, false, false, true) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::diagonal(
                DiagonalLocation::TopLeft,
                bottom_right == terrain)
        }),
        (true, true, false, false) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::diagonal(
                DiagonalLocation::TopRight,
                bottom_left == terrain)
        }),
        (false, true, true, false) => Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::diagonal(
                DiagonalLocation::BottomRight,
                top_left == terrain)
        }),
        (false, false, true, true) =>  Some(TileBorder {
            terrain: terrain.into(),
            spec: BorderSpec::diagonal(
                DiagonalLocation::BottomLeft,
                top_right == terrain)
        }),

        _ => None
        // (false, true, false, false) => todo!(),
        // (false, false, false, false) => todo!(),
        // (true, false, false, false) => todo!(),
        // (false, false, false, true) => todo!(),
        // (false, false, true, false) => todo!(),
    }
}

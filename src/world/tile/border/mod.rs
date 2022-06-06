use self::{
    neighbor::get_border_from_neighbor_effects,
    structs::{
        BorderSpec, DiagonalLocation, OpenEnd, TileBorder, TileBorderType, TunnelOrientation,
    },
};

use super::{
    position::TilePositionNeighbors,
    terrain::{generator::TerrainGenerator, Terrain},
};

pub mod neighbor;
pub mod structs;

impl TileBorder {
    pub fn from(matrix: &TilePositionNeighbors, gen: &TerrainGenerator) -> Vec<Self> {
        // We receive no borders if we are the strongest terrain.
        if &matrix.center.0.terrain == gen.terrains.last().unwrap() {
            // Return empty array.
            return Vec::new();
        }

        let center = &matrix.center.0.terrain;

        let left = &matrix.left.0.terrain;
        let top = &matrix.top.0.terrain;
        let right = &matrix.right.0.terrain;
        let bottom = &matrix.bottom.0.terrain;

        let top_left = &matrix.top_left.0.terrain;
        let top_right = &matrix.top_right.0.terrain;
        let bottom_left = &matrix.bottom_left.0.terrain;
        let bottom_right = &matrix.bottom_right.0.terrain;

        ((center.strength + 1)..gen.terrain_count)
            .map(|terrain_strength| {
                let tileborder_from_4: Option<TileBorder> =
                    get_tileborder_from_terrain(&gen.terrains[terrain_strength], matrix);

                if let Some(_) = tileborder_from_4 {
                    return tileborder_from_4;
                };

                let upper_left = get_border_from_neighbor_effects(
                    left == &gen.terrains[terrain_strength],
                    top == &gen.terrains[terrain_strength],
                    top_left == &gen.terrains[terrain_strength],
                );

                let upper_right = get_border_from_neighbor_effects(
                    right == &gen.terrains[terrain_strength],
                    top == &gen.terrains[terrain_strength],
                    top_right == &gen.terrains[terrain_strength],
                );

                let bottom_left = get_border_from_neighbor_effects(
                    left == &gen.terrains[terrain_strength],
                    bottom == &gen.terrains[terrain_strength],
                    bottom_left == &gen.terrains[terrain_strength],
                );

                let bottom_right = get_border_from_neighbor_effects(
                    right == &gen.terrains[terrain_strength],
                    bottom == &gen.terrains[terrain_strength],
                    bottom_right == &gen.terrains[terrain_strength],
                );

                let borders = [upper_left, upper_right, bottom_right, bottom_left];

                if borders == [None, None, None, None] {
                    return None;
                };

                Some(TileBorder {
                    terrain: gen.terrains[terrain_strength].clone(),
                    spec: borders.into(),
                })
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
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
    terrain: &Terrain,
    matrix: &TilePositionNeighbors,
) -> Option<TileBorder> {
    let left = &matrix.left.0.terrain;
    let top = &matrix.top.0.terrain;
    let right = &matrix.right.0.terrain;
    let bottom = &matrix.bottom.0.terrain;

    let top_left = &matrix.top_left.0.terrain;
    let top_right = &matrix.top_right.0.terrain;
    let bottom_left = &matrix.bottom_left.0.terrain;
    let bottom_right = &matrix.bottom_right.0.terrain;

    match (
        top == terrain,
        right == terrain,
        bottom == terrain,
        left == terrain,
    ) {
        // All Inner Corners
        (true, true, true, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::all_inner_corners(),
        }),

        // Open C's
        (false, true, true, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::open_c(OpenEnd::Top),
        }),
        (true, false, true, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::open_c(OpenEnd::Right),
        }),
        (true, true, false, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::open_c(OpenEnd::Bottom),
        }),
        (true, true, true, false) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::open_c(OpenEnd::Left),
        }),

        // Tunnels
        (true, false, true, false) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::tunnel(TunnelOrientation::Horizontal),
        }),
        (false, true, false, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::tunnel(TunnelOrientation::Vertical),
        }),

        // Diagonals
        (true, false, false, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::diagonal(DiagonalLocation::TopLeft, bottom_right == terrain),
        }),
        (true, true, false, false) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::diagonal(DiagonalLocation::TopRight, bottom_left == terrain),
        }),
        (false, true, true, false) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::diagonal(DiagonalLocation::BottomRight, top_left == terrain),
        }),
        (false, false, true, true) => Some(TileBorder {
            terrain: terrain.clone(),
            spec: BorderSpec::diagonal(DiagonalLocation::BottomLeft, top_right == terrain),
        }),

        // These last cases
        // should be handled by the external (Neightbors) rule.
        // (false, true, false, false) => None,
        // (false, false, false, false) => None,
        // (true, false, false, false) => None,
        // (false, false, false, true) => None,
        // (false, false, true, false) => None,
        _ => None,
    }
}

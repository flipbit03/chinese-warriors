use itertools::Itertools;

use crate::world::tile::terrain::{BaseTerrain, TerrainConfig};

pub fn generate_default_terrain_set(
    debug_generate_move_speed_ramp: bool,
) -> Vec<TerrainConfig> {
    [
        TerrainConfig::new("DeepWater".to_string(), BaseTerrain::DeepWater, true, None),
        TerrainConfig::new(
            "ShallowWater".to_string(),
            BaseTerrain::ShallowWater,
            true,
            None,
        ),
        TerrainConfig::new_color(
            "SwampWater".to_string(),
            BaseTerrain::ShallowWater,
            true,
            Some((215, 240, 200)),
            None,
            None,
        ),
        TerrainConfig::new("Grass".to_string(), BaseTerrain::Grass, true, None),
        TerrainConfig::new_color(
            "TundraGrass".to_string(),
            BaseTerrain::Grass,
            true,
            Some((230, 230, 255)),
            None,
            Some(1.5),
        ),
        TerrainConfig::new_color(
            "FrozenGrass".to_string(),
            BaseTerrain::Grass,
            true,
            Some((170, 190, 255)),
            None,
            Some(1.6),
        ),
        TerrainConfig::new_color(
            "PlainsGrass".to_string(),
            BaseTerrain::Grass,
            true,
            Some((250, 235, 220)),
            None,
            Some(1.7),
        ),
        TerrainConfig::new_color(
            "DarkGrass".to_string(),
            BaseTerrain::Grass,
            true,
            Some((225, 225, 225)),
            None,
            None,
        ),
        TerrainConfig::new_color(
            "SwampGrass".to_string(),
            BaseTerrain::Grass,
            true,
            Some((190, 185, 160)),
            None,
            None,
        ),
        TerrainConfig::new("Mud".to_string(), BaseTerrain::Mud, true, None),
        TerrainConfig::new_color(
            "Clay".to_string(),
            BaseTerrain::Mud,
            true,
            Some((255, 235, 220)),
            None,
            None,
        ),
        TerrainConfig::new_color(
            "DarkMud".to_string(),
            BaseTerrain::Mud,
            true,
            Some((225, 225, 225)),
            None,
            None,
        ),
        TerrainConfig::new("Sand".to_string(), BaseTerrain::Sand, true, None),
        TerrainConfig::new_color(
            "WetSand".to_string(),
            BaseTerrain::Sand,
            true,
            Some((235, 235, 235)),
            None,
            None,
        ),
        TerrainConfig::new("Stone".to_string(), BaseTerrain::Stone, true, None),
        TerrainConfig::new_color(
            "RedStone".to_string(),
            BaseTerrain::Stone,
            true,
            Some((235, 200, 160)),
            None,
            None,
        ),
        TerrainConfig::new_color(
            "DarkStone".to_string(),
            BaseTerrain::Stone,
            true,
            Some((215, 215, 215)),
            None,
            None,
        ),
        TerrainConfig::new_color(
            "SandStone".to_string(),
            BaseTerrain::Stone,
            true,
            Some((255, 225, 175)),
            None,
            None,
        ),
        TerrainConfig::new("Snow".to_string(), BaseTerrain::Snow, true, None),
        TerrainConfig::new_color(
            "BlueSnow".to_string(),
            BaseTerrain::Snow,
            false,
            Some((230, 240, 255)),
            None,
            None,
        ),
    ]
    .iter_mut()
    .enumerate()
    .map(|(i, x)| {
        if debug_generate_move_speed_ramp {
            x.move_speed_multiplier = Some(0.5 + i as f32 / 10.)
        };
        x.clone()
    })
    .collect_vec()
}

use crate::world::tile::terrain::{BaseTerrain, TerrainConfig};

pub fn generate_default_terrain_set() -> Vec<TerrainConfig> {
    [
        TerrainConfig::new("DeepWater".to_string(), BaseTerrain::DeepWater, true),
        TerrainConfig::new("ShallowWater".to_string(), BaseTerrain::ShallowWater, true),
        TerrainConfig::new_color(
            "SwampWater".to_string(),
            BaseTerrain::ShallowWater,
            true,
            (215, 240, 200),
        ),
        TerrainConfig::new("Grass".to_string(), BaseTerrain::Grass, true),
        TerrainConfig::new_color(
            "TundraGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (230, 230, 255),
        ),
        TerrainConfig::new_color(
            "FrozenGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (170, 190, 255),
        ),
        TerrainConfig::new_color(
            "PlainsGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (250, 235, 220),
        ),
        TerrainConfig::new_color(
            "DarkGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (225, 225, 225),
        ),
        TerrainConfig::new_color(
            "SwampGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (190, 185, 160),
        ),
        TerrainConfig::new("Mud".to_string(), BaseTerrain::Mud, true),
        TerrainConfig::new_color("Clay".to_string(), BaseTerrain::Mud, true, (255, 235, 220)),
        TerrainConfig::new_color(
            "DarkMud".to_string(),
            BaseTerrain::Mud,
            true,
            (225, 225, 225),
        ),
        TerrainConfig::new("Sand".to_string(), BaseTerrain::Sand, true),
        TerrainConfig::new_color(
            "WetSand".to_string(),
            BaseTerrain::Sand,
            true,
            (235, 235, 235),
        ),
        TerrainConfig::new("Stone".to_string(), BaseTerrain::Stone, true),
        TerrainConfig::new_color(
            "RedStone".to_string(),
            BaseTerrain::Stone,
            true,
            (235, 200, 160),
        ),
        TerrainConfig::new_color(
            "DarkStone".to_string(),
            BaseTerrain::Stone,
            true,
            (215, 215, 215),
        ),
        TerrainConfig::new_color(
            "SandStone".to_string(),
            BaseTerrain::Stone,
            true,
            (255, 225, 175),
        ),
        TerrainConfig::new("Snow".to_string(), BaseTerrain::Snow, true),
        TerrainConfig::new_color(
            "BlueSnow".to_string(),
            BaseTerrain::Snow,
            false,
            (230, 240, 255),
        ),
    ]
    .to_vec()
}

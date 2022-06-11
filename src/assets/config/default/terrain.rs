use crate::world::tile::terrain::{BaseTerrain, TerrainConfig};

pub fn generate_default_terrain_set() -> Vec<TerrainConfig> {
    [
        // Basic, Unmodified Terrains
        TerrainConfig::new("DeepWater".to_string(), BaseTerrain::DeepWater, false),
        TerrainConfig::new("Grass".to_string(), BaseTerrain::Grass, true),
        TerrainConfig::new("Stone".to_string(), BaseTerrain::Stone, true),
        TerrainConfig::new("Sand".to_string(), BaseTerrain::Sand, true),
        TerrainConfig::new("ShallowWater".to_string(), BaseTerrain::ShallowWater, true),
        TerrainConfig::new("Snow".to_string(), BaseTerrain::Snow, false),
        TerrainConfig::new("Mud".to_string(), BaseTerrain::Mud, false),
        // Some Terrain Variations
        TerrainConfig::new_color(
            "DarkSand".to_string(),
            BaseTerrain::Sand,
            true,
            (235, 235, 235),
        ),
        TerrainConfig::new_color(
            "DarkStone".to_string(),
            BaseTerrain::Grass,
            true,
            (155, 155, 155),
        ),
        TerrainConfig::new_color(
            "DeepGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (140, 255, 255),
        ),
        TerrainConfig::new_color(
            "TaintedGrass".to_string(),
            BaseTerrain::Grass,
            true,
            (140, 155, 79),
        ),
    ]
    .to_vec()
}

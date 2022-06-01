use noise::{Fbm, NoiseFn, Perlin, Seedable};
use serde::{Deserialize, Serialize};

use crate::{utilities::xy::XY, world::tile::position::TilePosition};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoiseGeneratorConfig {
    pub seed: u32,
    pub noise_scale_factor: XY<f64>,
}

pub struct NoiseGenerator {
    pub config: NoiseGeneratorConfig,
    pub perlin: Perlin,
    pub fbm: Fbm,
}

impl NoiseGenerator {
    pub fn new_from_config(config: &NoiseGeneratorConfig) -> Self {
        Self {
            config: config.clone(),
            perlin: Perlin::new().set_seed(config.seed),
            fbm: Fbm::new().set_seed(config.seed),
        }
    }

    pub fn get_noise(&self, tp: &TilePosition) -> f64 {
        let terrain_point = [
            tp.x as f64 / self.config.noise_scale_factor.x,
            tp.y as f64 / self.config.noise_scale_factor.y,
        ];

        // 2 noise components in the [-0.5, 0.5] range
        let fbm_value = self.fbm.get(terrain_point) / 2.0;
        let perlin_value = self.perlin.get(terrain_point) / 2.0;

        // Sum both, get a value "somewhere" in the [-1.0, 1.0] range
        let combined = fbm_value + perlin_value; // -1 a 1

        // Add 1.0 (range becomes [0.0, 2.0], divide by 2, final normalized range becomes [0.0, 1.0]
        let normalized = ((combined + 1.0) / 2.0).clamp(0.0, 1.0);

        normalized
    }
}

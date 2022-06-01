use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize, VariantCount)]
pub enum Biome {
    SandyForest,
}

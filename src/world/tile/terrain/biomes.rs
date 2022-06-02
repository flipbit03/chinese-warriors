use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum Biome {
    SandyForest,
}

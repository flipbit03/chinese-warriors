use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};

use crate::{hero::structs::HeroConfig, world::tile::builder::WorldBuilderConfig};

#[derive(Deserialize, Serialize, Debug, TypeUuid, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct CwConfig {
    pub world: WorldBuilderConfig,
    pub hero: HeroConfig,
}

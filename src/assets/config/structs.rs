use std::{
    error::Error,
    ffi::OsStr,
    path::{Component, Path},
};

use bevy::reflect::TypeUuid;
use clap::Parser;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{hero::structs::HeroConfig, world::tile::builder::WorldBuilderConfig};

#[derive(Deserialize, Serialize, Debug, TypeUuid, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f9"]
pub struct CwDebugFlags {
    pub print_player_position: bool,
}

#[derive(Deserialize, Serialize, Debug, TypeUuid, Clone)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct CwConfig {
    pub debug_flags: Option<CwDebugFlags>,
    pub world: WorldBuilderConfig,
    pub hero: HeroConfig,
}

pub const DEFAULT_CONFIG_PATH: &str = "assets/config/world.config.ron";
pub const DEBUG_CONFIG_PATH: &str = "assets/config/debug_world.config.ron";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CwCliConfig {
    #[clap(default_value_t = String::from(DEFAULT_CONFIG_PATH), value_parser = remove_assets_prefix)]
    pub config_file: String,
}

fn remove_assets_prefix(
    s: &str,
) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
    let path_components = Path::new(s).components().collect_vec();
    let component_count = path_components.iter().count();

    // Work in reverse from last component and find the "assets/" folder
    // path index that sits as near the END of the path as possible.
    let assets_path_pos = path_components
        .iter()
        .rev()
        .find_position(|p| p == &&Component::Normal(OsStr::new("assets")))
        .unwrap()
        .0;

    // Reassemble the final string with the "../../assets" portion removed.
    let reassembled_path = path_components
        .iter()
        .skip(component_count - assets_path_pos)
        .map(|x| x.as_os_str().to_str().unwrap())
        .join("/");

    Ok(reassembled_path)
}

use bevy::{asset::Asset, reflect::TypePath};
use serde::{Deserialize, Serialize};

use crate::{hero::structs::HeroConfig, world::tile::builder::WorldBuilderConfig};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CwDebugFlags {
    pub print_player_position: bool,
    pub debug_chunk: bool,
}

#[derive(Asset, TypePath, Deserialize, Serialize, Debug, Clone, bevy::prelude::Resource)]
pub struct CwConfig {
    pub debug_flags: CwDebugFlags,
    pub world: WorldBuilderConfig,
    pub hero: HeroConfig,
}

pub const DEFAULT_CONFIG_PATH: &str = "assets/config/world.config.ron";
pub const DEBUG_CONFIG_PATH: &str = "assets/config/debug_world.config.ron";

#[cfg(not(target_arch = "wasm32"))]
mod cli {
    use std::{
        error::Error,
        ffi::OsStr,
        path::{Component, Path},
    };

    use clap::Parser;
    use itertools::Itertools;

    use super::DEFAULT_CONFIG_PATH;

    #[derive(Parser, bevy::prelude::Resource)]
    #[command(author, version, about, long_about = None)]
    pub struct CwCliConfig {
        #[arg(default_value_t = String::from(DEFAULT_CONFIG_PATH), value_parser = remove_assets_prefix)]
        pub config_file: String,
    }

    pub fn remove_assets_prefix(
        s: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync + 'static>> {
        let path_components = Path::new(s).components().collect_vec();
        let component_count = path_components.iter().count();

        let assets_path_pos = path_components
            .iter()
            .rev()
            .find_position(|p| p == &&Component::Normal(OsStr::new("assets")))
            .unwrap()
            .0;

        let reassembled_path = path_components
            .iter()
            .skip(component_count - assets_path_pos)
            .map(|x| x.as_os_str().to_str().unwrap())
            .join("/");

        Ok(reassembled_path)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use cli::CwCliConfig;

#[cfg(target_arch = "wasm32")]
#[derive(bevy::prelude::Resource)]
pub struct CwCliConfig {
    pub config_file: String,
}

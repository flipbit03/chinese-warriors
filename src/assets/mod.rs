use bevy::prelude::*;

use crate::{app::GameState, world::tile::builder::WorldBuilder};

use self::{
    aseprite::{load_guri_aseprite, GuriAssets},
    config::{
        live_updater::live_update_main_config, load_config_save_handle,
        structs::CwConfig, CwConfigLoader,
    },
    fonts::load_fonts,
    game::create_initial_tilebuilder,
    textures::{load_textures, TerrainTextures},
    timers::load_timers,
};

pub mod aseprite;
pub mod asset_reloader;
pub mod config;
pub mod fonts;
pub mod game;
pub mod textures;
pub mod timers;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register CwConfig as a custom asset with our RON loader
            .init_asset::<CwConfig>()
            .init_asset_loader::<CwConfigLoader>()
            // Loading state systems
            .add_systems(
                OnEnter(GameState::Loading),
                (
                    load_config_save_handle,
                    load_fonts,
                    load_textures,
                    load_timers,
                    load_guri_aseprite,
                ),
            )
            // Wait for config to be loaded, then create the world builder
            .add_systems(
                Update,
                create_initial_tilebuilder
                    .run_if(in_state(GameState::Loading))
                    .run_if(resource_exists::<CwConfig>)
                    .run_if(not(resource_exists::<WorldBuilder>)),
            )
            // Transition to InGame when all resources are ready
            .add_systems(
                Update,
                change_mode_to_ingame
                    .run_if(in_state(GameState::Loading))
                    .run_if(resource_exists::<TerrainTextures>)
                    .run_if(resource_exists::<WorldBuilder>)
                    .run_if(resource_exists::<GuriAssets>),
            )
            // Live config update (always active)
            .add_systems(Update, live_update_main_config);
    }
}

pub fn change_mode_to_ingame(mut next_state: ResMut<NextState<GameState>>) {
    info!("Changing to InGame");
    next_state.set(GameState::InGame);
}

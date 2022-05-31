use bevy::prelude::{App, Commands, Plugin};
use bevy_asset_ron::RonAssetPlugin;
use iyes_loopless::{prelude::{AppLooplessStateExt, ConditionSet}, state::NextState};

use crate::{app::GameState, world::tile::TileBuilder};

use self::{
    asset_reloader::activate_live_asset_reloading,
    config::{live_updater::live_update_main_config, load_config_save_handle, structs::CwConfig},
    fonts::load_fonts,
    game::create_initial_tilebuilder,
    textures::load_textures,
    timers::load_timers,
};

pub mod asset_reloader;
pub mod config;
pub mod fonts;
pub mod game;
pub mod textures;
pub mod timers;
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CwConfig>::new(&["config"]))
            .add_enter_system(GameState::Loading, activate_live_asset_reloading)
            .add_enter_system(GameState::Loading, load_config_save_handle)
            .add_enter_system(GameState::Loading, load_fonts)
            .add_enter_system(GameState::Loading, load_textures)
            .add_enter_system(GameState::Loading, load_timers)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Loading)
                    .run_if_resource_exists::<CwConfig>()
                    .run_unless_resource_exists::<TileBuilder>()
                    .with_system(create_initial_tilebuilder)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Loading)
                    .run_if_resource_exists::<TileBuilder>()
                    .with_system(change_mode_to_ingame)
                    .into(),
            )
            .add_system(live_update_main_config);
    }
}

pub fn change_mode_to_ingame(mut commands: Commands) {
    println!("Changing to InGame");
    commands.insert_resource(NextState(GameState::InGame))
}

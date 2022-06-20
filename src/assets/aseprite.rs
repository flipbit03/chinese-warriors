use std::path::Path;

use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ase::{
    asset::{Animation, AseAsset, AseFileMap},
    loader::Loader,
};

static GURI_ASEPRITE_PATH: &str = "art/hero/guri/Guri.aseprite";

pub struct GuriAssets {
    pub walk_cycle_anim: (Handle<Animation>, Handle<SpriteSheetAnimation>),
    pub idle_anim: (Handle<Animation>, Handle<SpriteSheetAnimation>),
}

pub fn load_aseprite_assets(
    asset_server: Res<AssetServer>,
    mut aseloader: ResMut<Loader>,
) {
    info!("Loading Guri.aseprite...");
    let h: Handle<AseAsset> = asset_server.load(GURI_ASEPRITE_PATH);
    aseloader.add(h.clone());
}

pub struct AllAsepritesLoaded;

// Wait until all sprites are loaded.
pub fn wait_for_loaded_aseprites_to_be_processed(
    mut commands: Commands,
    ase_loader: Res<Loader>,
) {
    if ase_loader.is_loaded() {
        info!("All Aseprite files loaded");
        commands.insert_resource(AllAsepritesLoaded);
    }
}

// Wait until all sprites are loaded.
pub fn load_guri_animations(
    mut commands: Commands,
    ase_files: Res<AseFileMap>,
    raw_animations: Res<Assets<Animation>>,
    mut benimations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let idle_anim_handle = ase_files
        .animation(Path::new(GURI_ASEPRITE_PATH), "Idle")
        .unwrap();
    let walk_cycle_anim_handle = ase_files
        .animation(Path::new(GURI_ASEPRITE_PATH), "Walk")
        .unwrap();

    let idle_anim = raw_animations.get(idle_anim_handle.clone()).unwrap();
    let walk_cycle_anim = raw_animations.get(walk_cycle_anim_handle.clone()).unwrap();

    let idle_anim_benimator_handle = benimations.add(idle_anim.into());
    let walk_cycle_benimator_handle = benimations.add(walk_cycle_anim.into());

    commands.insert_resource(GuriAssets {
        idle_anim: (idle_anim_handle, idle_anim_benimator_handle),
        walk_cycle_anim: (walk_cycle_anim_handle, walk_cycle_benimator_handle),
    });
}

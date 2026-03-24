use bevy::prelude::*;

/// Sprite sheet animation configuration for the hero character.
/// Replaces the old benimator/bevy_ase aseprite loading.
/// The sprite sheet is pre-exported from Guri.aseprite as spritesheet.png (224x32, 7 frames of 32x32).
#[derive(Resource)]
pub struct GuriAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub idle_first: usize,
    pub idle_last: usize,
    pub walk_first: usize,
    pub walk_last: usize,
}

pub fn load_guri_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading Guri spritesheet...");

    let texture = asset_server.load("art/hero/guri/spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 7, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(GuriAssets {
        texture,
        layout: layout_handle,
        idle_first: 0,
        idle_last: 1,
        walk_first: 2,
        walk_last: 6,
    });
}

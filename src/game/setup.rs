use bevy::prelude::*;

#[derive(Deref)]
pub struct GuriData {
    pub texture_handle: Handle<TextureAtlas>
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    
    let texture_handle = asset_server.load("guri.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 7, 1);
    let guri_atlas_handle = texture_atlases.add(texture_atlas);

    let guridata = GuriData {
        texture_handle: guri_atlas_handle
    };

    commands.insert_resource(guridata);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
use bevy::prelude::*;
use noise::Perlin;

use super::terrain::{frustum_culling::WorldViewFrustum, TerrainCreation};

#[derive(Deref)]
pub struct GuriData {
    pub texture_handle: Handle<TextureAtlas>,
}

#[derive(Deref)]
pub struct TerrainTextureHandle {
    pub terrain: Handle<TextureAtlas>,
}

pub struct GlobalScaleFactor {
    pub factor: f32,
}

pub struct WorldGenerationProperties {
    seed: u32,
    perlin: Perlin,
}

pub struct FpsTimer(pub Timer);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //let terrain_handle: Handle<Image> = asset_server.load("terrain-kawe1");
    let terrain_handle: Handle<Image> = asset_server.load("terrain.png");
    let terrain_atlas = TextureAtlas::from_grid(
        terrain_handle, 
        Vec2::new(16.0, 16.0), 
        11, 
        1);
    let terrain_atlas_handle = texture_atlases.add(terrain_atlas);

    let terrain_handle = TerrainTextureHandle {
        terrain: terrain_atlas_handle,
    };

    commands.insert_resource(terrain_handle);

    let texture_handle = asset_server.load("guri2.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 7, 1);
    let guri_atlas_handle = texture_atlases.add(texture_atlas);

    let guridata = GuriData {
        texture_handle: guri_atlas_handle,
    };

    commands.insert_resource(guridata);

    commands.insert_resource(FpsTimer(Timer::from_seconds(4.0, true)));

    commands.insert_resource(GlobalScaleFactor { factor: 2.0 });

    commands.insert_resource(TerrainCreation::default());

    commands.insert_resource(WorldViewFrustum::default());

    // commands.insert_resource(WorldGenerationProperties::default());

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

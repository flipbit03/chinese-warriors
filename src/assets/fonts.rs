use bevy::prelude::*;

#[derive(Resource)]
pub struct MainFont {
    pub handle: Handle<Font>,
}

pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MainFont {
        handle: asset_server.load("art/font/iosevka-term-ss01-semibold.ttf"),
    });
}

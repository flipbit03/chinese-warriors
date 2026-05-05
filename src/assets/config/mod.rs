use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

pub mod default;
pub mod live_updater;
pub mod structs;
use self::structs::{CwCliConfig, CwConfig};

/// Custom RON asset loader for CwConfig (replaces bevy_asset_ron)
#[derive(Default, TypePath)]
pub struct CwConfigLoader;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CwConfigLoaderError {
    #[error("Could not read asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for CwConfigLoader {
    type Asset = CwConfig;
    type Settings = ();
    type Error = CwConfigLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let config = ron::de::from_bytes::<CwConfig>(&bytes)?;
        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["config.ron"]
    }
}

/// Wrapper to store Handle<CwConfig> as a Resource
#[derive(Resource)]
pub struct CwConfigHandle(pub Handle<CwConfig>);

/// Setup System
pub fn load_config_save_handle(
    mut commands: Commands,
    config: Res<CwCliConfig>,
    server: Res<AssetServer>,
) {
    let config_handle: Handle<CwConfig> = server.load(&config.config_file);
    commands.insert_resource(CwConfigHandle(config_handle));
}

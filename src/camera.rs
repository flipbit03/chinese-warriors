use bevy::prelude::{
    App, Commands, OrthographicCameraBundle, Plugin, StartupStage, UiCameraBundle,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_ui_camera)
            .add_startup_system_to_stage(StartupStage::PreStartup, spawn_2d_camera);
    }
}

pub fn spawn_2d_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.5;
    commands.spawn_bundle(camera);
}

pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

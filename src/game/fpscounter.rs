use bevy::{prelude::*, window::WindowResized};

use super::{
    setup::FpsTimer,
    terrain::{frustum_culling::WorldFrustum, TerrainCreation},
};

pub fn timed_fps_counter(
    time: Res<Time>,
    world_frustum: Res<WorldFrustum>,
    mut fpstimer: ResMut<FpsTimer>,
    terrain_counter: Res<TerrainCreation>,
) {
    if fpstimer.0.tick(time.delta()).just_finished() {
        println!("{:?}", world_frustum);
        println!("TILES_CREATED = {:?}", terrain_counter.storage.len());
    }
}

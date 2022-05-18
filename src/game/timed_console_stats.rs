use bevy::prelude::*;

use super::{
    setup::FpsTimer,
    terrain::{frustum_culling::WorldViewFrustum, TerrainCreation},
};

pub fn console_stats(
    time: Res<Time>,
    world_frustum: Res<WorldViewFrustum>,
    mut fpstimer: ResMut<FpsTimer>,
    terrain_counter: Res<TerrainCreation>,
) {
    if fpstimer.0.tick(time.delta()).just_finished() {
        // println!("{:?}", world_frustum);
        println!("TileStorageCount={:?}", terrain_counter.storage.len());
    }
}

use bevy::prelude::*;

use super::{resources::timers::ConsoleStatusTimer, world::tile::TileBuilder};

pub fn console_stats(
    time: Res<Time>,
    tile_builder: Res<TileBuilder>,
    mut fpstimer: ResMut<ConsoleStatusTimer>,
) {
    if fpstimer.0.tick(time.delta()).just_finished() {
        // println!("{:?}", world_frustum);
        println!("Existing Tiles = {}", tile_builder.storage.len());
    }
}

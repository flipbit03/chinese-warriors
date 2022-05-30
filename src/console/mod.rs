use bevy::prelude::*;

use super::assets::timers::ConsoleStatusTimer;

pub fn console_stats(time: Res<Time>, mut fpstimer: ResMut<ConsoleStatusTimer>) {
    if fpstimer.0.tick(time.delta()).just_finished() {
        // println!("{:?}", world_frustum);
        // println!("Existing Tiles = {}", tile_builder.storage.len());
    }
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(console_stats);
    }
}

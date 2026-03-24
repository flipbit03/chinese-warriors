use bevy::prelude::*;

use crate::app::GameState;
use crate::assets::timers::ConsoleStatusTimer;

pub fn console_stats(time: Res<Time>, mut fpstimer: ResMut<ConsoleStatusTimer>) {
    if fpstimer.0.tick(time.delta()).just_finished() {}
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            console_stats.run_if(in_state(GameState::InGame)),
        );
    }
}

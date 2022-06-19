use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use super::assets::timers::ConsoleStatusTimer;

pub fn console_stats(time: Res<Time>, mut fpstimer: ResMut<ConsoleStatusTimer>) {
    if fpstimer.0.tick(time.delta()).just_finished() {}
}

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(console_stats)
                .into(),
        );
    }
}

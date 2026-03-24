use bevy::prelude::*;

#[derive(Resource)]
pub struct ConsoleStatusTimer(pub Timer);

pub fn load_timers(mut commands: Commands) {
    commands.insert_resource(ConsoleStatusTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
}

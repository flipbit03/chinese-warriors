use bevy::{core::Timer, prelude::Commands};

pub struct ConsoleStatusTimer(pub Timer);

pub fn load_timers(mut commands: Commands) {
    commands.insert_resource(ConsoleStatusTimer(Timer::from_seconds(2.0, true)));
}

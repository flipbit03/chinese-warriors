use bevy::prelude::{App, Plugin};

use crate::systems::console::console_stats;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(console_stats);
    }
}

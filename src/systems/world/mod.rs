pub mod spawner;
pub mod tile;

use bevy::prelude::{App, Plugin};

use self::spawner::spawn_terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_terrain);
    }
}

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::app::GameState;

use self::{animation::animate_hero, spawner::spawn_hero};

pub mod animation;
pub mod spawner;
pub mod structs;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, spawn_hero)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::InGame)
                    .with_system(animate_hero)
                    .into(),
            );
    }
}

use bevy::{
    core::Timer,
    prelude::{Component, Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

pub enum HeroFacing {
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub struct HeroWalkCycleTimer(pub Timer);

#[derive(Component)]
pub struct Hero {
    pub facing: HeroFacing,
    pub walking: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HeroConfig {
    pub move_speed: f32,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            facing: HeroFacing::Left,
            walking: false,
        }
    }
}

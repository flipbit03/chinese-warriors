use bevy::{
    core::Timer,
    prelude::{Component, Deref, DerefMut},
};

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

impl Default for Hero {
    fn default() -> Self {
        Self {
            facing: HeroFacing::Left,
            walking: false,
        }
    }
}

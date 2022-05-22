use bevy::{
    core::Timer,
    prelude::{Component, Deref, DerefMut},
};

pub enum Facing {
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub struct HeroWalkCycleTimer(pub Timer);

#[derive(Component)]
pub struct Hero {
    pub facing: Facing,
    pub walking: bool,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            facing: Facing::Left,
            walking: false,
        }
    }
}

use bevy::{math::Vec2, prelude::Component};
use serde::{Deserialize, Serialize};

pub enum HeroFacing {
    Left,
    Right,
}

#[derive(Debug)]
pub enum HeroAction {
    Idling,
    Walking,
}

#[derive(Component)]
pub struct Hero {
    pub facing: HeroFacing,
    pub action: HeroAction,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HeroConfig {
    pub move_speed: f32,
    pub spawn_point: Vec2,
}

impl Default for Hero {
    fn default() -> Self {
        Self {
            facing: HeroFacing::Left,
            action: HeroAction::Idling,
        }
    }
}

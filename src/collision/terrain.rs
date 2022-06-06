use bevy::prelude::{Query, Transform, With};

use crate::hero::structs::Hero;

pub struct HeroConstrainedMovement<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}

pub fn collision_with_non_walkable_terrain(hero_query: Query<&Transform, With<Hero>>) {
    let _ = hero_query.single();
}

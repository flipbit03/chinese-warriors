use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use super::{
    current_tile::MoveSpeed,
    structs::{Hero, HeroAction, HeroFacing},
};

pub fn animate_hero(
    mut hero_query: Query<(
        &Hero,
        &mut AseAnimation,
        &mut Sprite,
        &MoveSpeed,
    )>,
) {
    let Ok((hero, mut ase, mut sprite, hero_movespeed)) = hero_query.single_mut() else {
        return;
    };

    // Switch animation tag based on action
    let target_tag = match hero.action {
        HeroAction::Idling => "Idle",
        HeroAction::Walking => "Walk",
    };

    // Only update if the tag actually changed
    let current_tag = ase.animation.tag.as_deref().unwrap_or("");
    if current_tag != target_tag {
        ase.animation = Animation::tag(target_tag).with_repeat(AnimationRepeat::Loop);
    }

    // Adjust animation speed based on move speed when walking
    let speed = match hero.action {
        HeroAction::Idling => 1.0,
        HeroAction::Walking => (hero_movespeed.0 * 0.95).max(0.3),
    };
    ase.animation.speed = speed;

    // Flip sprite based on facing
    sprite.flip_x = match hero.facing {
        HeroFacing::Left => false,
        HeroFacing::Right => true,
    };
}

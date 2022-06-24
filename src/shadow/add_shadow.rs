use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{Added, Color, Commands, Entity, Query, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::assets::textures::ShadowTexture;

use super::structs::CastsShadow;

pub fn add_shadow_to_character(
    mut commands: Commands,
    shadow: Res<ShadowTexture>,
    tile_instructions_query: Query<(Entity, &CastsShadow), Added<CastsShadow>>,
) {
    for (ent, casts_shadow) in tile_instructions_query.iter() {
        commands.entity(ent).with_children(|c| {
            c.spawn_bundle(SpriteBundle {
                texture: shadow.texture_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, casts_shadow.y_offset, -0.0000001),
                    scale: Vec3::new(casts_shadow.x_scale, 1.0, 1.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, casts_shadow.alpha),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}

use bevy::prelude::*;

use crate::assets::textures::ShadowTexture;

use super::structs::CastsShadow;

pub fn add_shadow_to_character(
    mut commands: Commands,
    shadow: Res<ShadowTexture>,
    tile_instructions_query: Query<(Entity, &CastsShadow), Added<CastsShadow>>,
) {
    for (ent, casts_shadow) in tile_instructions_query.iter() {
        commands.entity(ent).with_children(|c| {
            c.spawn((
                Sprite {
                    image: shadow.texture_handle.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, casts_shadow.alpha),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(0.0, casts_shadow.y_offset, -0.0000001),
                    scale: Vec3::new(casts_shadow.x_scale, 1.0, 1.0),
                    ..Default::default()
                },
            ));
        });
    }
}

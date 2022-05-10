use bevy::prelude::*;
mod util;
use self::util::Facing;

use super::setup::GuriData;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);


#[derive(Component)]
pub struct Hero {
    pub facing: Facing,
    pub walking: bool
}

impl Default for Hero {
    fn default() -> Self {
        Self { facing: Facing::Left, walking: false }
    }
}

pub fn spawn_hero(mut commands: Commands,
    asset_server: Res<AssetServer>,
    guri_data: Res<GuriData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: guri_data.texture_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        })
        .insert(Hero::default())
        .insert(AnimationTimer(Timer::from_seconds(0.07, true)));
}

pub fn animate_hero(
    time: Res<Time>,
    guri_data: Res<GuriData>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Hero), With<Hero>>) {
    
    let (mut timer, mut sprite, hero) = query.single_mut();

    timer.tick(time.delta());
    if timer.just_finished() {
        let texture_atlas = texture_atlases.get(guri_data.texture_handle.clone()).unwrap();
        match hero.facing {
            Facing::Right => { sprite.flip_x = true;},
            Facing::Left => { sprite.flip_x = false;},
        };
      
        if let true = hero.walking {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        };
        
    };
}

pub fn hero_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Hero>) {

    let mut hero = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::A) {
        hero.facing = Facing::Left;
    }

    if keyboard_input.pressed(KeyCode::A) {
        hero.facing = Facing::Left;
        hero.walking = true;
    }

    if keyboard_input.just_released(KeyCode::A) {
        hero.facing = Facing::Left;
        hero.walking = false;
    }

    // Right 
    if keyboard_input.just_pressed(KeyCode::D) {
        hero.facing = Facing::Right;
    }

    if keyboard_input.pressed(KeyCode::D) {
        hero.facing = Facing::Right;
        hero.walking = true;
    }

    if keyboard_input.just_released(KeyCode::D) {
        hero.facing = Facing::Right;
        hero.walking = false;
    }
}
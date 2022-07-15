use benimator::SpriteSheetAnimation;
use bevy::prelude::Handle;
use bevy_ase::asset::Animation;

pub static MOSQUITO_ASEPRITE_PATH: &str = "art/mob/mosquito/mosquito.aseprite";

pub struct MosquitoAssets {
    pub walk_cycle_anim: (Handle<Animation>, Handle<SpriteSheetAnimation>),
}
